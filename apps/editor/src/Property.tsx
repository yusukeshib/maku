import { memo } from 'react'
import css from './Property.module.css'
import { NumberInput } from './NumberInput'
import { getPropDef, NodeId, ValueType } from './project';
import { getAppStore, useAppStore } from './store';
import invariant from 'tiny-invariant';
import { useDrag, useDrop } from 'react-dnd'

export const Property = memo(function Property({ propId }: { propId: NodeId }) {
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

  return (
    <div className={css.container}>
      <div className={css.label}>{prop.key}</div>
      {def.cat === 'input' && <Input valueType={def.defaultValue.type} propId={propId} /> }
      {def.cat === 'output' && <Output valueType={def.defaultValue.type} propId={propId} />}
      {prop.link && <UnlinkButton propId={propId} /> }
      {prop.value.type === 'number' && <NumberValue disabled={def.cat === 'output'} propId={propId} />}
      {prop.value.type === 'string' && <StringValue disabled={def.cat === 'output'} propId={propId} />}
    </div>
  )
})

function NumberValue({ disabled, propId }: { disabled: boolean; propId: NodeId }) {
  const setValue = useAppStore(s => s.setPropertyValue);
  const value = useAppStore(s => {
    const prop = s.project.nodes[propId]
    invariant(prop?.ty=== 'property', 'invalid-node-type')
    invariant(prop.value.type === 'number');
    return prop.value.value;
  })

  const handleChange = (value: number) => {
    setValue(propId, { type: 'number', value });
  }

  return (
      <NumberInput step={0.1} disabled={disabled} value={value} onChange={handleChange} />
  )
}

function StringValue({ disabled, propId }: { disabled: boolean; propId: NodeId }) {
  const setValue = useAppStore(s => s.setPropertyValue);
  const value = useAppStore(s => {
    const prop = s.project.nodes[propId]
    invariant(prop?.ty=== 'property', 'invalid-node-type')
    invariant(prop.value.type === 'string');
    return prop.value.value;
  })

  const handleChange = (evt:React.ChangeEvent<HTMLInputElement>) => {
    setValue(propId, { type: 'string', value: evt.target.value });
  }

  return (
      <input type='text' disabled={disabled} value={value} onChange={handleChange} />
  )
}

function UnlinkButton({ propId }: { propId: NodeId }) {
  const handleClick = () => {
    getAppStore().unlinkProperty(propId)
  }

  return (
    <div onClick={handleClick} className={css.unlinkButton} />
  )
}

function Input({ valueType, propId }: { valueType: ValueType; propId: NodeId }) {
  const [{ canDrop: _, isOver }, ref] = useDrop(() => ({
    accept: 'property',
    drop: () => ({ propId }),
    collect: (monitor) => ({
      isOver: monitor.isOver(),
      canDrop: monitor.canDrop(),
    }),
  }))
  return (
    <div data-over={isOver} data-type={valueType} ref={ref} className={css.dotIn} />
  )
}

function Output({ valueType, propId }: { valueType: ValueType; propId: NodeId }) {
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
    <div data-dragging={isDragging} data-type={valueType} ref={ref} className={css.dotOut} />
  )
}
