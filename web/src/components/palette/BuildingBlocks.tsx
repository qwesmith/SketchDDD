import { useDomainStore } from '@/stores';
import type { NodeKind, DomainNode } from '@/types';
import {
  Box,
  Diamond,
  List,
  Layers,
  ArrowRight,
} from 'lucide-react';

interface BlockDefinition {
  kind: NodeKind;
  label: string;
  description: string;
  icon: React.ReactNode;
  color: string;
}

const blocks: BlockDefinition[] = [
  {
    kind: 'entity',
    label: 'Entity',
    description: 'Object with identity',
    icon: <Box className="w-5 h-5" />,
    color: 'border-blue-500 bg-blue-50 dark:bg-blue-950',
  },
  {
    kind: 'value',
    label: 'Value Object',
    description: 'Immutable, no identity',
    icon: <Diamond className="w-5 h-5" />,
    color: 'border-emerald-500 bg-emerald-50 dark:bg-emerald-950',
  },
  {
    kind: 'enum',
    label: 'Enum',
    description: 'Set of named values',
    icon: <List className="w-5 h-5" />,
    color: 'border-amber-500 bg-amber-50 dark:bg-amber-950',
  },
  {
    kind: 'aggregate',
    label: 'Aggregate',
    description: 'Consistency boundary',
    icon: <Layers className="w-5 h-5" />,
    color: 'border-red-500 bg-red-50 dark:bg-red-950',
  },
];

export function BuildingBlocks() {
  const { activeContextId, addNode } = useDomainStore();

  const handleAddBlock = (kind: NodeKind) => {
    if (!activeContextId) {
      alert('Please select or create a context first');
      return;
    }

    let node: DomainNode;
    const name = `New${kind.charAt(0).toUpperCase() + kind.slice(1)}`;

    switch (kind) {
      case 'entity':
        node = {
          kind: 'entity',
          name,
          fields: [{ id: crypto.randomUUID(), name: 'id', type: 'UUID', optional: false }],
        };
        break;
      case 'value':
        node = {
          kind: 'value',
          name,
          fields: [],
        };
        break;
      case 'enum':
        node = {
          kind: 'enum',
          name,
          variants: [{ id: crypto.randomUUID(), name: 'Variant1' }],
        };
        break;
      case 'aggregate':
        node = {
          kind: 'aggregate',
          name,
          rootId: '',
          memberIds: [],
          invariants: [],
        };
        break;
      default:
        return;
    }

    // Add at a random position in the viewport
    const position = {
      x: 100 + Math.random() * 200,
      y: 100 + Math.random() * 200,
    };

    addNode(activeContextId, node, position);
  };

  const handleDragStart = (e: React.DragEvent, kind: NodeKind) => {
    e.dataTransfer.setData('application/sketchddd-node', kind);
    e.dataTransfer.effectAllowed = 'copy';
  };

  return (
    <div className="p-3">
      <h3 className="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wide mb-3">
        Building Blocks
      </h3>
      <div className="space-y-2">
        {blocks.map((block) => (
          <div
            key={block.kind}
            draggable
            onDragStart={(e) => handleDragStart(e, block.kind)}
            onClick={() => handleAddBlock(block.kind)}
            className={`
              flex items-center gap-3 p-2 rounded-lg border-2 cursor-pointer
              transition-all hover:shadow-md hover:scale-[1.02]
              ${block.color}
            `}
          >
            <div className="text-slate-700 dark:text-slate-300">{block.icon}</div>
            <div className="flex-1 min-w-0">
              <div className="text-sm font-medium text-slate-800 dark:text-slate-200">
                {block.label}
              </div>
              <div className="text-xs text-slate-500 dark:text-slate-400 truncate">
                {block.description}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Morphism hint */}
      <div className="mt-4 pt-4 border-t border-slate-200 dark:border-slate-700">
        <h3 className="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wide mb-3">
          Relationships
        </h3>
        <div className="flex items-center gap-2 p-2 rounded bg-slate-100 dark:bg-slate-700/50 text-sm text-slate-600 dark:text-slate-400">
          <ArrowRight className="w-4 h-4" />
          <span>Drag between nodes to create morphisms</span>
        </div>
      </div>
    </div>
  );
}
