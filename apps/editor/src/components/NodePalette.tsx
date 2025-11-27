import { OP_CONFIG, type MakuOpType } from "./types";
import * as styles from "./NodePalette.css";

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
    <div className={styles.paletteContainer}>
      {Object.entries(categories).map(([category, ops]) => (
        <div key={category} className={styles.categoryContainer}>
          <div className={styles.categoryTitle}>{category}</div>

          <div className={styles.buttonList}>
            {ops.map(({ opType }) => (
              <button key={opType} onClick={() => onAddNode(opType)} className={styles.nodeButton}>
                <span className={styles.nodeButtonText}>{opType}</span>
              </button>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
