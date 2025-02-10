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
  value: Value;
  link: NodeId|null;
}

export interface Project {
  blocks: NodeId[];
  nodes: (Node|null)[];
}

export const defaultProject: Project = {
  blocks: [],
  nodes: [],
}

export type BlockType = 'add'|'multiply'|'concat';

export interface BlockDef {
  type: BlockType;
  props: PropDef[];
}

export interface PropDef {
  key: string;
  cat: 'input'|'output';
  defaultValue: Value;
}

export type Value = 
| { type: 'number'; value: number }
| { type: 'string'; value: string }

export function isBlockType(ty: string): ty is BlockType {
  return !!blockDefs.find(def => def.type === ty);
}

export const blockDefs: BlockDef[] = [
 {
   type: 'add',
   props: [
     { key: 'a', defaultValue: { type: 'number', value: 1 }, cat: 'input', },
     { key: 'b', defaultValue: { type: 'number', value: 2 }, cat: 'input', },
     { key: 'c', defaultValue: { type: 'number', value: 3 }, cat: 'output', },
   ]
 },
 {
   type: 'multiply',
   props: [
     { key: 'a', defaultValue: { type: 'number', value: 1 }, cat: 'input', },
     { key: 'b', defaultValue: { type: 'number', value: 2 }, cat: 'input', },
     { key: 'c', defaultValue: { type: 'number', value: 3 }, cat: 'output', },
   ]
 },
 {
   type: 'concat',
   props: [
     { key: 'a', defaultValue: { type: 'string', value: 'hello' }, cat: 'input', },
     { key: 'b', defaultValue: { type: 'string', value: 'yusuke' }, cat: 'input', },
     { key: 'c', defaultValue: { type: 'string', value: 'helloyusuke' }, cat: 'output', },
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

