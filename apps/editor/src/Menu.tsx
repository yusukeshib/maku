import { useState } from "react";
import css from "./Menu.module.css";
import { blockDefs, isBlockType } from "./project";
import { useAppStore } from "./store";

export function Menu() {
  const [value, setValue] = useState("");
  const [addBlock, dumpProject] = useAppStore((s) => [
    s.addBlock,
    s.dumpProject,
  ]);
  const handleSelect = (evt: React.ChangeEvent<HTMLSelectElement>) => {
    if (isBlockType(evt.target.value)) {
      addBlock(evt.target.value);
      setValue("");
    }
  };
  return (
    <div className={css.container}>
      <select value={value} onChange={handleSelect}>
        <option value="">(select)</option>
        {blockDefs.map((def) => (
          <option key={def.type} value={def.type}>
            {def.type}
          </option>
        ))}
      </select>
      <button onClick={dumpProject}>dump</button>
    </div>
  );
}
