# Architecture Decision: DSL vs Internal IR

## Context

When building the maku compute engine with React UI capabilities, a fundamental question arose:

**Should the React UI DSL and the internal IR be the same, or should they be separate?**

## Current Implementation

The codebase already implements a **2-layer separation**:

1. **Internal IR (Core)**: `ValueId(u32)`, `NodeId(u32)` - defined in `lib/maku/src/lib.rs`
2. **JavaScript DSL**: String-based IDs, JSON-serializable - defined in `lib/wasm/src/lib.rs`

```rust
// Internal IR
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

## Decision: Keep Them Separate ✅

### Rationale

Given the project's goals (MLIR-inspired design, optimizer/fusion implementation, and library ecosystem), **separation is the correct approach**.

### Benefits of Separation

#### 1. Enables Optimization Pipeline

```
React UI (High-level DSL)
  ↓ Deserialization
Graph IR (Internal IR)
  ↓ Optimization passes
Optimized Graph
  ↓ Lowering
Loop IR / Kernel IR
  ↓
CPU / SIMD / WebGPU
```

#### 2. JavaScript Ergonomics

- String IDs (`"layer1"`, `"relu_out"`) are human-readable
- JSON serializable out of the box
- Perfect for visual editing in React UI
- Easy debugging and inspection

#### 3. Internal IR Freedom

- Can evolve the internal IR aggressively while keeping JS API stable
- Examples of future changes:
  - Convert `ValueId` to SSA form
  - Add type inference system
  - Implement memory planning
  - Add backend-specific optimizations

#### 4. Industry-Standard Pattern

This matches the design of established frameworks:
- **TensorFlow**: Python API (high-level) ⇄ GraphDef (low-level IR)
- **PyTorch**: Python API ⇄ TorchScript
- **MLIR**: Multiple abstraction levels with progressive lowering

## Library Ecosystem Design

### Proposed 3-Layer Architecture

```
┌─────────────────────────────────────────┐
│  High-level DSL (React UI / Package)    │
│  - String-based IDs                      │
│  - Human-readable JSON                   │
│  - Composable functions/blocks           │
└────────────────┬────────────────────────┘
                 │ serialize/deserialize
┌────────────────▼────────────────────────┐
│  Package Format (Interchange Layer)     │
│  - Stable, versioned schema              │
│  - Metadata (author, version, deps)      │
│  - Input/output signatures               │
└────────────────┬────────────────────────┘
                 │ import/compile
┌────────────────▼────────────────────────┐
│  Core IR (Optimization Target)          │
│  - ValueId(u32), SSA form                │
│  - Type-checked, shape-inferred          │
│  - Backend-agnostic                      │
└─────────────────────────────────────────┘
```

### Package Format Example

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
      { "id": "conv1", "op": "Conv2d", "inputs": ["x", "weight1"], "output": "conv1_out" },
      { "id": "bn1", "op": "BatchNorm", "inputs": ["conv1_out"], "output": "bn1_out" },
      { "id": "relu", "op": "Relu", "inputs": ["bn1_out"], "output": "relu_out" },
      { "id": "add", "op": "Add", "inputs": ["relu_out", "x"], "output": "out" }
    ]
  },
  "parameters": {
    "weight1": { "shape": [256, 256, 3, 3], "init": "he_normal" }
  }
}
```

### Usage in React UI

```jsx
import { usePackage } from 'maku-packages';

function MyModel() {
  const resnetBlock = usePackage('resnet-block@1.0.0');

  return (
    <Graph>
      <Input name="image" />
      <Node use={resnetBlock} inputs={{x: "image"}} output="features" />
      <Node op="Dense" inputs={["features"]} output="logits" />
    </Graph>
  );
}
```

### Internal Processing

```rust
// Package Manager
pub struct PackageRegistry {
    packages: HashMap<String, Package>,
}

pub struct Package {
    pub name: String,
    pub version: String,
    pub graph_template: JsGraph,
    pub signature: Signature,
}

// Compile to Core IR on import
impl PackageRegistry {
    pub fn instantiate(&self, name: &str, context: &mut GraphContext) -> SubGraph {
        let pkg = self.packages.get(name).unwrap();
        // Convert JsGraph -> Core Graph
        // Integrate into context namespace
    }
}
```

