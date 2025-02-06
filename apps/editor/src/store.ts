import { useStore, createStore, ExtractState } from 'zustand'
import invariant from 'tiny-invariant';
import { combine } from 'zustand/middleware';
import { produce } from 'immer'
import { getBlockDef, type Block, type BlockType, type Project, defaultProject, type NodeId, type Point, Property } from './project'
import { useShallow } from 'zustand/shallow';

interface AppProps {
  project: Project;
  editing: Editing;
}

interface Editing {
  dragging: { blockId: NodeId; delta: Point }|null;
  linking: { propId: NodeId; point: Point }|null;
}

export type AppStore = ReturnType<typeof createAppStore>

const createAppStore = () => {
  const initial: AppProps = {
    project: defaultProject,
    editing: {
      dragging: null,
      linking: null,
    }
  }
  return createStore(combine(initial, (set, _get) => ({
    addBlock: (type: BlockType) => {
      const def = getBlockDef(type);
      invariant(def, 'invalid-block-type');

      set((state) => produce(state, state => {
        const block: Block = {
          ty: 'block',
          type,
          pos: { x:0,y:0},
          properties: [],
        }
        const blockId = state.project.nodes.length;
        state.project.nodes[blockId] = block;
        state.project.blocks.push(blockId);

        for(const p of def.props) {
          const prop: Property = {
            ty: 'property',
            blockId,
            key: p.key,
            value: p.defaultValue,
            link: null,
          }
          const propId = state.project.nodes.length;
          state.project.nodes[propId] = prop;
          block.properties.push(propId);
        }
      }));
    },
    removeBlock: (id: NodeId) => {
      set((state) => produce(state, state => {
        const index = state.project.blocks.indexOf(id);
        invariant(index >= 0, 'invalid-block-id');

        const block = state.project.nodes[id];
        invariant(block?.ty === 'block', 'invalid-block-id');

        // Unset property links for these properties
        const props = new Set(block.properties)
        for(const node of state.project.nodes) {
          if(node?.ty !== 'property') continue
          if(node.link !== null && props.has(node.link)) {
            node.link = null;
          }
        }

        for(const id of block.properties) {
          state.project.nodes[id] = null;
        }

        state.project.blocks.splice(index, 1);
        state.project.nodes[id] = null;
      }));
    },
    moveBlock: {
      start: (id: NodeId) => {
        set((state) => produce(state, state => {
          state.editing.dragging = { blockId: id, delta: { x: 0, y: 0 }};
        }));
      },
      move: (_id: NodeId, delta: Point) => {
        set((state) => produce(state, state => {
          invariant(state.editing.dragging);
          state.editing.dragging.delta = delta;
        }));
      },
      commit: () => {
        set((state) => produce(state, state => {
          invariant(state.editing.dragging);
          const block = state.project.nodes[state.editing.dragging.blockId];
          invariant(block?.ty === 'block', 'invalid-block-id');

          block.pos.x += state.editing.dragging.delta.x;
          block.pos.y += state.editing.dragging.delta.y;

          state.editing.dragging = null;
        }));
      },
    },
    unlinkProperty: (outputId: NodeId) => {
      set((state) => produce(state, state => {
        const output = state.project.nodes[outputId];
        invariant(output?.ty === 'property', 'invalid-output-property-id');
        output.link = null;
      }));
    },
    linkProperties: (inputId: NodeId, outputId: NodeId) => {
      set((state) => produce(state, state => {
        const input = state.project.nodes[inputId];
        const output = state.project.nodes[outputId];
        invariant(input?.ty === 'property', 'invalid-input-property-id');
        invariant(output?.ty === 'property', 'invalid-output-property-id');

        output.link = inputId;
      }));
    },
    setPropertyValue: (id: NodeId, value: number) => {
      set((state) => produce(state, state => {
        const prop= state.project.nodes[id];
        invariant(prop?.ty === 'property', 'invalid-property-id');
        prop.value = value;
      }));
    },
  })));
}

export const appStore = createAppStore();

type AppState = ExtractState<typeof appStore>;

export function useAppStore<R>(selector: (state: AppState) => R): R {
  return useStore(appStore, useShallow(selector));
}

export function getAppStore() {
  return appStore.getState();
}

