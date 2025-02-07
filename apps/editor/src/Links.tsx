import { memo, useState, useEffect } from "react";
import { NodeId } from "./project";
import { useAppStore } from "./store";
import invariant from "tiny-invariant";
import css from './Links.module.css'
import { useDragLayer } from 'react-dnd'

const BLOCK_WIDTH = 200;
const PROPERTY_HEIGHT = 24;

export function Links() {
  const [width, height] = useWindowSize();
  const links = useAppStore(s => {
    const links: NodeId[] = [];
    for(const id of s.project.blocks) {
      const block = s.project.nodes[id];
      invariant(block?.ty === 'block');
      for(const id of block.properties) {
        const prop = s.project.nodes[id];
        invariant(prop?.ty === 'property');
        if(prop.link) {
          links.push(id);
        }
      }
    }
    return links;
  });

  return (
    <svg className={css.svg} xmlns="http://www.w3.org/2000/svg" viewBox={`0 0 ${width} ${height}`}>
      {links.map((id) => (
        <Link key={id} propId={id} />
      ))}
      <DraggingLink/>
    </svg>
  )
}

const Link = memo(function Link({ propId: toId }: { propId: NodeId; }) {
  const { x1, y1, x2, y2 } = useAppStore(s => {
    const toProp = s.project.nodes[toId]
    invariant(toProp?.ty === 'property');
    const toBlock = s.project.nodes[toProp.blockId]
    invariant(toBlock?.ty === 'block');
    const toPropIndex = toBlock.properties.indexOf(toId);

    const toDelta = s.editing.dragging?.blockId === toProp.blockId ? s.editing.dragging.delta : { x: 0, y: 0 };

    const fromId = toProp.link;
    invariant(fromId);

    const fromProp = s.project.nodes[fromId]
    invariant(fromProp?.ty === 'property');
    const fromBlock = s.project.nodes[fromProp.blockId]
    invariant(fromBlock?.ty === 'block');
    const fromPropIndex = fromBlock.properties.indexOf(fromId);

    const fromDelta = s.editing.dragging?.blockId === fromProp.blockId ? s.editing.dragging.delta : { x: 0, y: 0 };

    return {
      x1: fromBlock.pos.x + fromDelta.x + BLOCK_WIDTH,
      y1: fromBlock.pos.y + fromDelta.y + (fromPropIndex + 1.5) * PROPERTY_HEIGHT,
      x2: toBlock.pos.x + toDelta.x,
      y2: toBlock.pos.y + toDelta.y + (toPropIndex + 1.5) * PROPERTY_HEIGHT,
    }
  });
  return <line className={css.line} x1={x1} y1={y1} x2={x2} y2={y2} />
})

function DraggingLink() {
  const { isDragging, start ,end } = useDragLayer(
    monitor => ({
      isDragging: monitor.isDragging(),
      start: monitor.getInitialClientOffset(),
      end: monitor.getClientOffset(),
    })
  )

  if(!isDragging || !start || !end) return null;

  return <line className={css.line} x1={start.x} y1={start.y} x2={end.x} y2={end.y} />
}

function useWindowSize() {
  const [dimensions, setDimensions] = useState<[number, number]>([0,0]);
  useEffect(() => {
    function update() {
      setDimensions([window.innerWidth, window.innerHeight]);
    }
    window.addEventListener('resize',update);
    update();
    return () => {
      window.removeEventListener('resize', update);
    }
  },[]);

  return dimensions;
}
