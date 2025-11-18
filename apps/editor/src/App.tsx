import { useState } from 'react'
import { runExample } from './test'
import * as styles from './App.css'

function App() {
  const [count, setCount] = useState<number>(0)

  const handleClick = () => {
    setCount((count) => count + 1)
    runExample()
  }

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>Vite + React</h1>
      <div className={styles.card}>
        <button className={styles.button} onClick={handleClick}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
