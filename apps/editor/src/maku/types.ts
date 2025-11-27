export type JsTensor = {
  shape: number[];
  data: number[];
};

// Attributes for each operation type
export type Conv2DAttrs = {
  kernel_shape: [number, number]; // [kh, kw]
  strides?: [number, number]; // [sh, sw], default: [1, 1]
  pads?: [number, number, number, number]; // [top, left, bottom, right], default: [0, 0, 0, 0]
  dilations?: [number, number]; // [dh, dw], default: [1, 1]
  group?: number; // default: 1
};

export type DepthwiseConv2DAttrs = {
  kernel_shape: [number, number]; // [kh, kw]
  strides?: [number, number]; // [sh, sw], default: [1, 1]
  pads?: [number, number, number, number]; // [top, left, bottom, right], default: [0, 0, 0, 0]
  dilations?: [number, number]; // [dh, dw], default: [1, 1]
  depth_multiplier?: number; // default: 1
};

export type BatchNormAttrs = {
  epsilon?: number; // default: 1e-5
  momentum?: number; // default: 0.9
};

export type AveragePoolAttrs = {
  kernel_shape: [number, number]; // [kh, kw]
  strides?: [number, number]; // [sh, sw], default: [1, 1]
  pads?: [number, number, number, number]; // [top, left, bottom, right], default: [0, 0, 0, 0]
  count_include_pad?: boolean; // default: false
};

export type MatMulAttrs = {
  transA?: boolean; // default: false
  transB?: boolean; // default: false
};

export type ReshapeAttrs = {
  shape: number[]; // target shape (can include -1 for auto-inference)
  allowzero?: boolean; // default: false
};

export type TransposeAttrs = {
  perm: number[]; // axis permutation, e.g., [0, 3, 1, 2] for NHWC->NCHW
};

export type ConcatAttrs = {
  axis: number; // axis to concatenate along
};

// Operation type definitions using discriminated union
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

export type JsGraph = {
  nodes: JsNode[];
  outputs: string[];
};
