import init, { WasmEngine } from "@makulib/web";
import { useState, useEffect, } from 'react';
import { MakuContext } from "./context";

export function MakuProvider(props: { children: React.ReactNode }) {
  const [engine, setEngine] = useState<WasmEngine | null>(null);

  useEffect(() => {
    init().then(() => {
      const engine = new WasmEngine()
      setEngine(engine);
    });
  }, []);

  if (!engine) return null;

  return (
    <MakuContext.Provider value={{ engine }} {...props} />
  )
}

