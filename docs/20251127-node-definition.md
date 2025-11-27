# MobileNetV2 対応ノード定義まとめ

**日付**: 2025-11-27
**目的**: MobileNetV2/V3 を実行するために必要なノードタイプとattributeの定義

---

## 概要

MobileNetV2/V3 の推論に必要な演算を maku のDSLに追加しました。
以下の3つのレイヤーで一貫した定義を行っています:

1. **TypeScript DSL** (`apps/editor/src/maku/types.ts`)
2. **Rust WASM Wrapper** (`lib/maku_web/src/lib.rs`)
3. **Rust Core** (`lib/maku/src/lib.rs`)

---

## サポートするノードタイプ一覧

### 基本演算（既存）

1. **Input** - 外部から提供される入力値
2. **Constant** - グラフに埋め込まれた定数テンソル
3. **Add** - 要素ごとの加算
4. **MatMul** - 行列積（2D × 2D）
5. **Relu** - 要素ごとのReLU活性化

### MobileNetV2 対応で追加したノード

#### 演算ノード

6. **Mul** - 要素ごとの乗算
7. **Relu6** - ReLU6活性化（0〜6の範囲にクリップ）
8. **HardSwish** - HardSwish活性化（MobileNetV3で使用）

#### 畳み込み演算

9. **Conv2D** - 2D畳み込み
10. **DepthwiseConv2D** - 深さ方向の2D畳み込み

#### 正規化

11. **BatchNorm** - バッチ正規化（推論時）

#### プーリング

12. **AveragePool** - 平均プーリング
13. **GlobalAveragePool** - グローバル平均プーリング

#### テンソル操作

14. **Reshape** - テンソルの形状変更
15. **Transpose** - テンソルの軸入れ替え
16. **Concat** - テンソルの連結

---

## Attributeの詳細定義

### Conv2D Attributes

```typescript
{
  kernel_shape: [number, number];     // [kh, kw] カーネルサイズ
  strides?: [number, number];         // [sh, sw] ストライド (default: [1, 1])
  pads?: [number, number, number, number]; // [top, left, bottom, right] (default: [0,0,0,0])
  dilations?: [number, number];       // [dh, dw] ダイレーション (default: [1, 1])
  group?: number;                     // グループ数 (default: 1)
}
```

**入力**:

- input: `[N, H, W, C_in]` (NHWC layout)
- kernel: `[Kh, Kw, C_in, C_out]` (OHWI layout)
- bias (optional): `[C_out]`

**出力**: `[N, H_out, W_out, C_out]`

**出力サイズ計算**:

```
H_out = floor((H + pad_top + pad_bottom - dilation_h * (kernel_h - 1) - 1) / stride_h + 1)
W_out = floor((W + pad_left + pad_right - dilation_w * (kernel_w - 1) - 1) / stride_w + 1)
```

---

### DepthwiseConv2D Attributes

```typescript
{
  kernel_shape: [number, number];     // [kh, kw]
  strides?: [number, number];         // [sh, sw] (default: [1, 1])
  pads?: [number, number, number, number]; // [top, left, bottom, right] (default: [0,0,0,0])
  dilations?: [number, number];       // [dh, dw] (default: [1, 1])
  depth_multiplier?: number;          // (default: 1)
}
```

**入力**:

- input: `[N, H, W, C]`
- kernel: `[Kh, Kw, C, depth_multiplier]`
- bias (optional): `[C * depth_multiplier]`

**出力**: `[N, H_out, W_out, C * depth_multiplier]`

---

### BatchNorm Attributes

```typescript
{
  epsilon?: number;   // 数値安定性のための小さな値 (default: 1e-5)
  momentum?: number;  // (default: 0.9)
}
```

**入力**:

- input: `[N, H, W, C]`
- scale (gamma): `[C]`
- bias (beta): `[C]`
- mean: `[C]`
- variance: `[C]`

**出力**: `[N, H, W, C]`

**計算式**:

```
output = scale * (input - mean) / sqrt(variance + epsilon) + bias
```

**NOTE**: 推論時は、Conv2Dに融合可能（weight folding）

---

### AveragePool Attributes

```typescript
{
  kernel_shape: [number, number];     // [kh, kw]
  strides?: [number, number];         // [sh, sw] (default: [1, 1])
  pads?: [number, number, number, number]; // (default: [0,0,0,0])
  count_include_pad?: boolean;        // (default: false)
}
```

**入力**: `[N, H, W, C]`
**出力**: `[N, H_out, W_out, C]`

---

### GlobalAveragePool

**Attributes**: なし

**入力**: `[N, H, W, C]`
**出力**: `[N, 1, 1, C]` または `[N, C]`（実装による）

各チャネルごとに全空間次元の平均を取る。

---

### MatMul Attributes

```typescript
{
  transA?: boolean;  // 第一引数を転置するか (default: false)
  transB?: boolean;  // 第二引数を転置するか (default: false)
}
```

