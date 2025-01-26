import { type ChangeEvent, useState } from 'react';
import css from './Property.module.css'

export function Property() {
  const [value, setValue] = useState(0)
  const handleChange = (evt: ChangeEvent<HTMLInputElement>) => {
    setValue(parseFloat(evt.target.value))
  }
  return (
    <div className={css.container}>
      <div className={css.label}>Opacity</div>
      <div className={css.dotIn} />
      <div className={css.dotOut} />
      <input className={css.input} type='text' value={value} onChange={handleChange} />
    </div>
  )

}


