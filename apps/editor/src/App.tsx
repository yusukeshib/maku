import { AddBlockButton } from './AddBlockButton'
import css from './App.module.css'
import { Block } from './Block'
import { useAppStore } from './store'

export function App() {
  const blocks = useAppStore(s => s.project.blocks);
  return (
    <div className={css.container}>
      <AddBlockButton />
      {blocks.map(id => (
      <Block key={id} blockId={id}/>
      ))}
    </div>
  )
}

