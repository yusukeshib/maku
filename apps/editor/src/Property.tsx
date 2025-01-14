import { type ChangeEvent, useState } from 'react';
import css from './Property.module.css'

export function Property({ label }: { label: string }) {
  const [value, setValue] = useState(0)
  const handleChange = (evt: ChangeEvent<HTMLInputElement>) => {
    setValue(parseFloat(evt.target.value))
  }
  return (
    <div className={css.container}>
      <div className={css.label}>{label}</div>
      <input type='number' value={value} onChange={handleChange} />
    </div>
  )

}


