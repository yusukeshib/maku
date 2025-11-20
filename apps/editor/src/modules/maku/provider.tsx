import init, { WasmEngine } from "@makulib/web"; // Adjust to match wasm-pack output
import { createContext, useContext, useMemo } from 'react';

const MakuContext = createContext<{ engine: WasmEngine }>(null!);

export function MakuProvider({ children }: { children: React.ReactNode }) {
  const engine = useMemo(() => new WasmEngine(), []);
  return (
    <MakuContext.Provider value={{ engine }}>
      {children}
    </<MakuContext.Provider>
  )
}
