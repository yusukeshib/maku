import { useMaku } from "./maku/provider"
import type { JsGraph, JsTensor } from "./modules/maku/types";

export function NodeMain() {
  const { engine } = useMaku();

  const handleClick = () => {
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

    const inputs = { x, w, };

    const result = engine.run(graph, inputs)

    console.log("outputs:", result);
  }
  return (
    <div>
      <button onClick={handleClick}>Run Graph</button>
      <pre>
      </pre>
    </div>
  )
}
