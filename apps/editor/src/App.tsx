import { MakuProvider } from './maku/provider'
import { Editor } from './Editor'

function App() {
  return (
    <MakuProvider>
      <Editor />
    </MakuProvider>
  )
}

export default App
