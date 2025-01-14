import css from './App.module.css'
import { Block } from './Block'

export function App() {
  return (
    <div className={css.container}>
      <Block x={100} y={200} label='Math-1' />
      <Block x={400} y={250} label='Math-2' />
    </div>
  )
}

