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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum JsOpKind {
    Input,
    Constant { tensor: JsTensor },
    Add,
    MatMul,
    Relu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsNode {
    pub id: String,
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
            JsOpKind::MatMul => OpKind::MatMul,
            JsOpKind::Relu => OpKind::Relu,
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

    /// graph: JsGraph を表す JS オブジェクト
    /// inputs: { [valueId: string]: JsTensor } な JS オブジェクト
    ///
    /// 返り値: { [valueId: string]: JsTensor } な JS オブジェクト
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
