import { memo } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import type { MakuNodeData } from "./types";

export interface BaseNodeProps extends NodeProps {
  data: MakuNodeData;
  color?: string;
  icon?: string;
}

export const BaseNode = memo(({ data, color = "#6366f1", icon }: BaseNodeProps) => {
  const hasInputs = data.op !== "Input" && data.op !== "Constant";
  const hasOutputs = true;

  return (
    <div
      style={{
        padding: "10px 15px",
        borderRadius: "8px",
        border: `2px solid ${color}`,
        background: "white",
        minWidth: "150px",
        fontSize: "12px",
      }}
    >
      {hasInputs && <Handle type="target" position={Position.Top} style={{ background: color }} />}

      <div style={{ fontWeight: "bold", marginBottom: "4px", color }}>
        {icon && <span style={{ marginRight: "6px" }}>{icon}</span>}
        {data.op}
      </div>

      {data.label && <div style={{ fontSize: "10px", color: "#666" }}>{data.label}</div>}

      {/* Show attributes if present */}
      {"attrs" in data && data.attrs && (
        <div
          style={{
            fontSize: "10px",
            color: "#999",
            marginTop: "4px",
            maxWidth: "200px",
            overflow: "hidden",
            textOverflow: "ellipsis",
          }}
        >
          {JSON.stringify(data.attrs, null, 0).slice(0, 50)}...
        </div>
      )}

      {hasOutputs && (
        <Handle type="source" position={Position.Bottom} style={{ background: color }} />
      )}
    </div>
  );
});

BaseNode.displayName = "BaseNode";