## Key Design Principles

### 1. Package Format Stability

- **Semantic versioning**: Major.Minor.Patch
- **Strict breaking change management**
- **Backward compatibility guarantees**
- **Clear migration paths** for version updates

### 2. Core IR Evolution Freedom

- Can add optimization passes without affecting packages
- Support new backends transparently
- Conversion layer isolates changes
- Internal refactoring doesn't break ecosystem

### 3. Type System & Signatures

- **Parametric shapes**: `["batch", "channels", H, W]`
- **Type checking** for composition compatibility
- **Shape inference** (Phase 1) becomes critical
- **Constraint propagation** for symbolic dimensions

## Inspiration from Existing Ecosystems

| Project | Package Format | Key Features |
|---------|----------------|--------------|
| **ONNX** | Protobuf | AI model interchange, operator registry |
| **npm** | JSON | Version management, dependency resolution |
| **Hugging Face Hub** | Git-based | Model & dataset sharing, community-driven |
| **TVM PackedFunc** | Binary | Pre-compiled functions, cross-platform |

## Current Issues & Improvements

### Problem: String-to-ID Conversion Cost

Current implementation in `lib/wasm/src/lib.rs:47-60`:

```rust
fn str_to_value_id(s: &str) -> ValueId {
    if let Ok(n) = s.parse::<u32>() {
        ValueId(n)
    } else {
        // Crude hash (MVP only)
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        let v = (h.finish() & 0xFFFF_FFFF) as u32;
        ValueId(v)
    }
}
```

### Recommended Improvements

#### 1. Maintain Bidirectional Mapping

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

#### 2. Shape Inference

Currently `value_types` is only partially populated (`lib/maku/src/lib.rs:88`). Need to:
- Implement shape inference for all operations
- Propagate type information through the graph
- Catch shape mismatches at compile time

## Implementation Roadmap

### Phase 0 (Current) ✓
- [x] Basic Tensor abstraction
- [x] Graph IR
- [x] CPU Backend
- [x] WASM Wrapper
- [x] Basic JsGraph ⇄ Core Graph conversion

### Phase 1 (Next Priority)
- [ ] GraphContext with name management
- [ ] Shape inference engine
- [ ] Package Format definition (JSON Schema)
- [ ] Static type checking
- [ ] Improved error diagnostics

### Phase 2 (Operator Expansion)
- [ ] Package Registry implementation
- [ ] Dependency resolution
- [ ] Versioning & compatibility checks
- [ ] Conv2d, Reduce, Broadcast operations
- [ ] Extended activation functions

### Phase 3 (Ecosystem)
- [ ] Web UI for browsing packages
- [ ] Package upload/download
- [ ] Validation & sandboxed execution
- [ ] Community package repository

### Phase 4 (Advanced Features)
- [ ] Pre-compiled artifacts (WASM binaries)
- [ ] Differential privacy support (untrusted packages)
- [ ] Federated learning capabilities
- [ ] Cross-platform binary distribution

### Phase 5 (GPU & Optimization)
- [ ] WebGPU backend (wgpu + WGSL)
- [ ] Kernel fusion
- [ ] Auto-tuning
- [ ] Memory optimization

## Conclusion

**The separation between DSL and Internal IR is essential** for:

✅ User-friendly, human-readable high-level API
✅ Stable, versioned package interchange format
✅ Freedom to optimize and evolve internal representation
✅ Scalable ecosystem with community packages
✅ Industry-standard architecture pattern

This design enables maku to achieve its vision: **"A universal compute runtime that works everywhere, built in Rust."**

## References

- MLIR (Multi-Level Intermediate Representation): https://mlir.llvm.org/
- ONNX: https://onnx.ai/
- TVM: https://tvm.apache.org/
- Halide: https://halide-lang.org/
- TensorFlow GraphDef: https://www.tensorflow.org/guide/intro_to_graphs

---

**Author**: Yusuke Shibata
**Date**: 2025-01-16
**Status**: Accepted
