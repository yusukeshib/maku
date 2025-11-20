import { style } from '@vanilla-extract/css'

export const container = style({
  display: 'flex',
})

export const button = style({
  padding: '0.6rem 1.2rem',
  fontSize: '1rem',
  fontWeight: '500',
  backgroundColor: '#646cff',
  color: 'white',
  border: 'none',
  borderRadius: '8px',
  cursor: 'pointer',
  transition: 'background-color 0.25s',
  ':hover': {
    backgroundColor: '#535bf2',
  },
})
