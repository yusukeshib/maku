import { type KeyboardEvent, type ChangeEvent, useState, useRef, useEffect } from 'react';
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

export function NumberInput({ step, disabled, value, onChange }: { step: number; disabled?: boolean; value: number; onChange: (value: number) => void; }) {
  const ref=  useRef<HTMLInputElement>(null);
  const [focus, setFocus] = useState(false);
  const [internalValue, setInternalValue] = useState(format(value))

  const handleChange = (evt: ChangeEvent<HTMLInputElement>) => {
    setInternalValue(evt.target.value)
  }

  const handleDraggableClick = () => {
    ref.current?.focus();
  }

  const handleFocus = () => {
    setFocus(true);
    ref.current?.select();
  }

  const handleBlur = () => {
    setFocus(false);
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

  useEffect(() => {
    setInternalValue(format(value))
  }, [value]);

  return (
    <span className={css.container}>
      <input
        ref={ref}
        disabled={disabled}
        onKeyDown={handleKeyDown}
        onFocus={handleFocus}
        onBlur={handleBlur}
        className={css.input}
        type='text'
        value={internalValue}
        onChange={handleChange}
      />
      {!focus && !disabled && <Draggable step={step} onClick={handleDraggableClick} value={value} onChange={onChange}/>}
    </span>
  )
}

function Draggable({ step, value, onClick, onChange }: { step: number; value: number; onClick: React.EventHandler<React.MouseEvent<HTMLDivElement>>; onChange: (value: number) => void; }) {
  const ref = useRef<HTMLDivElement>(null);
  const refChange = useRef(onChange);
  const refValue = useRef(value);

  refChange.current = onChange;
  refValue.current = value;

  useEffect(() => {
    const div = ref.current;
    if(!div) return;

    let startX= 0;
    let startValue = 0;

    function handleMove(evt: PointerEvent) {
      evt.stopPropagation();
      evt.preventDefault();
      const current = evt.clientX;
      const delta = (current - startX) * step
      refChange.current(startValue + delta);
    }

    function handleUp(evt: PointerEvent) {
      evt.stopPropagation();
      evt.preventDefault();
      document.body.removeEventListener('pointermove', handleMove);
      document.body.removeEventListener('pointerup', handleUp);
    }

    function handleDown(evt: PointerEvent) {
      evt.stopPropagation();
      evt.preventDefault();
      document.body.addEventListener('pointermove', handleMove);
      document.body.addEventListener('pointerup', handleUp);
      startX = evt.clientX;
      startValue = value;
    }

    div.addEventListener('pointerdown', handleDown);
    return () => {
      div.removeEventListener('pointerdown', handleDown);
    }
  }, [step]);

  return (<div ref={ref} onClick={onClick} className={css.draggable}/>)
}
