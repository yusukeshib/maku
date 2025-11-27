# アーキテクチャ決定: DSL vs 内部IR

## 背景

React UI機能を備えたmaku計算エンジンを構築する際、基本的な問いが生じました:

**React UI DSLと内部IRは同じであるべきか、それとも別々であるべきか?**

## 現在の実装

コードベースはすでに**2層分離**を実装しています:

1. **内部IR (コア)**: `ValueId(u32)`, `NodeId(u32)` - `lib/maku/src/lib.rs`で定義
2. **JavaScript DSL**: 文字列ベースのID、JSON直列化可能 - `lib/wasm/src/lib.rs`で定義

```rust
// 内部IR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);
```

```rust
// JavaScript DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsNode {
    pub id: String,
    pub op: JsOpKind,
    pub inputs: Vec<String>,
    pub output: String,
}
```

## 決定: 分離を維持する ✅

### 根拠

プロジェクトの目標（MLIRにインスパイアされた設計、オプティマイザ/フュージョンの実装、ライブラリエコシステム）を考慮すると、**分離が正しいアプローチ**です。

### 分離の利点

#### 1. 最適化パイプラインを可能にする

```
React UI (高レベルDSL)
  ↓ デシリアライゼーション
Graph IR (内部IR)
  ↓ 最適化パス
最適化されたグラフ
  ↓ 低レベル化
Loop IR / Kernel IR
  ↓
CPU / SIMD / WebGPU
```

#### 2. JavaScriptの人間工学

- 文字列ID（`"layer1"`, `"relu_out"`）は人間が読みやすい
- そのままJSONシリアライズ可能
- React UIでの視覚的編集に最適
- デバッグと検査が容易

#### 3. 内部IRの自由度

- JS APIを安定させたまま、内部IRを積極的に進化させることができる
- 将来の変更例:
  - `ValueId`をSSA形式に変換
  - 型推論システムの追加
  - メモリプランニングの実装
  - バックエンド固有の最適化の追加

#### 4. 業界標準パターン

これは確立されたフレームワークの設計と一致しています:

- **TensorFlow**: Python API (高レベル) ⇄ GraphDef (低レベルIR)
- **PyTorch**: Python API ⇄ TorchScript
- **MLIR**: 段階的な低レベル化を伴う複数の抽象レベル

## ライブラリエコシステムの設計

### 提案する3層アーキテクチャ

```
┌─────────────────────────────────────────┐
│  高レベルDSL (React UI / パッケージ)     │
│  - 文字列ベースID                        │
│  - 人間が読めるJSON                      │
│  - 合成可能な関数/ブロック               │
└────────────────┬────────────────────────┘
                 │ シリアライズ/デシリアライズ
┌────────────────▼────────────────────────┐
│  パッケージフォーマット (交換層)         │
│  - 安定したバージョン管理スキーマ        │
│  - メタデータ (作者、バージョン、依存)   │
│  - 入出力シグネチャ                      │
└────────────────┬────────────────────────┘
                 │ インポート/コンパイル
┌────────────────▼────────────────────────┐
│  コアIR (最適化ターゲット)               │
│  - ValueId(u32)、SSA形式                 │
│  - 型チェック済み、形状推論済み          │
│  - バックエンド非依存                    │
└─────────────────────────────────────────┘
```

### パッケージフォーマットの例

```json
{
  "name": "resnet-block",
  "version": "1.0.0",
  "author": "yusuke",
  "description": "Reusable ResNet block",
  "inputs": {
    "x": { "shape": ["batch", 256, 56, 56], "dtype": "f32" }
  },
  "outputs": {
    "out": { "shape": ["batch", 256, 56, 56], "dtype": "f32" }
  },
  "graph": {
    "nodes": [
      {
        "id": "conv1",
        "op": "Conv2d",
        "inputs": ["x", "weight1"],
        "output": "conv1_out"
      },
      {
        "id": "bn1",
        "op": "BatchNorm",
        "inputs": ["conv1_out"],
        "output": "bn1_out"
      },
      {
        "id": "relu",
        "op": "Relu",
        "inputs": ["bn1_out"],
        "output": "relu_out"
      },
      { "id": "add", "op": "Add", "inputs": ["relu_out", "x"], "output": "out" }
    ]
  },
  "parameters": {
    "weight1": { "shape": [256, 256, 3, 3], "init": "he_normal" }
  }
}
```

### React UIでの使用方法

```jsx
import { usePackage } from "maku-packages";

function MyModel() {
  const resnetBlock = usePackage("resnet-block@1.0.0");

  return (
    <Graph>
      <Input name="image" />
      <Node use={resnetBlock} inputs={{ x: "image" }} output="features" />
      <Node op="Dense" inputs={["features"]} output="logits" />
    </Graph>
  );
}
```

### 内部処理

```rust
// パッケージマネージャー
pub struct PackageRegistry {
    packages: HashMap<String, Package>,
}

pub struct Package {
    pub name: String,
    pub version: String,
    pub graph_template: JsGraph,
    pub signature: Signature,
}

// インポート時にコアIRへコンパイル
impl PackageRegistry {
    pub fn instantiate(&self, name: &str, context: &mut GraphContext) -> SubGraph {
        let pkg = self.packages.get(name).unwrap();
        // JsGraph -> Core Graph への変換
        // コンテキスト名前空間への統合
    }
}
```

