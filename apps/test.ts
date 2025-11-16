import init, { WasmEngine } from "maku_web"; // Adjust to match wasm-pack output

type JsTensor = {
  shape: number[];
  data: number[];
};

type JsNode = {
  id: string;
  op: "Input" | "Constant" | "Add" | "MatMul" | "Relu";
  inputs: string[];
  output: string;
  tensor?: JsTensor;
};

type JsGraph = {
  nodes: JsNode[];
  outputs: string[];
};

async function runExample() {
  // await init(); // Initialize wasm (if needed)

  const engine = new WasmEngine();

  // Small graph like y = Relu( MatMul(x, w) )
  const graph: JsGraph = {
    nodes: [
      {
        id: "matmul1",
        op: "MatMul",
        inputs: ["x", "w"],
        output: "z",
      },
      {
        id: "relu1",
        op: "Relu",
        inputs: ["z"],
        output: "y",
      },
    ],
    outputs: ["y"],
  };

  const x: JsTensor = {
    shape: [2, 3],
    data: [1, 2, 3, 4, 5, 6],
  };

  const w: JsTensor = {
    shape: [3, 1],
    data: [1, 0.5, -1],
  };

  const inputs = {
    x,
    w,
  };

  const result = engine.run(graph as any, inputs as any) as any;
  // In wasm-bindgen terms it's JsValue, so the type can be received as any or unknown

  console.log("outputs:", result);
  // Expected form: { "…": { shape: [2,1], data: [...] } }
}
