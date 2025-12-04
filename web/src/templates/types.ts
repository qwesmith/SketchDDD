import type { DomainNodeWithPosition, Morphism } from '@/types';

export interface TemplateMetadata {
  id: string;
  name: string;
  description: string;
  category: TemplateCategory;
  tags: string[];
  thumbnail?: string;
  author?: string;
  version: string;
}

export type TemplateCategory =
  | 'ecommerce'
  | 'healthcare'
  | 'banking'
  | 'inventory'
  | 'project-management'
  | 'custom';

export interface TemplateContext {
  name: string;
  nodes: Record<string, DomainNodeWithPosition>;
  morphisms: Morphism[];
}

export interface Template {
  metadata: TemplateMetadata;
  context: TemplateContext;
}

export interface TemplatePreview {
  metadata: TemplateMetadata;
  entityCount: number;
  valueCount: number;
  enumCount: number;
  aggregateCount: number;
  morphismCount: number;
}
