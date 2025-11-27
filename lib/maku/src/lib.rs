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

/// Attribute structures for each operation
#[derive(Debug, Clone)]
pub struct Conv2DAttrs {
    pub kernel_shape: [usize; 2],
    pub strides: [usize; 2],
    pub pads: [usize; 4],
    pub dilations: [usize; 2],
    pub group: usize,
}

#[derive(Debug, Clone)]
pub struct DepthwiseConv2DAttrs {
    pub kernel_shape: [usize; 2],
    pub strides: [usize; 2],
    pub pads: [usize; 4],
    pub dilations: [usize; 2],
    pub depth_multiplier: usize,
}

#[derive(Debug, Clone)]
pub struct BatchNormAttrs {
    pub epsilon: f32,
    pub momentum: f32,
}

#[derive(Debug, Clone)]
pub struct AveragePoolAttrs {
    pub kernel_shape: [usize; 2],
    pub strides: [usize; 2],
    pub pads: [usize; 4],
    pub count_include_pad: bool,
}

#[derive(Debug, Clone)]
pub struct MatMulAttrs {
    pub trans_a: bool,
    pub trans_b: bool,
}

#[derive(Debug, Clone)]
pub struct ReshapeAttrs {
    pub shape: Vec<isize>,
    pub allowzero: bool,
}

