import { DndProvider } from 'react-dnd'
import { HTML5Backend } from 'react-dnd-html5-backend'

import { AddBlockButton } from './AddBlockButton'
import css from './App.module.css'
import { Block } from './Block'
import { useAppStore } from './store'
import { Links } from './Links'

export function App() {
  return (
    <DndProvider backend={HTML5Backend}>
      <div className={css.container}>
        <AddBlockButton />
        <Blocks />
        <Links/>
      </div>
    </DndProvider>
  )
}

function Blocks() {
  const blocks = useAppStore(s => s.project.blocks);
  return (
    <>
      {blocks.map(id => (
        <Block key={id} blockId={id}/>
      ))}
    </>
  )
}

