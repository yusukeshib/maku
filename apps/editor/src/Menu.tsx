import { useState } from "react";
import css from "./Menu.module.css";
import { blockDefs, isBlockType } from "./project";
import { useAppStore } from "./store";
import invariant from "tiny-invariant";

export function Menu() {
  const [value, setValue] = useState("");
  const [addBlock, dumpProject, loadProject] = useAppStore((s) => [
    s.addBlock,
    s.dumpProject,
    s.loadProject,
  ]);
  const handleSelect = (evt: React.ChangeEvent<HTMLSelectElement>) => {
    if (isBlockType(evt.target.value)) {
      addBlock(evt.target.value);
      setValue("");
    }
  };
  const handleLoad = async () => {
    const file = await browse();
    await loadProject(file);
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
      <button onClick={handleLoad}>load</button>
    </div>
  );
}

const browse = (): Promise<File> =>
  new Promise((resolve, reject) => {
    const input = document.createElement("input");
    input.type = "file";
    input.style.display = "none";
    input.accept = "application/json";
    input.multiple = false;
    const elem = document.body.appendChild(input);

    // done
    elem.onchange = (evt: Event) => {
      invariant(
        evt.target &&
          "files" in evt.target &&
          evt.target.files instanceof FileList,
      );
      const file = evt.target.files[0];
      resolve(file);
      document.body.removeChild(elem);
    };

    const onCancel = () => {
      document.body.removeChild(elem);
      document.body.removeEventListener("focus", onCancel);
      reject();
    };

    // cancel
    document.body.addEventListener("focus", onCancel);

    //
    if (elem && document.createEvent) {
      const evt = document.createEvent("MouseEvents");
      evt.initEvent("click", true, false);
      elem.dispatchEvent(evt);
    }
  });
