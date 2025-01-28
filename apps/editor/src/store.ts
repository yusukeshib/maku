import { useStore, createStore } from 'zustand'

interface AppProps {
  count: number
}

interface AppState extends AppProps {
  plus: () => void
  minus: () => void
}

export type AppStore = ReturnType<typeof createAppStore>

const createAppStore = () => {
  const initial: AppProps = {
    count: 0,
  }
  return createStore<AppState>()((set) => ({
    ...initial,
    plus: () => set((state) => ({ count: state.count + 1 })),
    minus: () => set((state) => ({ count: state.count -1 })),
  }))
}

export const editorStore = createAppStore();

export function useAppStore<R>(selector: (state: AppState) => R): R {
  return useStore(editorStore, selector);
}
