import { type Edge } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { type MakuFlowNode, OP_TO_NODE_TYPE } from "./components/types";

export const initialNodes: MakuFlowNode[] = [
  {
    id: "input1",
    type: OP_TO_NODE_TYPE.Input,
    position: { x: 100, y: 50 },
    data: { op: "Input", label: "Input Layer" },
  },
  {
    id: "conv1",
    type: OP_TO_NODE_TYPE.Conv2D,
    position: { x: 100, y: 150 },
    data: {
      op: "Conv2D",
      attrs: {
        kernel_shape: [3, 3],
        strides: [1, 1],
        pads: [1, 1, 1, 1],
        dilations: [1, 1],
        group: 1,
      },
      label: "Conv2D 3x3",
    },
  },
  {
    id: "relu1",
    type: OP_TO_NODE_TYPE.Relu6,
    position: { x: 100, y: 270 },
    data: { op: "Relu6", label: "ReLU6" },
  },
  {
    id: "pool1",
    type: OP_TO_NODE_TYPE.GlobalAveragePool,
    position: { x: 100, y: 370 },
    data: { op: "GlobalAveragePool", label: "Global Avg Pool" },
  },
];

export const initialEdges: Edge[] = [
  { id: "e1", source: "input1", target: "conv1" },
  { id: "e2", source: "conv1", target: "relu1" },
  { id: "e3", source: "relu1", target: "pool1" },
];
