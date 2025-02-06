import { memo } from 'react'
import css from './Property.module.css'
import { NumberInput } from './NumberInput'
import { getPropDef, NodeId } from './project';
import { getAppStore, useAppStore } from './store';
import invariant from 'tiny-invariant';
import { useDrag, useDrop } from 'react-dnd'

export const Property = memo(function Property({ propId }: { propId: NodeId }) {
  const setValue = useAppStore(s => s.setPropertyValue);
  const prop = useAppStore(s => {
    const prop = s.project.nodes[propId]
    invariant(prop?.ty=== 'property', 'invalid-node-type')
    return prop
  })
  const block = useAppStore(s => {
    const block = s.project.nodes[prop.blockId]
    invariant(block?.ty=== 'block', 'invalid-node-type')
    return block
  })

  const def = getPropDef(block.type, prop.key)

  const handleChange = (value: number) => {
    setValue(propId, value);
  }

  return (
    <div className={css.container}>
      <div className={css.label}>{prop.key}</div>
      {def.cat === 'input' && <Input propId={propId} /> }
      {def.cat === 'output' && <Output propId={propId} />}
      {prop.link && <UnlinkButton propId={propId} /> }
      <NumberInput disabled={def.cat === 'output'} value={prop.value} onChange={handleChange} />
    </div>
  )
})

function UnlinkButton({ propId }: { propId: NodeId }) {
  const handleClick = () => {
    getAppStore().unlinkProperty(propId)
  }

  return (
    <div onClick={handleClick} className={css.unlinkButton} />
  )
}

function Input({ propId }: { propId: NodeId }) {
  const [{ canDrop: _, isOver }, ref] = useDrop(() => ({
    accept: 'property',
    drop: () => ({ propId }),
    collect: (monitor) => ({
      isOver: monitor.isOver(),
      canDrop: monitor.canDrop(),
    }),
  }))
  return (
    <div data-over={isOver} ref={ref} className={css.dotIn} />
  )
}

function Output({ propId }: { propId: NodeId }) {
  const [{ isDragging }, ref] = useDrag(() => ({
    type: 'property',
    item: { propId, },
    end: (from , monitor) => {
      const to = monitor.getDropResult<{ propId: NodeId }>()
      if (from && to) {
        getAppStore().linkProperties(from.propId, to.propId);
      }
    },
    collect: (monitor) => ({
      isDragging: monitor.isDragging(),
      handlerId: monitor.getHandlerId(),
    }),
  }))

  return (
    <div data-dragging={isDragging} ref={ref} className={css.dotOut} />
  )
}
