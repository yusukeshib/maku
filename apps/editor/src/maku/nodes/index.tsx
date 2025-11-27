import type { NodeTypes, NodeProps } from "@xyflow/react";
import { BaseNode } from "./BaseNode";
import { OP_TO_NODE_TYPE, NODE_TYPE_TO_OP, type MakuOpType, type MakuNodeData } from "./types";

/**
 * Configuration for each operation type
 */
const OP_CONFIG: Record<MakuOpType, { color: string; icon: string; category: string }> = {
  // Input/Output
  Input: { color: "#10b981", icon: "📥", category: "IO" },
  Constant: { color: "#8b5cf6", icon: "🔢", category: "IO" },

  // Basic Operations
  Add: { color: "#3b82f6", icon: "➕", category: "Math" },
  Mul: { color: "#3b82f6", icon: "✖️", category: "Math" },
  MatMul: { color: "#3b82f6", icon: "🔲", category: "Math" },

  // Activations
  Relu: { color: "#f59e0b", icon: "⚡", category: "Activation" },
  Relu6: { color: "#f59e0b", icon: "⚡", category: "Activation" },
  HardSwish: { color: "#f59e0b", icon: "🌊", category: "Activation" },

  // Convolutions
  Conv2D: { color: "#ec4899", icon: "🔍", category: "Conv" },
  DepthwiseConv2D: { color: "#ec4899", icon: "🔎", category: "Conv" },

  // Normalization
  BatchNorm: { color: "#14b8a6", icon: "📊", category: "Norm" },

  // Pooling
  AveragePool: { color: "#06b6d4", icon: "💧", category: "Pool" },
  GlobalAveragePool: { color: "#06b6d4", icon: "🌊", category: "Pool" },

  // Tensor Ops
  Reshape: { color: "#84cc16", icon: "🔄", category: "Tensor" },
  Transpose: { color: "#84cc16", icon: "🔃", category: "Tensor" },
  Concat: { color: "#84cc16", icon: "🔗", category: "Tensor" },
};

/**
 * Create a custom node component for a specific operation type
 */
function createNodeComponent(opType: MakuOpType) {
  const config = OP_CONFIG[opType];

  return function NodeComponent(props: NodeProps) {
    return (
      <BaseNode
        {...props}
        data={props.data as MakuNodeData}
        color={config.color}
        icon={config.icon}
      />
    );
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

/**
 * Export configuration for use in other components
 */
export { OP_CONFIG, OP_TO_NODE_TYPE, NODE_TYPE_TO_OP };
export type { MakuNodeData, MakuFlowNode, MakuOpType } from "./types";
