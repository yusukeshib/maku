import { useState } from 'react';
import css from './Property.module.css'
import { NumberInput } from './NumberInput'

export function Property() {
  const [value, setValue] = useState(0)
  const handleChange = (value: number) => {
    setValue(value)
  }
  return (
    <div className={css.container}>
      <div className={css.label}>Opacity</div>
      <div className={css.dotIn} />
      <div className={css.dotOut} />
      <NumberInput value={value} onChange={handleChange} />
    </div>
  )

}


