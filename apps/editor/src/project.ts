import { keyBy } from 'lodash-es'

export interface Point {
  x: number;
  y: number;
}

export type NodeId = number;
export type Node = Block|Property;

export interface Block {
  type: 'block';
  pos: Point;
  properties: NodeId[];
}

export interface Property {
  type: 'property';
  key: string;
  value: number;
}

export interface Project {
  blocks: NodeId[];
  nodes: (Node|null)[];
}

export const defaultProject: Project = {
  blocks: [],
  nodes: [],
}

export type BlockType = 'add';

export interface BlockDef {
  type: BlockType;
  props: PropDef[];
}

export interface PropDef {
  key: string;
  cat: 'input'|'output';
  defaultValue: number;
}

const blockDefs: BlockDef[] = [
 {
   type: 'add',
   props: [
     { key: 'a', defaultValue: 1, cat: 'input', },
     { key: 'b', defaultValue: 2, cat: 'input', },
     { key: 'c', defaultValue: 3, cat: 'output', },
   ]
 }
];

export const blockDefMap = keyBy(blockDefs, 'type');