## 主要な設計原則

### 1. パッケージフォーマットの安定性

- **セマンティックバージョニング**: Major.Minor.Patch
- **厳格な破壊的変更管理**
- **後方互換性保証**
- **明確な移行パス** (バージョン更新時)

### 2. コアIR進化の自由度

- パッケージに影響を与えずに最適化パスを追加できる
- 新しいバックエンドを透過的にサポート
- 変換レイヤーが変更を分離
- 内部リファクタリングがエコシステムを壊さない

### 3. 型システムとシグネチャ

- **パラメトリック形状**: `["batch", "channels", H, W]`
- **型チェック** (合成互換性)
- **形状推論** (フェーズ1) が重要になる
- **制約伝播** (シンボリック次元)

## 既存エコシステムからのインスピレーション

| プロジェクト         | パッケージフォーマット | 主要機能                                   |
| -------------------- | ---------------------- | ------------------------------------------ |
| **ONNX**             | Protobuf               | AIモデル交換、オペレータレジストリ         |
| **npm**              | JSON                   | バージョン管理、依存関係解決               |
| **Hugging Face Hub** | Gitベース              | モデル・データセット共有、コミュニティ     |
| **TVM PackedFunc**   | バイナリ               | プリコンパイル関数、クロスプラットフォーム |

## 現在の課題と改善策

### 問題: 文字列からIDへの変換コスト

現在の実装 `lib/wasm/src/lib.rs:47-60`:

```rust
fn str_to_value_id(s: &str) -> ValueId {
    if let Ok(n) = s.parse::<u32>() {
        ValueId(n)
    } else {
        // 粗いハッシュ (MVP のみ)
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        let v = (h.finish() & 0xFFFF_FFFF) as u32;
        ValueId(v)
    }
}
```

### 推奨される改善策

#### 1. 双方向マッピングの維持

```rust
pub struct GraphContext {
    name_to_id: HashMap<String, ValueId>,
    id_to_name: HashMap<ValueId, String>,
    next_id: u32,
}

impl GraphContext {
    pub fn register_value(&mut self, name: String) -> ValueId {
        if let Some(&id) = self.name_to_id.get(&name) {
            return id;
        }
        let id = ValueId(self.next_id);
        self.next_id += 1;
        self.name_to_id.insert(name.clone(), id);
        self.id_to_name.insert(id, name);
        id
    }

    pub fn get_name(&self, id: ValueId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }
}
```

#### 2. 形状推論

現在、`value_types`は部分的にのみ埋められています（`lib/maku/src/lib.rs:88`）。必要なこと:

- すべての操作に対して形状推論を実装
- グラフ全体に型情報を伝播
- コンパイル時に形状の不一致をキャッチ

## 実装ロードマップ

### フェーズ 0 (現在) ✓

- [x] 基本的なTensor抽象化
- [x] グラフIR
- [x] CPUバックエンド
- [x] WASMラッパー
- [x] 基本的なJsGraph ⇄ コアグラフ変換

### フェーズ 1 (次の優先事項)

- [ ] 名前管理機能付きGraphContext
- [ ] 形状推論エンジン
- [ ] パッケージフォーマット定義 (JSON Schema)
- [ ] 静的型チェック
- [ ] 改善されたエラー診断

### フェーズ 2 (オペレータ拡張)

- [ ] パッケージレジストリ実装
- [ ] 依存関係解決
- [ ] バージョニングと互換性チェック
- [ ] Conv2d、Reduce、Broadcast操作
- [ ] 拡張活性化関数

### フェーズ 3 (エコシステム)

- [ ] パッケージ閲覧用Web UI
- [ ] パッケージアップロード/ダウンロード
- [ ] 検証とサンドボックス実行
- [ ] コミュニティパッケージリポジトリ

### フェーズ 4 (高度な機能)

- [ ] プリコンパイル成果物 (WASMバイナリ)
- [ ] 差分プライバシーサポート (信頼されていないパッケージ)
- [ ] 連合学習機能
- [ ] クロスプラットフォームバイナリ配布

### フェーズ 5 (GPU & 最適化)

- [ ] WebGPUバックエンド (wgpu + WGSL)
- [ ] カーネルフュージョン
- [ ] 自動チューニング
- [ ] メモリ最適化

## 結論

**DSLと内部IRの分離は以下のために不可欠です**:

✅ ユーザーフレンドリーで人間が読みやすい高レベルAPI
✅ 安定したバージョン管理されたパッケージ交換フォーマット
✅ 内部表現を最適化し進化させる自由度
✅ コミュニティパッケージを持つスケーラブルなエコシステム
✅ 業界標準のアーキテクチャパターン

この設計により、makuはそのビジョンを達成できます: **「Rustで構築された、どこでも動作するユニバーサル計算ランタイム。」**

## 参考文献

- MLIR (Multi-Level Intermediate Representation): https://mlir.llvm.org/
- ONNX: https://onnx.ai/
- TVM: https://tvm.apache.org/
- Halide: https://halide-lang.org/
- TensorFlow GraphDef: https://www.tensorflow.org/guide/intro_to_graphs

---

**著者**: Yusuke Shibata
**日付**: 2025-01-16
**ステータス**: 承認済み
