import { type ChangeEvent, useEffect, useRef, useState } from 'react';
import css from './NumberInput.module.css'

function format(v:number): string {
  const prime = Math.floor(v);
  const frac = Math.round((v - prime) * 100) / 100;
  return `${prime}.${frac}`;
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


  const ref = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const elem = ref.current;
    if(!elem) return;

    function handleKeyDown(evt: KeyboardEvent) {
      if(evt.code === 'Enter' || evt.code === 'Escape') {
        elem?.blur();
      }
    }

    elem.addEventListener('keydown', handleKeyDown)
    return () => {
      elem.removeEventListener('keydown', handleKeyDown)
    }
  }, []);


  return (
    <input ref={ref} onBlur={handleBlur} className={css.input} type='text' value={internalValue} onChange={handleChange} />
  )
}
