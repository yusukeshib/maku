import { create } from "zustand";

interface TestStoreState {
  count: number;
  increment: () => void;
  decrement: () => void;
}

export const useTestStore = create<TestStoreState>(set => ({
  count: 0,
  increment: () => set(state => ({ count: state.count + 1 })),
  decrement: () => set(state => ({ count: state.count - 1 })),
}));
