import { MakuProvider } from './maku/provider'
import { NodeMain } from './NodeMain'

function App() {
  return (
    <MakuProvider>
      <NodeMain />
    </MakuProvider>
  )
}

export default App
