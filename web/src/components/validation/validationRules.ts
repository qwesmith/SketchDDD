import type { ContextData, ValidationError, DomainNode, EntityNode, ValueNode, EnumNode, AggregateNode } from '@/types';

type ValidationRule = (context: ContextData) => ValidationError[];

// Helper to create validation errors
function createError(
  code: string,
  message: string,
  severity: 'error' | 'warning' | 'info',
  nodeId?: string,
  field?: string
): ValidationError {
  return { code, message, severity, nodeId, field };
}

// Check for entities without identity fields (id)
const entityNeedsIdentity: ValidationRule = (context) => {
  const errors: ValidationError[] = [];

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (node.kind !== 'entity') return;
    const entity = node as EntityNode;

    const hasIdField = entity.fields.some(
      (f) => f.name.toLowerCase() === 'id' || f.type === 'UUID'
    );

    if (!hasIdField) {
      errors.push(
        createError(
          'ENTITY_NO_IDENTITY',
          `Entity "${entity.name}" should have an identity field (typically 'id' of type UUID)`,
          'warning',
          id
        )
      );
    }
  });

  return errors;
};

// Check for empty nodes (no fields/variants)
const nodeNotEmpty: ValidationRule = (context) => {
  const errors: ValidationError[] = [];

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (node.kind === 'entity' || node.kind === 'value') {
      const hasFields = (node as EntityNode | ValueNode).fields.length > 0;
      if (!hasFields) {
        errors.push(
          createError(
            'NODE_EMPTY',
            `${node.kind === 'entity' ? 'Entity' : 'Value Object'} "${node.name}" has no fields defined`,
            'warning',
            id
          )
        );
      }
    }

    if (node.kind === 'enum') {
      const hasVariants = (node as EnumNode).variants.length > 0;
      if (!hasVariants) {
        errors.push(
          createError(
            'ENUM_NO_VARIANTS',
            `Enum "${node.name}" has no variants defined`,
            'error',
            id
          )
        );
      }
    }
  });

  return errors;
};

// Check for aggregates with valid root
const aggregateHasValidRoot: ValidationRule = (context) => {
  const errors: ValidationError[] = [];

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (node.kind !== 'aggregate') return;
    const aggregate = node as AggregateNode;

    if (!aggregate.rootId) {
      errors.push(
        createError(
          'AGGREGATE_NO_ROOT',
          `Aggregate "${aggregate.name}" must have a root entity`,
          'error',
          id
        )
      );
      return;
    }

    const rootNode = context.nodes[aggregate.rootId];
    if (!rootNode) {
      errors.push(
        createError(
          'AGGREGATE_INVALID_ROOT',
          `Aggregate "${aggregate.name}" references a non-existent root entity`,
          'error',
          id
        )
      );
    } else if (rootNode.node.kind !== 'entity') {
      errors.push(
        createError(
          'AGGREGATE_ROOT_NOT_ENTITY',
          `Aggregate "${aggregate.name}" root must be an entity, not a ${rootNode.node.kind}`,
          'error',
          id
        )
      );
    }
  });

  return errors;
};

// Check for morphisms with valid source/target
const morphismHasValidEndpoints: ValidationRule = (context) => {
  const errors: ValidationError[] = [];

  context.morphisms.forEach((morphism) => {
    if (!context.nodes[morphism.sourceId]) {
      errors.push(
        createError(
          'MORPHISM_INVALID_SOURCE',
          `Relationship "${morphism.name}" references a non-existent source`,
          'error',
          undefined,
          morphism.id
        )
      );
    }

    if (!context.nodes[morphism.targetId]) {
      errors.push(
        createError(
          'MORPHISM_INVALID_TARGET',
          `Relationship "${morphism.name}" references a non-existent target`,
          'error',
          undefined,
          morphism.id
        )
      );
    }
  });

  return errors;
};

// Check for duplicate names
const uniqueNames: ValidationRule = (context) => {
  const errors: ValidationError[] = [];
  const names = new Map<string, string[]>();

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    const existing = names.get(node.name.toLowerCase()) || [];
    existing.push(id);
    names.set(node.name.toLowerCase(), existing);
  });

  names.forEach((ids, _name) => {
    if (ids.length > 1) {
      ids.forEach((id) => {
        errors.push(
          createError(
            'DUPLICATE_NAME',
            `Multiple concepts named "${context.nodes[id].node.name}" exist. Consider unique names.`,
            'warning',
            id
          )
        );
      });
    }
  });

  return errors;
};

