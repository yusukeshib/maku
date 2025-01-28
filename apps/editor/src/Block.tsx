import { Property } from './Property'
import css from './Block.module.css'
import { useAppStore } from './store'

export function Block() {
  const _count = useAppStore(s => s.count);
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
