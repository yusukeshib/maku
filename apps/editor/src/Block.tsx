import { Property } from './Property'
import css from './Block.module.css'
import { Point, type NodeId } from './project'
import { getAppStore, useAppStore } from './store'
import invariant from 'tiny-invariant'
import { RefObject, useEffect, useRef, useState } from 'react'

export function Block({ blockId }: { blockId: NodeId }) {
  const block = useAppStore(s => {
    const node = s.project.nodes[blockId]
    invariant(node?.type === 'block', 'invalid-node-type')
    return node;
  })

  const handleClickClose = () =>  {
    getAppStore().removeBlock(blockId);
  }

  const ref = useRef<HTMLDivElement>(null);
  const [dragging,delta] = useDrag(blockId, ref);

  return (
    <div className={css.container} style={{
      left: block.pos.x + delta.x,
      top: block.pos.y + delta.y,
    }}>
      <div ref={ref} data-dragging={dragging} className={css.header}>
        <span className={css.name}>{block.blockType}</span>
        <span className={css.close} onClick={handleClickClose}>x</span>
      </div>
      {block.properties.map(id => (
        <Property propId={id} key={id} />
      ))}
    </div>
  )
}

function useDrag(blockId: NodeId, ref: RefObject<HTMLDivElement>): [boolean, Point] {
  const [delta, setDelta] = useState<Point>({ x: 0, y: 0 });
  const [dragging, setDragging] = useState(false);

  useEffect(() => {
    const elem = ref.current;
    if(!elem) return;

    let start: Point|null = null;

    function handleDown(evt: PointerEvent) {
      document.body.addEventListener('pointermove', handleMove);
      document.body.addEventListener('pointercancel', handleCancel);
      document.body.addEventListener('pointerup', handleUp);
      start = { x: evt.clientX, y: evt.clientY}
      setDragging(true);
    }

    function handleMove(evt: PointerEvent) {
      invariant(start, '');
      const p = { x: evt.clientX, y: evt.clientY}
      setDelta({ x: p.x-start.x, y: p.y-start.y });
    }

    function handleCancel() {
      document.body.removeEventListener('pointermove', handleMove);
      document.body.removeEventListener('pointerup', handleUp);
      setDelta({x: 0, y: 0});
      setDragging(false);
    }

    function handleUp(evt: PointerEvent) {
      invariant(start, '');
      const p = { x: evt.clientX, y: evt.clientY}
      const delta = { x: p.x-start.x, y: p.y-start.y };
      getAppStore().moveBlock(blockId, delta);

      handleCancel();
    }

    elem.addEventListener('pointerdown', handleDown);
    return () => {
      elem.removeEventListener('pointerdown', handleDown);
    }
  }, [ref]);

  return [dragging, delta];
}
