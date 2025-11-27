use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use maku::{CpuBackend, DType, Graph, Node, NodeId, OpKind, Tensor, TensorDesc, ValueId};

/// ---------- Types for communication with JS ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsTensor {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
}

// Attribute structures for each operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conv2DAttrs {
    pub kernel_shape: [usize; 2],
    #[serde(default = "default_strides")]
    pub strides: [usize; 2],
    #[serde(default = "default_pads")]
    pub pads: [usize; 4],
    #[serde(default = "default_dilations")]
    pub dilations: [usize; 2],
    #[serde(default = "default_group")]
    pub group: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthwiseConv2DAttrs {
    pub kernel_shape: [usize; 2],
    #[serde(default = "default_strides")]
    pub strides: [usize; 2],
    #[serde(default = "default_pads")]
    pub pads: [usize; 4],
    #[serde(default = "default_dilations")]
    pub dilations: [usize; 2],
    #[serde(default = "default_depth_multiplier")]
    pub depth_multiplier: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchNormAttrs {
    #[serde(default = "default_epsilon")]
    pub epsilon: f32,
    #[serde(default = "default_momentum")]
    pub momentum: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AveragePoolAttrs {
    pub kernel_shape: [usize; 2],
    #[serde(default = "default_strides")]
    pub strides: [usize; 2],
    #[serde(default = "default_pads")]
    pub pads: [usize; 4],
    #[serde(default)]
    pub count_include_pad: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatMulAttrs {
    #[serde(default)]
    pub trans_a: bool,
    #[serde(default)]
    pub trans_b: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReshapeAttrs {
    pub shape: Vec<isize>, // can include -1 for auto-inference
    #[serde(default)]
    pub allowzero: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransposeAttrs {
    pub perm: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcatAttrs {
    pub axis: usize,
}

// Default value functions
fn default_strides() -> [usize; 2] { [1, 1] }
fn default_pads() -> [usize; 4] { [0, 0, 0, 0] }
fn default_dilations() -> [usize; 2] { [1, 1] }
fn default_group() -> usize { 1 }
fn default_depth_multiplier() -> usize { 1 }
fn default_epsilon() -> f32 { 1e-5 }
fn default_momentum() -> f32 { 0.9 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum JsOpKind {
    Input,
    Constant { tensor: JsTensor },
    Add,
    Mul,
    MatMul {
        #[serde(default)]
        attrs: Option<MatMulAttrs>
    },
    Relu,
    Relu6,
    HardSwish,
    Conv2D { attrs: Conv2DAttrs },
    DepthwiseConv2D { attrs: DepthwiseConv2DAttrs },
    BatchNorm {
        #[serde(default)]
        attrs: Option<BatchNormAttrs>
    },
    AveragePool { attrs: AveragePoolAttrs },
    GlobalAveragePool,
    Reshape { attrs: ReshapeAttrs },
    Transpose { attrs: TransposeAttrs },
    Concat { attrs: ConcatAttrs },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsNode {
    pub id: String,
    #[serde(flatten)]
    pub op: JsOpKind,
    pub inputs: Vec<String>,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsGraph {
    pub nodes: Vec<JsNode>,
    pub outputs: Vec<String>,
}

/// Input tensor: valueId(string) -> JsTensor
type JsInputs = HashMap<String, JsTensor>;

/// Output tensor: valueId(string) -> JsTensor
type JsOutputs = HashMap<String, JsTensor>;

/// ---------- Helper for ID conversion ----------

fn str_to_value_id(s: &str) -> ValueId {
    // MVP: Can use hash-like approach, or simply parse to u32
    // For now, try simple u32 parse, if it fails, use hash, etc.
    if let Ok(n) = s.parse::<u32>() {
        ValueId(n)
    } else {
        // Very crude hash (because MVP)
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        let v = (h.finish() & 0xFFFF_FFFF) as u32;
        ValueId(v)
    }
}

fn str_to_node_id(s: &str) -> NodeId {
    if let Ok(n) = s.parse::<u32>() {
        NodeId(n)
    } else {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        let v = (h.finish() & 0xFFFF_FFFF) as u32;
        NodeId(v)
    }
}

fn js_tensor_to_core(t: &JsTensor) -> Tensor {
    Tensor::new(
        TensorDesc {
            dtype: DType::F32,
            shape: t.shape.clone(),
        },
        t.data.clone(),
    )
}

fn core_tensor_to_js(t: &Tensor) -> JsTensor {
    JsTensor {
        shape: t.desc.shape.clone(),
        data: t.data.clone(),
    }
}

/// ---------- JsGraph -> Graph conversion ----------

fn build_core_graph(js_graph: &JsGraph) -> Graph {
    let mut nodes = Vec::new();
    let mut value_types = HashMap::new();

    for js_node in &js_graph.nodes {
        let node_id = str_to_node_id(&js_node.id);
        let output_id = str_to_value_id(&js_node.output);
        let input_ids: Vec<ValueId> = js_node.inputs.iter().map(|s| str_to_value_id(s)).collect();

        let op = match &js_node.op {
            JsOpKind::Input => OpKind::Input,
            JsOpKind::Constant { tensor } => {
                let core_t = js_tensor_to_core(tensor);
                // Register Constant's output type since it's determined from tensor
                value_types.insert(output_id, core_t.desc.clone());
                OpKind::Constant(core_t)
            }
            JsOpKind::Add => OpKind::Add,
            JsOpKind::Mul => OpKind::Mul,
            JsOpKind::MatMul { attrs } => {
                OpKind::MatMul(attrs.as_ref().map(|a| maku::MatMulAttrs {
                    trans_a: a.trans_a,
                    trans_b: a.trans_b,
                }))
            }
            JsOpKind::Relu => OpKind::Relu,
            JsOpKind::Relu6 => OpKind::Relu6,
            JsOpKind::HardSwish => OpKind::HardSwish,
            JsOpKind::Conv2D { attrs } => {
                OpKind::Conv2D(maku::Conv2DAttrs {
                    kernel_shape: attrs.kernel_shape,
                    strides: attrs.strides,
                    pads: attrs.pads,
                    dilations: attrs.dilations,
                    group: attrs.group,
                })
            }
            JsOpKind::DepthwiseConv2D { attrs } => {
                OpKind::DepthwiseConv2D(maku::DepthwiseConv2DAttrs {
                    kernel_shape: attrs.kernel_shape,
                    strides: attrs.strides,
                    pads: attrs.pads,
                    dilations: attrs.dilations,
                    depth_multiplier: attrs.depth_multiplier,
                })
            }
            JsOpKind::BatchNorm { attrs } => {
                OpKind::BatchNorm(attrs.as_ref().map(|a| maku::BatchNormAttrs {
                    epsilon: a.epsilon,
                    momentum: a.momentum,
                }))
            }
            JsOpKind::AveragePool { attrs } => {
                OpKind::AveragePool(maku::AveragePoolAttrs {
                    kernel_shape: attrs.kernel_shape,
                    strides: attrs.strides,
                    pads: attrs.pads,
                    count_include_pad: attrs.count_include_pad,
                })
            }
            JsOpKind::GlobalAveragePool => OpKind::GlobalAveragePool,
            JsOpKind::Reshape { attrs } => {
                OpKind::Reshape(maku::ReshapeAttrs {
                    shape: attrs.shape.clone(),
                    allowzero: attrs.allowzero,
                })
            }
            JsOpKind::Transpose { attrs } => {
                OpKind::Transpose(maku::TransposeAttrs {
                    perm: attrs.perm.clone(),
                })
            }
            JsOpKind::Concat { attrs } => {
                OpKind::Concat(maku::ConcatAttrs {
                    axis: attrs.axis,
                })
            }
        };

        // Input/output type information should really be inferred, but
        // in MVP it works even if we skip non-Constants.
        let node = Node {
            id: node_id,
            op,
            inputs: input_ids,
            output: output_id,
        };
        nodes.push(node);
    }

    let outputs: Vec<ValueId> = js_graph
        .outputs
        .iter()
        .map(|s| str_to_value_id(s))
        .collect();

    Graph {
        nodes,
        outputs,
        value_types,
    }
}

/// ---------- Engine exposed to WASM ----------

#[wasm_bindgen]
pub struct WasmEngine {
    backend: CpuBackend,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEngine {
        console_error_panic_hook::set_once(); // For outputting panic to console (optional)
        WasmEngine {
            backend: CpuBackend::new(),
        }
    }

    /// graph: JS object representing JsGraph
    /// inputs: JS object of { [valueId: string]: JsTensor }
    ///
    /// Returns: JS object of { [valueId: string]: JsTensor }
    #[wasm_bindgen]
    pub fn run(&mut self, graph: JsValue, inputs: JsValue) -> Result<JsValue, JsValue> {
        // JsValue -> Rust構造体 (serde_wasm_bindgen)
        let js_graph: JsGraph = serde_wasm_bindgen::from_value(graph)
            .map_err(|e| JsValue::from_str(&format!("graph parse error: {}", e)))?;
        let js_inputs: JsInputs = serde_wasm_bindgen::from_value(inputs)
            .map_err(|e| JsValue::from_str(&format!("inputs parse error: {}", e)))?;

        // JsGraph -> core Graph
        let core_graph = build_core_graph(&js_graph);

        // Convert input tensor to core HashMap<ValueId, Tensor>
        let mut core_inputs = HashMap::new();
        for (key, js_t) in js_inputs {
            let vid = str_to_value_id(&key);
            let t = js_tensor_to_core(&js_t);
            core_inputs.insert(vid, t);
        }

        // Execute
        let core_outputs = self
            .backend
            .run(&core_graph, &core_inputs)
            .map_err(|e| JsValue::from_str(&format!("run error: {}", e)))?;

        // Convert return value to JsOutputs
        let mut js_outputs: JsOutputs = HashMap::new();
        for (vid, t) in core_outputs {
            let key = format!("{}", vid.0); // ValueId(u32) -> "0", "1", ...
            js_outputs.insert(key, core_tensor_to_js(&t));
        }

        // Rust struct -> JsValue
        serde_wasm_bindgen::to_value(&js_outputs)
            .map_err(|e| JsValue::from_str(&format!("to_value error: {}", e)))
    }
}
