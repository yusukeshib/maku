import {
  ReactFlow,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
  Background,
  Controls,
  type OnConnect,
  type OnEdgesChange,
  type OnNodesChange,
  type Edge,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { useCallback, useState } from "react";
import { nodeTypes, type MakuFlowNode, OP_TO_NODE_TYPE, type MakuOpType } from "./maku/nodes";
import { NodePalette } from "./components/NodePalette";

const initialNodes: MakuFlowNode[] = [
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

const initialEdges: Edge[] = [
  { id: "e1", source: "input1", target: "conv1" },
  { id: "e2", source: "conv1", target: "relu1" },
  { id: "e3", source: "relu1", target: "pool1" },
];

export function Editor() {
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);

  const onNodesChange: OnNodesChange<MakuFlowNode> = useCallback(
    changes =>
      setNodes(nodesSnapshot => applyNodeChanges(changes, nodesSnapshot) as MakuFlowNode[]),
    []
  );
  const onEdgesChange: OnEdgesChange = useCallback(
    changes => setEdges(edgesSnapshot => applyEdgeChanges(changes, edgesSnapshot)),
    []
  );
  const onConnect: OnConnect = useCallback(
    params => setEdges(edgesSnapshot => addEdge(params, edgesSnapshot)),
    []
  );

  const onAddNode = useCallback((opType: MakuOpType) => {
    // Create node data based on operation type
    let data: MakuFlowNode["data"];

    switch (opType) {
      case "Conv2D":
        data = {
          op: "Conv2D",
          attrs: {
            kernel_shape: [3, 3],
            strides: [1, 1],
            pads: [0, 0, 0, 0],
            dilations: [1, 1],
            group: 1,
          },
        };
        break;
      case "DepthwiseConv2D":
        data = {
          op: "DepthwiseConv2D",
          attrs: {
            kernel_shape: [3, 3],
            strides: [1, 1],
            pads: [0, 0, 0, 0],
            dilations: [1, 1],
            depth_multiplier: 1,
          },
        };
        break;
      case "AveragePool":
        data = {
          op: "AveragePool",
          attrs: {
            kernel_shape: [2, 2],
            strides: [2, 2],
            pads: [0, 0, 0, 0],
            count_include_pad: false,
          },
        };
        break;
      case "Reshape":
        data = { op: "Reshape", attrs: { shape: [-1], allowzero: false } };
        break;
      case "Transpose":
        data = { op: "Transpose", attrs: { perm: [0, 1, 2, 3] } };
        break;
      case "Concat":
        data = { op: "Concat", attrs: { axis: 0 } };
        break;
      case "Constant":
        data = { op: "Constant", tensor: { shape: [1], data: [0] } };
        break;
      case "MatMul":
        data = { op: "MatMul", attrs: { transA: false, transB: false } };
        break;
      case "BatchNorm":
        data = { op: "BatchNorm", attrs: { epsilon: 1e-5, momentum: 0.9 } };
        break;
      default:
        data = { op: opType } as MakuFlowNode["data"];
    }

    const newNode: MakuFlowNode = {
      id: `node-${Date.now()}`,
      type: OP_TO_NODE_TYPE[opType],
      position: {
        x: Math.random() * 400 + 100,
        y: Math.random() * 400 + 100,
      },
      data,
    };
    setNodes(nodes => [...nodes, newNode]);
  }, []);

  return (
    <div style={{ position: "absolute", left: 0, right: 0, top: 0, bottom: 0 }}>
      <NodePalette onAddNode={onAddNode} />
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}
