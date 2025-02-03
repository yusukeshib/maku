import { type KeyboardEvent, type ChangeEvent, useState } from 'react';
import css from './NumberInput.module.css'

const precision = 2;
const base = Math.pow(10, precision);

function format(v:number): string {
  const negative = v < 0;
  const p = Math.abs(v);
  const prime = Math.floor(p);
  const frac = `${Math.round((p - prime) * base)}`.padStart(precision, '0')
  return `${negative ? '-' : ''}${prime}.${frac}`;
}

function parse(str: string): number {
  return parseFloat(str)
}

export function NumberInput({ value, onChange }: { value: number; onChange: (value: number) => void; }) {
  const [internalValue, setInternalValue] = useState(format(value))

  const handleChange = (evt: ChangeEvent<HTMLInputElement>) => {
    setInternalValue(evt.target.value)
  }

  const handleBlur = () => {
    const parsed = parse(internalValue)
    if(!isNaN(parsed)) {
      setInternalValue(format(parsed))
      onChange(parsed);
    } else {
      setInternalValue(format(value))
    }
  }

  function handleKeyDown(evt: KeyboardEvent<HTMLInputElement>) {
    switch(evt.code) {
    case 'Enter':
    case 'Escape':
      {
        evt.currentTarget.blur();
      }
      break;
    case 'ArrowUp':
      {
        const newValue = value + (evt.shiftKey ? 10 : 1)/base;
        setInternalValue(format(newValue))
        onChange(newValue);
      }
      break
    case 'ArrowDown':
      {
        const newValue = value - (evt.shiftKey ? 10 : 1)/base;
        setInternalValue(format(newValue))
        onChange(newValue);
      }
      break
    } 
  }

  return (
    <input onKeyDown={handleKeyDown} onBlur={handleBlur} className={css.input} type='text' value={internalValue} onChange={handleChange} />
  )
}
