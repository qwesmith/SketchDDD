import type { Template } from './types';
import type { DomainNodeWithPosition } from '@/types';
import { ecommerceTemplate } from './ecommerce';
import { healthcareTemplate } from './healthcare';
import { bankingTemplate } from './banking';
import { inventoryTemplate } from './inventory';
import { projectManagementTemplate } from './project-management';

export type { Template, TemplateMetadata, TemplateCategory, TemplatePreview } from './types';

export const templates: Template[] = [
  ecommerceTemplate,
  healthcareTemplate,
  bankingTemplate,
  inventoryTemplate,
  projectManagementTemplate,
];

export function getTemplateById(id: string): Template | undefined {
  return templates.find((t) => t.metadata.id === id);
}

export function getTemplatePreview(template: Template) {
  const nodes: DomainNodeWithPosition[] = Object.values(template.context.nodes);

  return {
    metadata: template.metadata,
    entityCount: nodes.filter((n) => n.node.kind === 'entity').length,
    valueCount: nodes.filter((n) => n.node.kind === 'value').length,
    enumCount: nodes.filter((n) => n.node.kind === 'enum').length,
    aggregateCount: nodes.filter((n) => n.node.kind === 'aggregate').length,
    morphismCount: template.context.morphisms.length,
  };
}

export function getTemplatesByCategory(category: string): Template[] {
  return templates.filter((t) => t.metadata.category === category);
}

export function searchTemplates(query: string): Template[] {
  const lowerQuery = query.toLowerCase();
  return templates.filter(
    (t) =>
      t.metadata.name.toLowerCase().includes(lowerQuery) ||
      t.metadata.description.toLowerCase().includes(lowerQuery) ||
      t.metadata.tags.some((tag) => tag.toLowerCase().includes(lowerQuery))
  );
}
