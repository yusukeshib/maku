import type { Node, NodeProps, NodeTypes } from "@xyflow/react";
import type { MakuOp } from "../maku/types";
import { BaseNode } from "./BaseNode";

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

/**
 * Configuration for each operation type
 */
export const OP_CONFIG: Record<MakuOpType, { category: string }> = {
  // Input/Output
  Input: { category: "IO" },
  Constant: { category: "IO" },

  // Basic Operations
  Add: { category: "Math" },
  Mul: { category: "Math" },
  MatMul: { category: "Math" },

  // Activations
  Relu: { category: "Activation" },
  Relu6: { category: "Activation" },
  HardSwish: { category: "Activation" },

  // Convolutions
  Conv2D: { category: "Conv" },
  DepthwiseConv2D: { category: "Conv" },

  // Normalization
  BatchNorm: { category: "Norm" },

  // Pooling
  AveragePool: { category: "Pool" },
  GlobalAveragePool: { category: "Pool" },

  // Tensor Ops
  Reshape: { category: "Tensor" },
  Transpose: { category: "Tensor" },
  Concat: { category: "Tensor" },
};

/**
 * Create a custom node component for a specific operation type
 */
function createNodeComponent(opType: MakuOpType) {
  return function NodeComponent(props: NodeProps) {
    return <BaseNode {...props} type={opType} data={props.data as MakuNodeData} />;
  };
}

/**
 * Generate all node types for ReactFlow
 * This is automatically generated from the Zod schema via OP_TO_NODE_TYPE
 */
export const nodeTypes: NodeTypes = Object.entries(OP_TO_NODE_TYPE).reduce(
  (acc, [opType, nodeType]) => {
    acc[nodeType] = createNodeComponent(opType as MakuOpType);
    return acc;
  },
  {} as NodeTypes
);
