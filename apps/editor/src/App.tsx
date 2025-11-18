import { useState } from 'react'
import { runExample } from './test'

function App() {
  const [count, setCount] = useState<number>(0)

  const handleClick = () => {
    setCount((count) => count + 1)
    runExample()
  }

  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={handleClick}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
