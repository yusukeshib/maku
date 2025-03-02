import { memo } from "react";
import { Property } from "./Property";
import css from "./Block.module.css";
import { Point, type NodeId } from "./project";
import { getAppStore, useAppStore } from "./store";
import invariant from "tiny-invariant";
import { RefObject, useEffect, useRef } from "react";

export const Block = memo(function Block({ blockId }: { blockId: NodeId }) {
  const block = useAppStore((s) => {
    const node = s.project.nodes[blockId];
    invariant(node?.ty === "block", "invalid-node-type");
    return node;
  });

  const handleClickClose = () => {
    getAppStore().removeBlock(blockId);
  };

  const ref = useRef<HTMLDivElement>(null);
  const [dragging, delta] = useDrag(blockId, ref);

  return (
    <div
      className={css.container}
      style={{
        left: block.pos.x + delta.x,
        top: block.pos.y + delta.y,
      }}
    >
      <div ref={ref} data-dragging={dragging} className={css.header}>
        <span className={css.name}>{block.type}</span>
        <span className={css.close} onClick={handleClickClose}>
          x
        </span>
      </div>
      {block.properties.map((id) => (
        <Property propId={id} key={id} />
      ))}
    </div>
  );
});

function useDrag(
  blockId: NodeId,
  ref: RefObject<HTMLDivElement>,
): [boolean, Point] {
  const delta = useAppStore((s) => {
    if (s.editing.dragging?.blockId === blockId) {
      return s.editing.dragging.delta;
    } else {
      return { x: 0, y: 0 };
    }
  });
  const dragging = useAppStore((s) => s.editing.dragging?.blockId === blockId);

  useEffect(() => {
    const elem = ref.current;
    if (!elem) return;

    let start: Point | null = null;

    function handleDown(evt: PointerEvent) {
      evt.preventDefault();
      document.body.addEventListener("pointermove", handleMove);
      document.body.addEventListener("pointerup", handleUp);
      start = { x: evt.clientX, y: evt.clientY };
      getAppStore().moveBlock.start(blockId);
    }

    function handleMove(evt: PointerEvent) {
      evt.preventDefault();
      invariant(start, "");
      const p = { x: evt.clientX, y: evt.clientY };
      const delta = {
        x: Math.round(p.x - start.x),
        y: Math.round(p.y - start.y),
      };
      getAppStore().moveBlock.move(blockId, delta);
    }

    function handleUp(evt: PointerEvent) {
      evt.preventDefault();
      if (start) {
        getAppStore().moveBlock.commit();
        document.body.removeEventListener("pointermove", handleMove);
        document.body.removeEventListener("pointerup", handleUp);
        start = null;
      }
    }

    elem.addEventListener("pointerdown", handleDown);
    return () => {
      elem.removeEventListener("pointerdown", handleDown);
    };
  }, [blockId, ref]);

  return [dragging, delta];
}
