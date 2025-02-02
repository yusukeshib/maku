import { useStore, createStore, ExtractState } from 'zustand'
import invariant from 'tiny-invariant';
import { combine } from 'zustand/middleware';
import { produce } from 'immer'

interface AppProps {
  project: Project;
}

export type AppStore = ReturnType<typeof createAppStore>

const createAppStore = () => {
  const initial: AppProps = {
    project: { blocks: [], nodes: {} }
  }
  return createStore(combine(initial, (set, ) => ({
    move: (id: NodeId, pos: Point) => {
      set((state) => produce(state, state => {
        const block = state.project.nodes[id];
        invariant(block.type === 'block');

        block.pos = pos;
      }));
    }
  })));
}

export const appStore = createAppStore();

type AppState = ExtractState<typeof appStore>;

export function useAppStore<R>(selector: (state: AppState) => R): R {
  return useStore(appStore, selector);
}

interface Point {
  x: number;
  y: number;
}

type NodeId = string;
type Node = Block|Property;

interface Block {
  type: 'block';
  pos: Point;
  properties: NodeId[];
}

interface Property {
  type: 'property';
  key: string;
  value: number;
}

interface Project {
  blocks: NodeId[];
  nodes: Record<NodeId, Node>;
}
