import type { Node } from "@xyflow/react";
import type { MakuOp } from "../types";

/**
 * ReactFlow Node data for Maku operations
 * Extends MakuOp with additional node metadata
 */
export type MakuNodeData = MakuOp & {
  label?: string;
};

/**
 * ReactFlow Node type for Maku operations
 */
export type MakuFlowNode = Node<MakuNodeData>;

/**
 * Extract the operation name from MakuOp discriminated union
 */
export type MakuOpType = MakuOp["op"];

/**
 * Map each operation type to a custom node type name
 */
export const OP_TO_NODE_TYPE: Record<MakuOpType, string> = {
  Input: "makuInput",
  Constant: "makuConstant",
  Add: "makuAdd",
  Mul: "makuMul",
  MatMul: "makuMatMul",
  Relu: "makuRelu",
  Relu6: "makuRelu6",
  HardSwish: "makuHardSwish",
  Conv2D: "makuConv2D",
  DepthwiseConv2D: "makuDepthwiseConv2D",
  BatchNorm: "makuBatchNorm",
  AveragePool: "makuAveragePool",
  GlobalAveragePool: "makuGlobalAveragePool",
  Reshape: "makuReshape",
  Transpose: "makuTranspose",
  Concat: "makuConcat",
};

/**
 * Reverse mapping: node type to operation type
 */
export const NODE_TYPE_TO_OP = Object.fromEntries(
  Object.entries(OP_TO_NODE_TYPE).map(([k, v]) => [v, k])
) as Record<string, MakuOpType>;
