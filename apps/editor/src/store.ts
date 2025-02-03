import { useStore, createStore, ExtractState } from 'zustand'
import invariant from 'tiny-invariant';
import { combine } from 'zustand/middleware';
import { produce } from 'immer'
import { blockDefMap, type Block, type BlockType, type Project, defaultProject, type NodeId, type Point, Property } from './project'

interface AppProps {
  project: Project;
}

export type AppStore = ReturnType<typeof createAppStore>

const createAppStore = () => {
  const initial: AppProps = {
    project: defaultProject,
  }
  return createStore(combine(initial, (set, ) => ({
    addBlock: (type: BlockType) => {
      const def = blockDefMap[type]
      invariant(def, 'invalid-block-type');

      set((state) => produce(state, state => {
        const block: Block = {
          type: 'block',
          pos: { x:0,y:0},
          properties: [],
        }
        for(const p of def.props) {
          const prop: Property = {
            type: 'property',
            key: p.key,
            value: p.defaultValue,
          }
          const propId = state.project.nodes.length;
          state.project.nodes[propId] = prop;
          block.properties.push(propId);
        }
        const blockId = state.project.nodes.length;
        state.project.nodes[blockId] = block;
        state.project.blocks.push(blockId);
      }));
    },
    removeBlock: (id: NodeId) => {
      set((state) => produce(state, state => {
        const index = state.project.blocks.indexOf(id);
        invariant(index >= 0, 'invalid-block-id');
        state.project.blocks.splice(index, 1);
        state.project.nodes[id] = null;
      }));
    },
    moveBlock: (id: NodeId, pos: Point) => {
      set((state) => produce(state, state => {
        const block = state.project.nodes[id];
        invariant(block?.type === 'block', 'invalid-block-id');

        block.pos = pos;
      }));
    },
    setPropertyValue: (id: NodeId, value: number) => {
      set((state) => produce(state, state => {
        const prop= state.project.nodes[id];
        invariant(prop?.type === 'property', 'invalid-property-id');
        prop.value = value;
      }));
    },
  })));
}

export const appStore = createAppStore();

type AppState = ExtractState<typeof appStore>;

export function useAppStore<R>(selector: (state: AppState) => R): R {
  return useStore(appStore, selector);
}

