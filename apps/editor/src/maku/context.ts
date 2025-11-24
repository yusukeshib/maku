import { WasmEngine } from "@makulib/web";
import { createContext, useContext, } from 'react';

export const MakuContext = createContext<{ engine: WasmEngine }>(null!);

export function useMaku() {
  return useContext(MakuContext);
}

