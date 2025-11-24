# Wasm + Rust による Conv2D / Add 実装まとめ（maku向け）

このドキュメントは **Rust で実装したカーネルを WebAssembly (Wasm) にコンパイルして動かす**前提で、  
**Conv2D（通常畳み込み）** と **Add（要素加算）** の実装方法を、maku の設計方針に沿って整理したものです。

- 目的：**まず正しく動く（correctness）実装を作り、後から高速化する**
- レイアウト：Web/Wasm 向きの **NHWC**
- データ型：**f32**

---

## 1. 前提：NHWC レイアウト

### テンソル形状

- 入力 `x` : `[N, H, W, C_in]`
- 出力 `y` : `[N, H_out, W_out, C_out]`
- カーネル `w` : `[K_h, K_w, C_in, C_out]`（OHWI）

### フラット配列の index

NHWC のとき、4次元テンソルは 1次元配列に次の式でマップできます：

```rust
#[inline]
fn idx_nhwc(n: usize, h: usize, w: usize, c: usize,
            h_dim: usize, w_dim: usize, c_dim: usize) -> usize {
    ((n * h_dim + h) * w_dim + w) * c_dim + c
}
```

カーネル（OHWI）の index は：

```rust
#[inline]
fn idx_kernel(kh: usize, kw: usize, ic: usize, oc: usize,
              k_h: usize, k_w: usize, in_c: usize, out_c: usize) -> usize {
    (((kh * k_w + kw) * in_c) + ic) * out_c + oc
}
```

---

## 2. Conv2D（NHWC）のナイーブ実装

### 2.1 必要なパラメータ

```rust
pub struct Conv2dParams {
    pub batch: usize,
    pub in_h: usize,
    pub in_w: usize,
    pub in_c: usize,
    pub out_h: usize,
    pub out_w: usize,
    pub out_c: usize,
    pub k_h: usize,
    pub k_w: usize,
    pub stride_h: usize,
    pub stride_w: usize,
    pub pad_top: isize,
    pub pad_left: isize,
}
```

`out_h, out_w` は **shape inference** で事前に計算しておきます。

---

### 2.2 実装（zero padding / stride 対応）

```rust
pub fn conv2d_nhwc(
    input: &[f32],      // [N, H, W, C_in]
    kernel: &[f32],     // [K_h, K_w, C_in, C_out]
    bias: Option<&[f32]>, // [C_out]
    output: &mut [f32],   // [N, H_out, W_out, C_out]
    p: &Conv2dParams,
) {
    let Conv2dParams {
        batch, in_h, in_w, in_c,
        out_h, out_w, out_c,
        k_h, k_w,
        stride_h, stride_w,
        pad_top, pad_left,
    } = *p;

    // 出力バッファを初期化
    output.fill(0.0);

    for n in 0..batch {
        for oh in 0..out_h {
            for ow in 0..out_w {
                // 出力位置に対応する入力の左上座標
                let ih0 = oh as isize * stride_h as isize - pad_top;
                let iw0 = ow as isize * stride_w as isize - pad_left;

                for oc in 0..out_c {
                    let mut acc = bias.map(|b| b[oc]).unwrap_or(0.0);

                    for kh in 0..k_h {
                        let ih = ih0 + kh as isize;
                        if ih < 0 || ih >= in_h as isize { continue; }

                        for kw in 0..k_w {
                            let iw = iw0 + kw as isize;
                            if iw < 0 || iw >= in_w as isize { continue; }

                            for ic in 0..in_c {
                                let in_idx = idx_nhwc(
                                    n, ih as usize, iw as usize, ic,
                                    in_h, in_w, in_c
                                );
                                let k_idx = idx_kernel(
                                    kh, kw, ic, oc,
                                    k_h, k_w, in_c, out_c
                                );
                                acc += input[in_idx] * kernel[k_idx];
                            }
                        }
                    }

                    let out_idx = idx_nhwc(n, oh, ow, oc, out_h, out_w, out_c);
                    output[out_idx] = acc;
                }
            }
        }
    }
}
```

#### ポイント

- **stride / padding を座標変換で処理**
- 入力範囲外は 0 とみなす（zero padding）
- まずはスカラで正しく動く形を作る

---

### 2.3 Wasm で使う（wasm-bindgen の最小例）

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn conv2d_run(
    input: &[f32],
    kernel: &[f32],
    bias: &[f32],
    params: Conv2dParams,
) -> Vec<f32> {
    let mut output = vec![0.0; params.batch * params.out_h * params.out_w * params.out_c];
    conv2d_nhwc(input, kernel, Some(bias), &mut output, &params);
    output
}
```

JS 側は `Float32Array` を渡して `Vec<f32>` を受け取る形になります。

---

### 2.4 高速化の方向（後からでOK）

- **im2col + GEMM** に切り替える  
  Conv を行列積へ変換し、GEMM だけ最適化すればよくなる。
- **Wasm SIMD（f32x4）** を hot loop に部分適用
- **tiling / cache-friendly ループ順** の調整
- ワークバッファを **再利用してアロケーションを削減**

---

## 3. Add（要素加算）の実装

### 3.1 仕様

Add は形状が一致するテンソル同士の **要素ごとの加算**：

```text
out[i] = a[i] + b[i]
```

レイアウト（NHWC / NCHW）は関係ありません。  
**フラット配列が同じ順で並んでいることだけが条件**です。

---

### 3.2 最小実装（超シンプル）

```rust
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), out.len());

    for i in 0..out.len() {
        out[i] = a[i] + b[i];
    }
}
```

---

### 3.3 Broadcast が必要な場合（例：bias add）

ONNX などでは broadcasting を許すため、必要なら拡張します。

例：`[N,H,W,C] + [C]`

```rust
pub fn add_bias_nhwc(input: &[f32], bias: &[f32], out: &mut [f32], c: usize) {
    for i in 0..out.len() {
        out[i] = input[i] + bias[i % c];
    }
}
```

---

### 3.4 Wasm SIMD での高速化（オプション）

```rust
use std::arch::wasm32::*;

pub fn add_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = out.len();
    let mut i = 0;

    while i + 4 <= len {
        unsafe {
            let va = v128_load(a.as_ptr().add(i) as *const _);
            let vb = v128_load(b.as_ptr().add(i) as *const _);
            let vc = f32x4_add(va, vb);
            v128_store(out.as_mut_ptr().add(i) as *mut _, vc);
        }
        i += 4;
    }

    while i < len {
        out[i] = a[i] + b[i];
        i += 1;
    }
}
```

---

## 4. Conv2D と Add の難易度比較

| op     | 実装難易度 | 主な理由                                   |
| ------ | ---------: | ------------------------------------------ |
| Add    |      ★☆☆☆☆ | 線形走査で足すだけ。layout依存なし。       |
| Conv2D |      ★★★★★ | stride/padding/shape/layout/最適化が絡む。 |

---

## 5. maku での実装優先度

MobileNetV2/V3 互換を狙う場合の優先順：

1. **Conv2D (NHWC)**
2. **DepthwiseConv2D**
3. **MatMul / FC**
4. **Add / Mul / Activations**
5. Pooling / Softmax / Reshape / Transpose

---

## Appendix: ありがちな落とし穴

- **layout mismatch**：ONNX が NCHW の場合、IR 正規化で Transpose を挿入
- **padding の扱い差**：same/valid を明示的に数式化して実装
- **float の誤差**：CPU 参照と完全一致を求めすぎない（許容誤差で比較）
- **Wasm の heap アロケーション**：ホットループ中の Vec 生成を避ける

---
