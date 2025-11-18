import { style } from '@vanilla-extract/css'

export const container = style({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  justifyContent: 'center',
  minHeight: '100vh',
  padding: '2rem',
})

export const title = style({
  fontSize: '3rem',
  fontWeight: 'bold',
  color: '#646cff',
  marginBottom: '2rem',
})

export const card = style({
  padding: '2rem',
  backgroundColor: '#f9f9f9',
  borderRadius: '8px',
  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
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
