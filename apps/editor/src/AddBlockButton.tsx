import { useState } from 'react';
import css from './AddBlockButton.module.css'
import { blockDefs, isBlockType } from './project';
import { useAppStore } from './store'

export function AddBlockButton() {
  const [value, setValue] = useState('');
  const addBlock = useAppStore(s => s.addBlock);
  const handleSelect = (evt: React.ChangeEvent<HTMLSelectElement>) => {
    if(isBlockType(evt.target.value)) {
      addBlock(evt.target.value);
      setValue('');
    }
  }
  return (
    <select value={value} onChange={handleSelect} className={css.button}>
        <option value=''>(select)</option>
      {blockDefs.map(def => (
        <option key={def.type} value={def.type}>{def.type}</option>
      ))}
    </select>
  )
}