#[derive(Debug, Clone)]
pub struct TransposeAttrs {
    pub perm: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ConcatAttrs {
    pub axis: usize,
}

/// Types of supported operations
#[derive(Debug, Clone)]
pub enum OpKind {
    Input,                                      // Value provided from outside
    Constant(Tensor),                           // Constant embedded in the graph
    Add,                                        // Element-wise addition
    Mul,                                        // Element-wise multiplication
    MatMul(Option<MatMulAttrs>),                // Matrix multiplication (2D x 2D)
    Relu,                                       // Element-wise ReLU
    Relu6,                                      // Element-wise ReLU6 (clamp to [0, 6])
    HardSwish,                                  // Element-wise HardSwish
    Conv2D(Conv2DAttrs),                        // 2D Convolution
    DepthwiseConv2D(DepthwiseConv2DAttrs),      // Depthwise 2D Convolution
    BatchNorm(Option<BatchNormAttrs>),          // Batch Normalization
    AveragePool(AveragePoolAttrs),              // Average Pooling
    GlobalAveragePool,                          // Global Average Pooling
    Reshape(ReshapeAttrs),                      // Reshape tensor
    Transpose(TransposeAttrs),                  // Transpose tensor
    Concat(ConcatAttrs),                        // Concatenate tensors
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
                OpKind::Mul => {
                    let a = values.get(&node.inputs[0]).expect("Mul missing input 0");
                    let b = values.get(&node.inputs[1]).expect("Mul missing input 1");
                    mul(a, b)?
                }
                OpKind::MatMul(attrs) => {
                    let a = values.get(&node.inputs[0]).expect("MatMul missing input 0");
                    let b = values.get(&node.inputs[1]).expect("MatMul missing input 1");
                    matmul(a, b, attrs.as_ref())?
                }
                OpKind::Relu => {
                    let x = values.get(&node.inputs[0]).expect("Relu missing input 0");
                    relu(x)
                }
                OpKind::Relu6 => {
                    let x = values.get(&node.inputs[0]).expect("Relu6 missing input 0");
                    relu6(x)
                }
                OpKind::HardSwish => {
                    let x = values.get(&node.inputs[0]).expect("HardSwish missing input 0");
                    hard_swish(x)
                }
                OpKind::Conv2D(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("Conv2D missing input 0");
                    let kernel = values.get(&node.inputs[1]).expect("Conv2D missing kernel");
                    let bias = if node.inputs.len() > 2 {
                        Some(values.get(&node.inputs[2]).expect("Conv2D missing bias"))
                    } else {
                        None
                    };
                    conv2d(input, kernel, bias, attrs)?
                }
                OpKind::DepthwiseConv2D(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("DepthwiseConv2D missing input 0");
                    let kernel = values.get(&node.inputs[1]).expect("DepthwiseConv2D missing kernel");
                    let bias = if node.inputs.len() > 2 {
                        Some(values.get(&node.inputs[2]).expect("DepthwiseConv2D missing bias"))
                    } else {
                        None
                    };
                    depthwise_conv2d(input, kernel, bias, attrs)?
                }
                OpKind::BatchNorm(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("BatchNorm missing input");
                    let scale = values.get(&node.inputs[1]).expect("BatchNorm missing scale");
                    let bias = values.get(&node.inputs[2]).expect("BatchNorm missing bias");
                    let mean = values.get(&node.inputs[3]).expect("BatchNorm missing mean");
                    let var = values.get(&node.inputs[4]).expect("BatchNorm missing var");
                    batch_norm(input, scale, bias, mean, var, attrs.as_ref())?
                }
                OpKind::AveragePool(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("AveragePool missing input");
                    average_pool(input, attrs)?
                }
                OpKind::GlobalAveragePool => {
                    let input = values.get(&node.inputs[0]).expect("GlobalAveragePool missing input");
                    global_average_pool(input)?
                }
                OpKind::Reshape(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("Reshape missing input");
                    reshape(input, attrs)?
                }
                OpKind::Transpose(attrs) => {
                    let input = values.get(&node.inputs[0]).expect("Transpose missing input");
                    transpose(input, attrs)?
                }
                OpKind::Concat(attrs) => {
                    let inputs: Vec<&Tensor> = node.inputs.iter()
                        .map(|id| values.get(id).expect("Concat missing input"))
                        .collect();
                    concat(&inputs, attrs)?
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

fn mul(a: &Tensor, b: &Tensor) -> anyhow::Result<Tensor> {
    anyhow::ensure!(
        a.desc.shape == b.desc.shape,
        "Mul shape mismatch: {:?} vs {:?}",
        a.desc.shape,
        b.desc.shape
    );

    let mut out = Tensor::zeros(a.desc.shape.clone());
    for i in 0..a.len() {
        out.data[i] = a.data[i] * b.data[i];
    }
    Ok(out)
}

fn matmul(a: &Tensor, b: &Tensor, _attrs: Option<&MatMulAttrs>) -> anyhow::Result<Tensor> {
    // TODO: Implement transpose support
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

fn relu6(x: &Tensor) -> Tensor {
    let mut out = x.clone();
    for v in &mut out.data {
        *v = v.clamp(0.0, 6.0);
    }
    out
}

fn hard_swish(x: &Tensor) -> Tensor {
    let mut out = x.clone();
    for v in &mut out.data {
        // HardSwish: x * relu6(x + 3) / 6
        *v = *v * (*v + 3.0).clamp(0.0, 6.0) / 6.0;
    }
    out
}

fn conv2d(_input: &Tensor, _kernel: &Tensor, _bias: Option<&Tensor>, _attrs: &Conv2DAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement Conv2D kernel
    anyhow::bail!("Conv2D not yet implemented")
}

fn depthwise_conv2d(_input: &Tensor, _kernel: &Tensor, _bias: Option<&Tensor>, _attrs: &DepthwiseConv2DAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement DepthwiseConv2D kernel
    anyhow::bail!("DepthwiseConv2D not yet implemented")
}

fn batch_norm(_input: &Tensor, _scale: &Tensor, _bias: &Tensor, _mean: &Tensor, _var: &Tensor, _attrs: Option<&BatchNormAttrs>) -> anyhow::Result<Tensor> {
    // TODO: Implement BatchNorm kernel
    anyhow::bail!("BatchNorm not yet implemented")
}

fn average_pool(_input: &Tensor, _attrs: &AveragePoolAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement AveragePool kernel
    anyhow::bail!("AveragePool not yet implemented")
}

fn global_average_pool(_input: &Tensor) -> anyhow::Result<Tensor> {
    // TODO: Implement GlobalAveragePool kernel
    anyhow::bail!("GlobalAveragePool not yet implemented")
}

fn reshape(_input: &Tensor, _attrs: &ReshapeAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement Reshape kernel
    anyhow::bail!("Reshape not yet implemented")
}

fn transpose(_input: &Tensor, _attrs: &TransposeAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement Transpose kernel
    anyhow::bail!("Transpose not yet implemented")
}

fn concat(_inputs: &[&Tensor], _attrs: &ConcatAttrs) -> anyhow::Result<Tensor> {
    // TODO: Implement Concat kernel
    anyhow::bail!("Concat not yet implemented")
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
