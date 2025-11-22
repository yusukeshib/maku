
# Milestone 1: MobileNetV2 / V3 互換の WASM Runtime

maku の最初の大きなマイルストーンとして **「MobileNetV2 / MobileNetV3 に互換性を持つ WASM Runtime」** を狙う理由と、それを実現するための実装ロードマップをまとめます。

---

## なぜ MobileNetV2 / V3 × WASM を最初のマイルストーンにするのか

### 1. もっとも“使われている軽量モデル”であり入口になりやすい
MobileNet 系は、世界中で最も広く使われている軽量 CNN の代表格です。

- 画像分類の定番モデル
- モバイル／Web／IoT など軽量推論の基礎として採用されやすい
- 学習・デモ・実運用の幅が広い

「MobileNet が動く」だけで、利用者が **すぐに試せてすぐ役立つ** 状態を作れます。

---

### 2. 必要な演算（op）が少なく、互換実装のコストが読みやすい
MobileNetV2/V3 が要求する演算は比較的シンプルなサブセットで完結します。

主に必要な op:

- Conv2D
- DepthwiseConv2D
- BatchNorm（推論時は Conv へ fuse 可能）
- Add（Residual）
- Mul
- ReLU6 / HardSwish
- AveragePool / GlobalAveragePool
- FullyConnected（MatMul + bias）
- Reshape / Transpose / Concat（最小限）

**10〜12種類程度の op 実装で高い互換率が出せる**ので、  
「最初の互換対象」として最も現実的です。

---

### 3. Web エコシステムと噛み合う（NHWC・WASM・WebGPU）
MobileNet は **NHWC レイアウト前提の実装が自然**で、

- ブラウザの画像 I/O（Canvas, video）のデータ並びと一致
- WASM のシンプルな linear memory に乗せやすい
- DepthwiseConv が NHWC で高速
- 後から WebGPU backend を追加するときもスムーズ

つまり **maku が目指す Web/軽量推論の強みと真正面から一致**します。

---

### 4. “動くデモ”が作りやすく、ユーザー獲得が最短
WASM runtime で MobileNet を動かせると:

- 画像分類デモ（ドラッグ&ドロップ／カメラ入力）
- 速度検証（CPU vs WASM）
- IR エディタからの実行デモ

がすぐに用意できます。  
OSS の伸びは **最初のデモ体験で決まる**ので、ここに一番近い道です。

---

### 5. Runtime と IR の設計が固まり、次の拡張が楽になる
MobileNet を通すと:

- IR の opset
- shape inference
- layout 戦略（NHWC）
- execution plan
- memory allocator
- kernel 設計（Conv / DWConv / MatMul）

など「ランタイムの骨格」が一通り固まります。

その後の

- ResNet / EfficientNet / TinyBERT
- WebGPU backend 追加
- SIMD 最適化
- ONNX opset 拡大

が **同じ土台の上で増築できる**ようになります。

---

## ロードマップ（MobileNetV2/V3 互換 WASM Runtime）

以下は **“動くものを最短で出す”** ことを重視した順番です。  
（期間は目安。並行して進められる部分もあります）

---

### Phase 0: ゴールの明確化（1–2日）
- MobileNetV2/V3 の ONNX を固定ターゲットとして決める
- 入力／出力の仕様を確定（例: 224×224 RGB, float32, NHWC）
- サポートする opset の範囲を宣言（README に ALPHA として明記）

成果物:
- `docs/milestone1-goals.md`（対象モデルと範囲を明記）

---

### Phase 1: IR 最小セットの確定（〜1週）
- MobileNet が必要とする op を IR として定義
- ノードの属性（stride, padding, groups, activation etc）を整理
- IR の内部レイアウトを **NHWC に統一**する方針を決定

成果物:
- `crates/ir` の op 定義
- `docs/ir-opset-mobilenet.md`

---

### Phase 2: Shape inference & IR 正規化パイプライン（1–2週）
- 各 op の出力 shape 推論を実装
- NCHW 入力モデルへの対応として  
  **必要な箇所に Transpose を自動挿入**
- 推論向け最適化:
  - BatchNorm folding（Conv に融合）
  - 定数畳み込み（constant folding）
  - 単純な op fusion（Conv+Activation など）

成果物:
- `crates/optimizer`（または lowering の一部）
- `tests/shape_inference/*.rs`

---

### Phase 3: WASM Runtime の骨格（1–2週）
- Linear memory allocator を実装
  - Tensor = (offset, shape, dtype)
  - ワークバッファを事前確保して再利用
- Execution plan を生成し、dispatcher で順次実行できるようにする

成果物:
- `crates/runtime-wasm`
- `ExecutionPlan` 構造体 + 実行ループ

---

### Phase 4: コアカーネル実装（2–4週）
優先順位は MobileNet の計算コスト順に。

1. **Conv2D（NHWC）**
   - まず naive 実装で correctness を確保
   - 後から im2col / tiling を検討
2. **DepthwiseConv2D**
3. **MatMul / FullyConnected**
4. **Add / Mul / Activations（ReLU6, HardSwish）**
5. **Pooling / Softmax**
6. **Reshape / Transpose / Concat**

成果物:
- `crates/kernels-wasm`
- `tests/kernels/*.rs`（CPU reference と突き合わせ）

---

### Phase 5: ONNX → IR Converter（1–2週）
- ONNX parser で graph/weights を取得
- opset mapping（Conv, BN, DWConv, Add, Gemm etc）
- weights の layout 変換（必要な場合のみ）
- IR 正規化 → lowering → execution plan まで通す

成果物:
- `crates/onnx-import`
- `examples/import_mobilenet.rs`

---

### Phase 6: 互換検証 & 最初のデモ（1–2週）
- MobileNetV2/V3 の推論結果を以下で一致させる
  - PyTorch / ONNXRuntime 参照
  - maku WASM runtime
- Top-1 精度 & 出力誤差を評価
- Web デモ（最小）を公開
  - 画像を読み込み → 推論 → top-k 表示

成果物:
- `apps/demo-web`（または `examples/web/`）
- `docs/mobilenet-compat-report.md`

---

### Phase 7: 最低限の性能改善（オプション / 1–2週）
“使える速さ”に達していれば後回しでも良い。

- Conv / DWConv のホットスポットを profiling
- WASM SIMD（f32x4）を  
  Conv・MatMul に **部分的に**適用
- メモリ再割当の削減 / バッファ再利用

成果物:
- `bench/wasm_conv_bench.md`
- SIMD 版 kernel（feature flag）

---

## マイルストーン達成条件（Definition of Done）

- [ ] MobileNetV2 の ONNX を **そのまま読み込み** 推論できる
- [ ] MobileNetV3 も同様に推論できる（Small / Large どちらか片方でも可）
- [ ] 参照実装（ONNXRuntime / PyTorch）と  
      **出力誤差が許容範囲**に収まる
- [ ] Web デモで実際に動かせる
- [ ] README / docs に “対応範囲と既知の制限” を明記

---

## 次の拡張に向けて
このマイルストーンが終わると、次の道が開きます:

1. ResNet / EfficientNet-Lite の互換拡大  
2. WebGPU backend 追加（同じ IR / plan を再利用）  
3. より広い ONNX opset 対応  
4. 高度な fusion / memory planning  
5. 本格 SIMD / multi-threading

MobileNet は **maku の“最小で最大の価値を証明する”入口**なので、  
ここをしっかり決め切るのが一番コスパが高い戦略です。

---
