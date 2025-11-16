use std::collections::HashMap;

/// ---------- Basic types: Tensor / Op / Graph ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone, Copy)]
pub enum DType {
    F32,
}

#[derive(Debug, Clone)]
pub struct TensorDesc {
    pub dtype: DType,
    /// Example: [2, 3] = 2x3 matrix
    pub shape: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Tensor {
    pub desc: TensorDesc,
    /// Row-major flat array (C-style)
    pub data: Vec<f32>,
}

impl Tensor {
    pub fn new(desc: TensorDesc, data: Vec<f32>) -> Self {
        let expected: usize = desc.shape.iter().product();
        assert_eq!(
            expected,
            data.len(),
            "tensor data length {} does not match shape {:?} ({} elements expected)",
            data.len(),
            desc.shape,
            expected
        );
        Tensor { desc, data }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let len: usize = shape.iter().product();
        Tensor {
            desc: TensorDesc {
                dtype: DType::F32,
                shape,
            },
            data: vec![0.0; len],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Types of supported operations (MVP version)
#[derive(Debug, Clone)]
pub enum OpKind {
    Input,            // Value provided from outside
    Constant(Tensor), // Constant embedded in the graph
    Add,              // Element-wise addition
    MatMul,           // Matrix multiplication (2D x 2D)
    Relu,             // Element-wise ReLU
}

/// Node in graph
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub op: OpKind,
    /// Input value IDs
    pub inputs: Vec<ValueId>,
    /// Output value ID (MVP assumes only one)
    pub output: ValueId,
}

/// Entire computation graph
#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    /// ValueIds to extract as the graph's final outputs
    pub outputs: Vec<ValueId>,
    /// Type information (Shape) for each ValueId
    pub value_types: HashMap<ValueId, TensorDesc>,
}

/// ---------- CPU Backend main body ----------

pub struct CpuBackend;

impl CpuBackend {
    pub fn new() -> Self {
        CpuBackend
    }

    /// Execute the graph.
    ///
    /// * `graph` : Pre-built computation graph
    /// * `input_tensors` : ValueId -> Tensor corresponding to Input Op
    ///
    /// Return value is ValueId -> Tensor corresponding to graph.outputs
    pub fn run(
        &self,
        graph: &Graph,
        input_tensors: &HashMap<ValueId, Tensor>,
    ) -> anyhow::Result<HashMap<ValueId, Tensor>> {
        // Entity of ValueId -> Tensor
        let mut values: HashMap<ValueId, Tensor> = HashMap::new();

        // Register Input first
        for (id, t) in input_tensors {
            values.insert(*id, t.clone());
        }

        // Execute nodes in order (MVP assumes topologically sorted)
        for node in &graph.nodes {
            let out = match &node.op {
                OpKind::Input => {
                    // Assumes Input is already in values
                    continue;
                }
                OpKind::Constant(t) => t.clone(),
                OpKind::Add => {
                    let a = values.get(&node.inputs[0]).expect("Add missing input 0");
                    let b = values.get(&node.inputs[1]).expect("Add missing input 1");
                    add(a, b)?
                }
                OpKind::MatMul => {
                    let a = values.get(&node.inputs[0]).expect("MatMul missing input 0");
                    let b = values.get(&node.inputs[1]).expect("MatMul missing input 1");
                    matmul(a, b)?
                }
                OpKind::Relu => {
                    let x = values.get(&node.inputs[0]).expect("Relu missing input 0");
                    relu(x)
                }
            };

            values.insert(node.output, out);
        }

        // Extract only outputs
        let mut outputs = HashMap::new();
        for &vid in &graph.outputs {
            if let Some(t) = values.get(&vid) {
                outputs.insert(vid, t.clone());
            } else {
                anyhow::bail!("missing output tensor for {:?}", vid);
            }
        }
        Ok(outputs)
    }
}

/// ---------- Implementation of individual operations ----------

fn add(a: &Tensor, b: &Tensor) -> anyhow::Result<Tensor> {
    anyhow::ensure!(
        a.desc.shape == b.desc.shape,
        "Add shape mismatch: {:?} vs {:?}",
        a.desc.shape,
        b.desc.shape
    );

    let mut out = Tensor::zeros(a.desc.shape.clone());
    for i in 0..a.len() {
        out.data[i] = a.data[i] + b.data[i];
    }
    Ok(out)
}

fn matmul(a: &Tensor, b: &Tensor) -> anyhow::Result<Tensor> {
    anyhow::ensure!(
        a.desc.shape.len() == 2 && b.desc.shape.len() == 2,
        "MatMul expects 2D tensors, got {:?} and {:?}",
        a.desc.shape,
        b.desc.shape
    );

    let (m, k1) = (a.desc.shape[0], a.desc.shape[1]);
    let (k2, n) = (b.desc.shape[0], b.desc.shape[1]);
    anyhow::ensure!(k1 == k2, "MatMul inner dim mismatch: {} vs {}", k1, k2);

    let mut out = Tensor::zeros(vec![m, n]);

    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..k1 {
                let a_idx = i * k1 + k;
                let b_idx = k * n + j;
                sum += a.data[a_idx] * b.data[b_idx];
            }
            out.data[i * n + j] = sum;
        }
    }

    Ok(out)
}

fn relu(x: &Tensor) -> Tensor {
    let mut out = x.clone();
    for v in &mut out.data {
        if *v < 0.0 {
            *v = 0.0;
        }
    }
    out
}

/// ---------- Mini sample usage ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_graph() {
        // ValueId assignment
        let x_id = ValueId(0);
        let w_id = ValueId(1);
        let y_id = ValueId(2);

        // Type information (ideally want to fill all, but MVP minimum)
        let mut value_types = HashMap::new();
        value_types.insert(
            x_id,
            TensorDesc {
                dtype: DType::F32,
                shape: vec![2, 3], // 2x3
            },
        );
        value_types.insert(
            w_id,
            TensorDesc {
                dtype: DType::F32,
                shape: vec![3, 1], // 3x1
            },
        );
        value_types.insert(
            y_id,
            TensorDesc {
                dtype: DType::F32,
                shape: vec![2, 1], // 2x1 = MatMul(x, w)
            },
        );

        // Define node: y = MatMul(x, w)
        let node = Node {
            id: NodeId(0),
            op: OpKind::MatMul,
            inputs: vec![x_id, w_id],
            output: y_id,
        };

        let graph = Graph {
            nodes: vec![node],
            outputs: vec![y_id],
            value_types,
        };

        // Input tensors
        let x = Tensor::new(
            TensorDesc {
                dtype: DType::F32,
                shape: vec![2, 3],
            },
            vec![
                1.0, 2.0, 3.0, // row 0
                4.0, 5.0, 6.0, // row 1
            ],
        );

        let w = Tensor::new(
            TensorDesc {
                dtype: DType::F32,
                shape: vec![3, 1],
            },
            vec![
                1.0, // col 0
                0.5, -1.0,
            ],
        );

        let mut inputs = HashMap::new();
        inputs.insert(x_id, x);
        inputs.insert(w_id, w);

        let backend = CpuBackend::new();
        let outputs = backend.run(&graph, &inputs).unwrap();

        let y = outputs.get(&y_id).unwrap();
        println!("y: {:?}", y);

        assert_eq!(y.desc.shape, vec![2, 1]);
        // If you want to roughly verify the calculation result, you can assert here
    }
}
