import { Property } from './Property'
import css from './Block.module.css'
import { type NodeId } from './project'
import { getAppStore, useAppStore } from './store'
import invariant from 'tiny-invariant'

export function Block({ blockId }: { blockId: NodeId }) {
  const block = useAppStore(s => {
    const node = s.project.nodes[blockId]
    invariant(node?.type === 'block', 'invalid-node-type')
    return node;
  })

  const handleClickClose = () =>  {
    getAppStore().removeBlock(blockId);
  }

  return (
    <div className={css.container} style={{ left: block.pos.x, top: block.pos.y }}>
      <div className={css.header}>
        <span className={css.name}>{block.type}</span>
        <span className={css.close} onClick={handleClickClose}>x</span>
      </div>
      {block.properties.map(id => (
      <Property propId={id} key={id} />
      ))}
    </div>
  )
}
