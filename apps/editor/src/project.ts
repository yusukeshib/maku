import { keyBy } from 'lodash-es'
import invariant from 'tiny-invariant';

export interface Point {
  x: number;
  y: number;
}

export type NodeId = number;
export type Node = Block|Property;

export interface Block {
  ty: 'block';
  type: BlockType;
  pos: Point;
  properties: NodeId[];
}

export interface Property {
  ty: 'property';
  blockId: NodeId;
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

const blockDefMap = keyBy(blockDefs, 'type');

export function getBlockDef(type: BlockType) {
  return blockDefMap[type];
}

export function getPropDef(type: BlockType, key: string) {
  const block = getBlockDef(type);
  const prop = block.props.find(p => p.key === key);
  invariant(prop, 'invalid-prop-key');
  return prop;
}

