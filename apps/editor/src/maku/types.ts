export type JsTensor = {
  shape: number[];
  data: number[];
};

export type JsNode = {
  id: string;
  op: "Input" | "Constant" | "Add" | "MatMul" | "Relu";
  inputs: string[];
  output: string;
  tensor?: JsTensor;
};

export type JsGraph = {
  nodes: JsNode[];
  outputs: string[];
};
