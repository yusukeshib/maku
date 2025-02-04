import { memo } from 'react'
import css from './Property.module.css'
import { NumberInput } from './NumberInput'
import { NodeId } from './project';
import { useAppStore } from './store';
import invariant from 'tiny-invariant';

export const Property = memo(function Property({ propId }: { propId: NodeId }) {
  const setValue = useAppStore(s => s.setPropertyValue);
  const prop = useAppStore(s => {
    const prop = s.project.nodes[propId]
    invariant(prop?.ty=== 'property', 'invalid-node-type')
    // const block = s.project.nodes[prop.blockId]
    // invariant(block?.ty=== 'block', 'invalid-node-type')
    return prop;
  })

  // const def = getPropDef(block.type, prop.key)

  const handleChange = (value: number) => {
    setValue(propId, value);
  }

  return (
    <div className={css.container}>
      <div className={css.label}>{prop.key}</div>
      <div className={css.dotIn} /> 
      <div className={css.dotOut} />
      <NumberInput value={prop.value} onChange={handleChange} />
    </div>
  )
})


