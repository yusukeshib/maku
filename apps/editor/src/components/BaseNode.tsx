import { memo } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import type { MakuNodeData } from "./types";
import * as styles from "./BaseNode.css";

export interface BaseNodeProps extends NodeProps {
  data: MakuNodeData;
}

export const BaseNode = memo(({ data }: BaseNodeProps) => {
  const hasInputs = data.op !== "Input" && data.op !== "Constant";
  const hasOutputs = true;

  return (
    <div className={styles.nodeContainer}>
      {hasInputs && <Handle type="target" position={Position.Top} className={styles.handle} />}

      <div className={styles.nodeHeader}>
        <span className={styles.nodeTitle}>{data.op}</span>
      </div>

      {data.label && <div className={styles.nodeLabel}>{data.label}</div>}

      {/* Show attributes if present */}
      {"attrs" in data && data.attrs && (
        <div className={styles.nodeAttrs}>{JSON.stringify(data.attrs, null, 0)}</div>
      )}

      {hasOutputs && <Handle type="source" position={Position.Bottom} className={styles.handle} />}
    </div>
  );
});

BaseNode.displayName = "BaseNode";
