import { OP_CONFIG, type MakuOpType } from "../maku/nodes";

interface NodePaletteProps {
  onAddNode: (opType: MakuOpType) => void;
}

export function NodePalette({ onAddNode }: NodePaletteProps) {
  // Group operations by category
  const categories = Object.entries(OP_CONFIG).reduce(
    (acc, [opType, config]) => {
      if (!acc[config.category]) {
        acc[config.category] = [];
      }
      acc[config.category].push({ opType: opType as MakuOpType, config });
      return acc;
    },
    {} as Record<string, Array<{ opType: MakuOpType; config: (typeof OP_CONFIG)[MakuOpType] }>>
  );

  return (
    <div
      style={{
        position: "absolute",
        top: 10,
        left: 10,
        background: "white",
        borderRadius: "8px",
        boxShadow: "0 2px 10px rgba(0,0,0,0.1)",
        padding: "16px",
        maxWidth: "250px",
        maxHeight: "calc(100vh - 40px)",
        overflowY: "auto",
        zIndex: 10,
      }}
    >
      <h3 style={{ margin: "0 0 12px 0", fontSize: "14px", fontWeight: "bold" }}>Node Palette</h3>

      {Object.entries(categories).map(([category, ops]) => (
        <div key={category} style={{ marginBottom: "16px" }}>
          <div
            style={{
              fontSize: "11px",
              fontWeight: "600",
              color: "#666",
              marginBottom: "8px",
              textTransform: "uppercase",
              letterSpacing: "0.5px",
            }}
          >
            {category}
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
            {ops.map(({ opType, config }) => (
              <button
                key={opType}
                onClick={() => onAddNode(opType)}
                style={{
                  padding: "8px 12px",
                  border: `1px solid ${config.color}`,
                  borderRadius: "6px",
                  background: "white",
                  cursor: "pointer",
                  fontSize: "12px",
                  display: "flex",
                  alignItems: "center",
                  gap: "8px",
                  transition: "all 0.2s",
                }}
                onMouseEnter={e => {
                  e.currentTarget.style.background = `${config.color}10`;
                  e.currentTarget.style.transform = "translateX(2px)";
                }}
                onMouseLeave={e => {
                  e.currentTarget.style.background = "white";
                  e.currentTarget.style.transform = "translateX(0)";
                }}
              >
                <span style={{ fontSize: "16px" }}>{config.icon}</span>
                <span style={{ color: config.color, fontWeight: "500" }}>{opType}</span>
              </button>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
