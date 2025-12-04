// Domain types for SketchDDD visual builder

export type NodeKind = 'entity' | 'value' | 'enum' | 'aggregate' | 'context';

export interface Field {
  id: string;
  name: string;
  type: string;
  optional: boolean;
}

export interface EnumVariant {
  id: string;
  name: string;
  payload?: string;
}

export interface EntityNode {
  kind: 'entity';
  name: string;
  fields: Field[];
}

export interface ValueNode {
  kind: 'value';
  name: string;
  fields: Field[];
}

export interface EnumNode {
  kind: 'enum';
  name: string;
  variants: EnumVariant[];
}

export interface AggregateNode {
  kind: 'aggregate';
  name: string;
  rootId: string;
  memberIds: string[];
  invariants: string[];
}

export interface ContextNode {
  kind: 'context';
  name: string;
}

export type DomainNode = EntityNode | ValueNode | EnumNode | AggregateNode | ContextNode;

export interface Morphism {
  id: string;
  name: string;
  sourceId: string;
  targetId: string;
  cardinality: 'one' | 'many' | 'optional';
}

export interface ContextMap {
  id: string;
  name: string;
  sourceContextId: string;
  targetContextId: string;
  pattern: ContextMapPattern;
  mappings: ObjectMapping[];
}

export type ContextMapPattern =
  | 'SharedKernel'
  | 'CustomerSupplier'
  | 'Conformist'
  | 'AntiCorruptionLayer'
  | 'OpenHostService'
  | 'PublishedLanguage'
  | 'Partnership'
  | 'SeparateWays';

export interface ObjectMapping {
  sourceId: string;
  targetId: string;
}

export interface DomainModel {
  contexts: Record<string, ContextData>;
  contextMaps: ContextMap[];
}

export interface ContextData {
  id: string;
  name: string;
  nodes: Record<string, DomainNodeWithPosition>;
  morphisms: Morphism[];
}

export interface DomainNodeWithPosition {
  node: DomainNode;
  position: { x: number; y: number };
}

// Validation types
export interface ValidationError {
  code: string;
  message: string;
  nodeId?: string;
  field?: string;
  severity: 'error' | 'warning' | 'info';
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationError[];
}
