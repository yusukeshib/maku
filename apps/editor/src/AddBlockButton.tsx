import css from './AddBlockButton.module.css'
import { useAppStore } from './store'

export function AddBlockButton() {
  const addBlock = useAppStore(s => s.addBlock);
  const handleClick = () => {
    addBlock('add');
  }
  return (
    <button onClick={handleClick} className={css.button}>Add</button>
  )
}