**入力**: 2つの2Dテンソル
**出力**: 行列積の結果

---

### Reshape Attributes

```typescript
{
  shape: number[];     // ターゲット形状 (-1で自動推論可能)
  allowzero?: boolean; // (default: false)
}
```

**例**:

- Input: `[2, 3, 4]` (24要素)
- Shape: `[2, -1]` → Output: `[2, 12]`

---

### Transpose Attributes

```typescript
{
  perm: number[];  // 軸の並べ替え順序
}
```

**例**:

- Input: `[N, H, W, C]` (NHWC)
- Perm: `[0, 3, 1, 2]` → Output: `[N, C, H, W]` (NCHW)

---

### Concat Attributes

```typescript
{
  axis: number; // 連結する軸
}
```

**例**:

- Input1: `[2, 3, 4]`, Input2: `[2, 3, 5]`
- Axis: 2 → Output: `[2, 3, 9]`

---

## TypeScript型定義

### Discriminated Union による型安全なOp定義

```typescript
export type JsOp =
  | { op: "Input" }
  | { op: "Constant"; tensor: JsTensor }
  | { op: "Add" }
  | { op: "Mul" }
  | { op: "MatMul"; attrs?: MatMulAttrs }
  | { op: "Relu" }
  | { op: "Relu6" }
  | { op: "HardSwish" }
  | { op: "Conv2D"; attrs: Conv2DAttrs }
  | { op: "DepthwiseConv2D"; attrs: DepthwiseConv2DAttrs }
  | { op: "BatchNorm"; attrs?: BatchNormAttrs }
  | { op: "AveragePool"; attrs: AveragePoolAttrs }
  | { op: "GlobalAveragePool" }
  | { op: "Reshape"; attrs: ReshapeAttrs }
  | { op: "Transpose"; attrs: TransposeAttrs }
  | { op: "Concat"; attrs: ConcatAttrs };

export type JsNode = {
  id: string;
  inputs: string[];
  output: string;
} & JsOp;
```

---

## Rust Core OpKind定義

```rust
pub enum OpKind {
    Input,
    Constant(Tensor),
    Add,
    Mul,
    MatMul(Option<MatMulAttrs>),
    Relu,
    Relu6,
    HardSwish,
    Conv2D(Conv2DAttrs),
    DepthwiseConv2D(DepthwiseConv2DAttrs),
    BatchNorm(Option<BatchNormAttrs>),
    AveragePool(AveragePoolAttrs),
    GlobalAveragePool,
    Reshape(ReshapeAttrs),
    Transpose(TransposeAttrs),
    Concat(ConcatAttrs),
}
```

---

## 実装状況

### ✅ 完了

- [x] TypeScript型定義の追加
- [x] Rust WASM Wrapper の `JsOpKind` 定義
- [x] Rust Core の `OpKind` 定義
- [x] JsOpKind → OpKind のマッピング実装
- [x] CpuBackend の run 関数への dispatch 追加

### ⏳ 未実装（カーネル実装）

- [ ] Conv2D カーネル実装
- [ ] DepthwiseConv2D カーネル実装
- [ ] BatchNorm カーネル実装
- [ ] AveragePool カーネル実装
- [ ] GlobalAveragePool カーネル実装
- [ ] Reshape カーネル実装
- [ ] Transpose カーネル実装
- [ ] Concat カーネル実装

### ✅ 実装済み（簡易版）

- [x] Mul カーネル実装
- [x] Relu6 カーネル実装
- [x] HardSwish カーネル実装

---

## 次のステップ

### Phase 1: 基本カーネル実装

1. **Reshape** - 最も単純（データコピーのみ）
2. **Transpose** - インデックス計算
3. **Concat** - 軸方向のデータ連結
4. **GlobalAveragePool** - リダクション演算

### Phase 2: プーリング実装

5. **AveragePool** - ウィンドウ平均

### Phase 3: 畳み込み実装（最重要）

6. **Conv2D** (naive 実装)
7. **DepthwiseConv2D**

### Phase 4: 正規化

8. **BatchNorm** - 推論時の正規化

### Phase 5: 最適化

- Conv2D → im2col + GEMM
- WASM SIMD 対応（f32x4）
- メモリ再利用

---

## 参考資料

- [docs/20251122-mobilenet-wasm-milestone.md](./20251122-mobilenet-wasm-milestone.md) - MobileNetV2対応のマイルストーン
- [docs/20251122-wasm-rust-conv2d-add-notes.md](./20251122-wasm-rust-conv2d-add-notes.md) - Conv2D実装ガイド
- [docs/20251116-architecture-dsl-vs-ir.md](./20251116-architecture-dsl-vs-ir.md) - DSL vs IR アーキテクチャ

---

**Author**: Claude Code
**Date**: 2025-11-27
