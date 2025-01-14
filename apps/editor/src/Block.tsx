import { Property } from './Property'
import css from './Block.module.css'

export function Block({ label, x, y }: { label: string; x: number; y: number; }) {
  return (
    <div className={css.container} style={{ left: x, top: y }}>
      <div className={css.header}>{label}</div>
      <Property label='A' />
      <Property label='A' />
      <Property label='A' />
      <Property label='A' />
    </div>
  )
}
