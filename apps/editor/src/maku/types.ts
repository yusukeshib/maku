import { z } from "zod";

// ========== Base Schemas ==========

export const JsTensorSchema = z.object({
  shape: z.array(z.number()),
  data: z.array(z.number()),
});

// ========== Attribute Schemas ==========

export const Conv2DAttrsSchema = z.object({
  kernel_shape: z.tuple([z.number(), z.number()]), // [kh, kw]
  strides: z.tuple([z.number(), z.number()]).default([1, 1]), // [sh, sw], default: [1, 1]
  pads: z.tuple([z.number(), z.number(), z.number(), z.number()]).default([0, 0, 0, 0]), // [top, left, bottom, right]
  dilations: z.tuple([z.number(), z.number()]).default([1, 1]), // [dh, dw], default: [1, 1]
  group: z.number().default(1), // default: 1
});

export const DepthwiseConv2DAttrsSchema = z.object({
  kernel_shape: z.tuple([z.number(), z.number()]), // [kh, kw]
  strides: z.tuple([z.number(), z.number()]).default([1, 1]), // [sh, sw], default: [1, 1]
  pads: z.tuple([z.number(), z.number(), z.number(), z.number()]).default([0, 0, 0, 0]), // [top, left, bottom, right]
  dilations: z.tuple([z.number(), z.number()]).default([1, 1]), // [dh, dw], default: [1, 1]
  depth_multiplier: z.number().default(1), // default: 1
});

export const BatchNormAttrsSchema = z.object({
  epsilon: z.number().default(1e-5), // default: 1e-5
  momentum: z.number().default(0.9), // default: 0.9
});

export const AveragePoolAttrsSchema = z.object({
  kernel_shape: z.tuple([z.number(), z.number()]), // [kh, kw]
  strides: z.tuple([z.number(), z.number()]).default([1, 1]), // [sh, sw], default: [1, 1]
  pads: z.tuple([z.number(), z.number(), z.number(), z.number()]).default([0, 0, 0, 0]), // [top, left, bottom, right]
  count_include_pad: z.boolean().default(false), // default: false
});

export const MatMulAttrsSchema = z.object({
  transA: z.boolean().default(false), // default: false
  transB: z.boolean().default(false), // default: false
});

export const ReshapeAttrsSchema = z.object({
  shape: z.array(z.number()), // target shape (can include -1 for auto-inference)
  allowzero: z.boolean().default(false), // default: false
});

export const TransposeAttrsSchema = z.object({
  perm: z.array(z.number()), // axis permutation, e.g., [0, 3, 1, 2] for NHWC->NCHW
});

export const ConcatAttrsSchema = z.object({
  axis: z.number(), // axis to concatenate along
});

// ========== Operation Schemas (Discriminated Union) ==========

export const JsOpSchema = z.discriminatedUnion("op", [
  z.object({ op: z.literal("Input") }),
  z.object({ op: z.literal("Constant"), tensor: JsTensorSchema }),
  z.object({ op: z.literal("Add") }),
  z.object({ op: z.literal("Mul") }),
  z.object({ op: z.literal("MatMul"), attrs: MatMulAttrsSchema.optional() }),
  z.object({ op: z.literal("Relu") }),
  z.object({ op: z.literal("Relu6") }),
  z.object({ op: z.literal("HardSwish") }),
  z.object({ op: z.literal("Conv2D"), attrs: Conv2DAttrsSchema }),
  z.object({ op: z.literal("DepthwiseConv2D"), attrs: DepthwiseConv2DAttrsSchema }),
  z.object({ op: z.literal("BatchNorm"), attrs: BatchNormAttrsSchema.optional() }),
  z.object({ op: z.literal("AveragePool"), attrs: AveragePoolAttrsSchema }),
  z.object({ op: z.literal("GlobalAveragePool") }),
  z.object({ op: z.literal("Reshape"), attrs: ReshapeAttrsSchema }),
  z.object({ op: z.literal("Transpose"), attrs: TransposeAttrsSchema }),
  z.object({ op: z.literal("Concat"), attrs: ConcatAttrsSchema }),
]);

// ========== Node Schema ==========

export const JsNodeSchema = z.intersection(
  z.object({
    id: z.string(),
    inputs: z.array(z.string()),
    output: z.string(),
  }),
  JsOpSchema
);

// ========== Graph Schema ==========

export const JsGraphSchema = z.object({
  nodes: z.array(JsNodeSchema),
  outputs: z.array(z.string()),
});

// ========== Inferred TypeScript Types ==========

export type JsTensor = z.infer<typeof JsTensorSchema>;
export type Conv2DAttrs = z.infer<typeof Conv2DAttrsSchema>;
export type DepthwiseConv2DAttrs = z.infer<typeof DepthwiseConv2DAttrsSchema>;
export type BatchNormAttrs = z.infer<typeof BatchNormAttrsSchema>;
export type AveragePoolAttrs = z.infer<typeof AveragePoolAttrsSchema>;
export type MatMulAttrs = z.infer<typeof MatMulAttrsSchema>;
export type ReshapeAttrs = z.infer<typeof ReshapeAttrsSchema>;
export type TransposeAttrs = z.infer<typeof TransposeAttrsSchema>;
export type ConcatAttrs = z.infer<typeof ConcatAttrsSchema>;
export type JsOp = z.infer<typeof JsOpSchema>;
export type JsNode = z.infer<typeof JsNodeSchema>;
export type JsGraph = z.infer<typeof JsGraphSchema>;
