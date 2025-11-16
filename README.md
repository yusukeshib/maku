# maku

maku は **Rust** 製の、**AI**, **画像処理**, **数値計算**, そして **Web ネイティブな高速実行**のための**モダンで拡張可能なマルチバックエンド計算エンジン**です。

本プロジェクトは、独自の **IR（中間表現）** と  
**CPU / SIMD / WASM /（将来）WebGPU** 向けの共通実行ランタイムを提供し、  
あらゆる環境で同じ計算グラフを実行できる “ユニバーサル計算層” を目指しています。

## 🧠 なぜこのプロジェクトが必要なのか

現在の計算エコシステムは分断されています：

### ❌ AI フレームワーク
行列演算やニューラルネットには強いが、画像処理・汎用数値処理・Web対応に弱い。

### ❌ 画像処理フレームワーク（OpenCV / Halide）
フィルタ処理は強いが、AI や一般化された計算グラフを表現できない。

### ❌ WebAssembly / WebGPU の世界
高速だが、**IR / オプティマイザ / グラフランタイムが存在しない**。

### ❌ CPU / SIMD / GPU がそれぞれ独立し統一抽象が無い  
各環境ごとに別実装が必要。

## 🎯 目的

**「Web / Native / AI / GPU」を統合する計算レイヤの構築**

- AI と画像処理を統一して扱える IR  
- Rust CPU バックエンド  
- SIMD バックエンド  
- WebAssembly バックエンド  
- 将来的には WebGPU（wgpu + WGSL）  
- Web では JSON の Graph をそのまま投げて実行  
- 最終的には Optimizer、Lowering、Fusion なども実装

最終到達点は： **どこでも動く、次世代の汎用計算ランタイムを Rust で作ること。**


## 🧩 アーキテクチャ概要

```
               +-------------------------+
               |   React UI (任意)       |
               +-------------+-----------+
                             |
                        JSON Graph
                             |
                 +-----------+----------+
                 |     WASM ラッパ      |
                 |   (wasm-bindgen)     |
                 +-----------+----------+
                             |
                             v
        +-----------------------------------------------+
        |              Rust Compute Core                |
        |-----------------------------------------------|
        |  - Graph IR                                   |
        |  - Tensor 抽象                               |
        |  - Executor                                   |
        +-------------+---------------+-----------------+
                      |               |
                      |               |
        +-------------+----+     +----+----------------+
        |   CPU Backend     |     | SIMD Backend       |
        |  (Rust loops)     |     | (std::simd)        |
        +-------------------+     +--------------------+
                             将来:
                        +----------------+
                        | WebGPU Backend |
                        |   (wgpu/WGSL)  |
                        +----------------+
```

## 🚀 使用例（JavaScript + WASM）

### エンジンをロード

```js
import init, { WasmEngine } from "my_engine_wasm";

await init();
const engine = new WasmEngine();
```

### グラフ定義

```js
const graph = {
  nodes: [
    { id: "mm1", op: "MatMul", inputs: ["x", "w"], output: "z" },
    { id: "relu1", op: "Relu", inputs: ["z"], output: "y" }
  ],
  outputs: ["y"]
};
```

### 入力 Tensor

```js
const inputs = {
  x: { shape: [2,3], data: [1,2,3,4,5,6] },
  w: { shape: [3,1], data: [1,0.5,-1] }
};
```

### 実行

```js
const result = engine.run(graph, inputs);
console.log(result);
```

## 🗺 フェーズ別 TODO

### Phase 0 — 基盤構築
- [ ] Tensor 抽象  
- [ ] Graph IR  
- [ ] 基本的な Ops  
- [ ] CPU Runtime  
- [ ] WASM Wrapper  
- [ ] JavaScript API  

### Phase 1 — 安定化・高速化
- [ ] Shape 推論  
- [ ] 静的型チェック  
- [ ] エラー診断改善  
- [ ] SIMD backend（Rust `std::simd`）  
- [ ] Memory Pool / Buffer Reuse  
- [ ] ベンチマーク  

### Phase 2 — 演算拡張
- [ ] Conv2d  
- [ ] Reduce（sum, max）  
- [ ] Broadcast  
- [ ] Elementwise Ops  
- [ ] Activation Ops  

### Phase 3 — GPU Backend（WebGPU / wgpu）
- [ ] WGSL カーネル生成  
- [ ] WebGPU パイプライン  
- [ ] 演算融合（Fusion）  
- [ ] 自動タイル化  

### Phase 4 — IR 最適化と Lowering
- [ ] Graph IR → Loop IR  
- [ ] Constant Folding  
- [ ] Layout Transform  
- [ ] Fusion Pass  

### Phase 5 — 開発者ツール
- [ ] CLI デバッガ  
- [ ] Graph 可視化  
- [ ] React UI（任意）  
- [ ] Web Playground  

## 🏁 最終ゴール

### **「どこでも動く、高性能なユニバーサル計算エンジン」の実現**

- Web / WASM / Native / GPU すべてに対応  
- AI / 画像処理 / 汎用計算を一つの IR で統一  
- Rust の安全性と速度  
- Web のアクセス性  
- MLIR に通じる次世代 IR 設計  

最終的には、  
**Web の TensorFlow/XLA、または Halide のような立ち位置を狙う**  
野心的プロジェクトです。

## 🤝 Author

Created by **Yusuke Shibata**  
Built for the future of Web-native compute.

