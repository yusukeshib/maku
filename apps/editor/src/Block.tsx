import { Property } from './Property'
import css from './Block.module.css'

export function Block() {
  return (
    <div className={css.container} style={{ left: 100, top: 200 }}>
      <div className={css.header}>Math</div>
      <Property />
      <Property />
      <Property />
      <Property />
    </div>
  )
}
