# maku

maku is a **modern, extensible multi-backend compute engine** built in **Rust** for **AI**, **image processing**, **numerical computation**, and **Web-native high-performance execution**.

This project provides a custom **IR (Intermediate Representation)** and
a unified execution runtime for **CPU / SIMD / WASM / (future) WebGPU**,
aiming to be a "universal compute layer" that can execute the same computation graph in any environment.

## 🧠 Why This Project Is Needed

The current computational ecosystem is fragmented:

### ❌ AI Frameworks
Strong for matrix operations and neural networks, but weak in image processing, general numerical computation, and Web support.

### ❌ Image Processing Frameworks (OpenCV / Halide)
Strong for filter processing, but cannot express AI or generalized computation graphs.

### ❌ WebAssembly / WebGPU World
Fast, but **lacks IR / optimizer / graph runtime**.

### ❌ CPU / SIMD / GPU Are Independent with No Unified Abstraction
Separate implementations required for each environment.

## 🎯 Goals

**Building a Compute Layer that Unifies Web / Native / AI / GPU**

- IR that can unify AI and image processing
- Rust CPU backend
- SIMD backend
- WebAssembly backend
- Eventually WebGPU (wgpu + WGSL)
- Execute JSON graphs directly on the Web
- Eventually implement Optimizer, Lowering, Fusion, etc.

The ultimate goal: **Create a next-generation general-purpose compute runtime in Rust that runs everywhere.**


## 🧩 Architecture Overview

```
               +-------------------------+
               |   React UI (optional)   |
               +-------------+-----------+
                             |
                        JSON Graph
                             |
                 +-----------+----------+
                 |     WASM Wrapper     |
                 |   (wasm-bindgen)     |
                 +-----------+----------+
                             |
                             v
        +-----------------------------------------------+
        |              Rust Compute Core                |
        |-----------------------------------------------|
        |  - Graph IR                                   |
        |  - Tensor Abstraction                         |
        |  - Executor                                   |
        +-------------+---------------+-----------------+
                      |               |
                      |               |
        +-------------+----+     +----+----------------+
        |   CPU Backend     |     | SIMD Backend       |
        |  (Rust loops)     |     | (std::simd)        |
        +-------------------+     +--------------------+
                             Future:
                        +----------------+
                        | WebGPU Backend |
                        |   (wgpu/WGSL)  |
                        +----------------+
```

## 🚀 Usage Example (JavaScript + WASM)

### Load Engine

```js
import init, { WasmEngine } from "maku";

await init();
const engine = new WasmEngine();
```

### Define Graph

```js
const graph = {
  nodes: [
    { id: "mm1", op: "MatMul", inputs: ["x", "w"], output: "z" },
    { id: "relu1", op: "Relu", inputs: ["z"], output: "y" }
  ],
  outputs: ["y"]
};
```

### Input Tensors

```js
const inputs = {
  x: { shape: [2,3], data: [1,2,3,4,5,6] },
  w: { shape: [3,1], data: [1,0.5,-1] }
};
```

### Execute

```js
const result = engine.run(graph, inputs);
console.log(result);
```

## 🗺 Phase-wise TODO

### Phase 0 — Foundation
- [ ] Tensor Abstraction
- [ ] Graph IR
- [ ] Basic Ops
- [ ] CPU Runtime
- [ ] WASM Wrapper
- [ ] JavaScript API

### Phase 1 — Stabilization & Optimization
- [ ] Shape Inference
- [ ] Static Type Checking
- [ ] Error Diagnostics Improvement
- [ ] SIMD Backend (Rust `std::simd`)
- [ ] Memory Pool / Buffer Reuse
- [ ] Benchmarks

### Phase 2 — Operation Extension
- [ ] Conv2d
- [ ] Reduce (sum, max)
- [ ] Broadcast
- [ ] Elementwise Ops
- [ ] Activation Ops

### Phase 3 — GPU Backend (WebGPU / wgpu)
- [ ] WGSL Kernel Generation
- [ ] WebGPU Pipeline
- [ ] Operation Fusion
- [ ] Auto Tiling

### Phase 4 — IR Optimization & Lowering
- [ ] Graph IR → Loop IR
- [ ] Constant Folding
- [ ] Layout Transform
- [ ] Fusion Pass

### Phase 5 — Developer Tools
- [ ] CLI Debugger
- [ ] Graph Visualization
- [ ] React UI (optional)
- [ ] Web Playground  

## 🏁 Ultimate Goal

### **Realizing a "High-Performance Universal Compute Engine That Runs Everywhere"**

- Support for all: Web / WASM / Native / GPU
- Unify AI / Image Processing / General Computation with a single IR
- Rust's safety and speed
- Web's accessibility
- Next-generation IR design aligned with MLIR

Ultimately,
**An ambitious project aiming for a position similar to TensorFlow/XLA or Halide for the Web.**

## 🤝 Author

Created by **Yusuke Shibata**
Built for the future of Web-native compute.

