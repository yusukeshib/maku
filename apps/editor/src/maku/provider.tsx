import init, { WasmEngine } from "@makulib/web";
import { createContext, useState, useEffect, useContext, } from 'react';

const MakuContext = createContext<{ engine: WasmEngine }>(null!);

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

export function useMaku() {
  return useContext(MakuContext);
}
