import { Property } from './Property'
import css from './Block.module.css'
import { type NodeId } from './project'
import { useAppStore } from './store'
import invariant from 'tiny-invariant'

export function Block({ blockId }: { blockId: NodeId }) {
  const block = useAppStore(s => {
    const node = s.project.nodes[blockId]
    invariant(node?.type === 'block', 'invalid-node-type')
    return node;
  })
  return (
    <div className={css.container} style={{ left: block.pos.x, top: block.pos.y }}>
      <div className={css.header}>{block.type}</div>
      {block.properties.map(id => (
      <Property propId={id} key={id} />
      ))}
    </div>
  )
}