// Check for naming conventions (PascalCase for types)
const namingConventions: ValidationRule = (context) => {
  const errors: ValidationError[] = [];
  const pascalCaseRegex = /^[A-Z][a-zA-Z0-9]*$/;

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (!pascalCaseRegex.test(node.name)) {
      errors.push(
        createError(
          'NAMING_CONVENTION',
          `"${node.name}" should use PascalCase (e.g., "${toPascalCase(node.name)}")`,
          'info',
          id
        )
      );
    }
  });

  return errors;
};

// Check for orphan nodes (no relationships)
const noOrphanNodes: ValidationRule = (context) => {
  const errors: ValidationError[] = [];
  const connectedNodes = new Set<string>();

  context.morphisms.forEach((m) => {
    connectedNodes.add(m.sourceId);
    connectedNodes.add(m.targetId);
  });

  // Aggregates also connect their root and members
  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (node.kind === 'aggregate') {
      const aggregate = node as AggregateNode;
      connectedNodes.add(id);
      if (aggregate.rootId) connectedNodes.add(aggregate.rootId);
      aggregate.memberIds.forEach((mid) => connectedNodes.add(mid));
    }
  });

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    // Skip aggregates, they connect themselves
    if (node.kind === 'aggregate') return;

    if (!connectedNodes.has(id)) {
      errors.push(
        createError(
          'ORPHAN_NODE',
          `"${node.name}" has no relationships with other concepts`,
          'info',
          id
        )
      );
    }
  });

  return errors;
};

// Check for circular dependencies in aggregates
const noCircularAggregates: ValidationRule = (context) => {
  const errors: ValidationError[] = [];

  Object.entries(context.nodes).forEach(([id, { node }]) => {
    if (node.kind !== 'aggregate') return;
    const aggregate = node as AggregateNode;

    // Check if root is the aggregate itself
    if (aggregate.rootId === id) {
      errors.push(
        createError(
          'AGGREGATE_SELF_ROOT',
          `Aggregate "${aggregate.name}" cannot be its own root`,
          'error',
          id
        )
      );
    }

    // Check if members include the aggregate
    if (aggregate.memberIds.includes(id)) {
      errors.push(
        createError(
          'AGGREGATE_SELF_MEMBER',
          `Aggregate "${aggregate.name}" cannot be its own member`,
          'error',
          id
        )
      );
    }
  });

  return errors;
};

// Helper to convert to PascalCase
function toPascalCase(str: string): string {
  return str
    .split(/[\s_-]+/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join('');
}

// All validation rules
const validationRules: ValidationRule[] = [
  entityNeedsIdentity,
  nodeNotEmpty,
  aggregateHasValidRoot,
  morphismHasValidEndpoints,
  uniqueNames,
  namingConventions,
  noOrphanNodes,
  noCircularAggregates,
];

// Run all validations
export function validateContext(context: ContextData): ValidationError[] {
  const errors: ValidationError[] = [];

  validationRules.forEach((rule) => {
    errors.push(...rule(context));
  });

  return errors;
}

// Quick fixes for common errors
export interface QuickFix {
  label: string;
  action: () => void;
}

export function getQuickFixes(
  error: ValidationError,
  _context: ContextData,
  actions: {
    addField: (nodeId: string, field: { name: string; type: string; optional: boolean }) => void;
    addVariant: (nodeId: string, variant: { name: string }) => void;
    deleteNode: (nodeId: string) => void;
    updateNode: (nodeId: string, updates: Partial<DomainNode>) => void;
  }
): QuickFix[] {
  const fixes: QuickFix[] = [];

  switch (error.code) {
    case 'ENTITY_NO_IDENTITY':
      if (error.nodeId) {
        fixes.push({
          label: 'Add UUID id field',
          action: () =>
            actions.addField(error.nodeId!, {
              name: 'id',
              type: 'UUID',
              optional: false,
            }),
        });
      }
      break;

    case 'ENUM_NO_VARIANTS':
      if (error.nodeId) {
        fixes.push({
          label: 'Add default variant',
          action: () =>
            actions.addVariant(error.nodeId!, { name: 'Default' }),
        });
      }
      break;

    case 'ORPHAN_NODE':
      if (error.nodeId) {
        fixes.push({
          label: 'Delete this node',
          action: () => actions.deleteNode(error.nodeId!),
        });
      }
      break;
  }

  return fixes;
}
