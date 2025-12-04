import { useState, useMemo } from 'react';
import {
  X,
  Search,
  ShoppingCart,
  Stethoscope,
  Landmark,
  Package,
  FolderKanban,
  Box,
  Diamond,
  List,
  Layers,
  ArrowRight,
} from 'lucide-react';
import { templates, getTemplatePreview } from '@/templates';
import type { Template, TemplateCategory } from '@/templates';
import type { DomainNodeWithPosition, Morphism, AggregateNode } from '@/types';
import { useDomainStore } from '@/stores';

interface TemplateBrowserProps {
  isOpen: boolean;
  onClose: () => void;
}

const categoryIcons: Record<TemplateCategory, React.ReactNode> = {
  ecommerce: <ShoppingCart className="w-5 h-5" />,
  healthcare: <Stethoscope className="w-5 h-5" />,
  banking: <Landmark className="w-5 h-5" />,
  inventory: <Package className="w-5 h-5" />,
  'project-management': <FolderKanban className="w-5 h-5" />,
  custom: <Box className="w-5 h-5" />,
};

const categoryColors: Record<TemplateCategory, string> = {
  ecommerce: 'bg-blue-500',
  healthcare: 'bg-green-500',
  banking: 'bg-purple-500',
  inventory: 'bg-orange-500',
  'project-management': 'bg-pink-500',
  custom: 'bg-slate-500',
};

export function TemplateBrowser({ isOpen, onClose }: TemplateBrowserProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTemplate, setSelectedTemplate] = useState<Template | null>(null);
  const { createContext, setActiveContext } = useDomainStore();

  const filteredTemplates = useMemo(() => {
    if (!searchQuery.trim()) return templates;
    const query = searchQuery.toLowerCase();
    return templates.filter(
      (t) =>
        t.metadata.name.toLowerCase().includes(query) ||
        t.metadata.description.toLowerCase().includes(query) ||
        t.metadata.tags.some((tag) => tag.toLowerCase().includes(query))
    );
  }, [searchQuery]);

  const handleUseTemplate = (template: Template) => {
    // Generate new IDs for the template context
    const nodeIdMap = new Map<string, string>();
    const newNodes: Record<string, DomainNodeWithPosition> = {};

    // Map old node IDs to new ones
    Object.entries(template.context.nodes).forEach(([oldId, nodeData]) => {
      const newId = crypto.randomUUID();
      nodeIdMap.set(oldId, newId);

      // Create a copy of the node with proper typing
      if (nodeData.node.kind === 'aggregate') {
        newNodes[newId] = {
          position: { ...nodeData.position },
          node: {
            kind: 'aggregate',
            name: nodeData.node.name,
            rootId: '', // Will be updated below
            memberIds: [],
            invariants: [...(nodeData.node as AggregateNode).invariants],
          },
        };
      } else {
        newNodes[newId] = {
          position: { ...nodeData.position },
          node: { ...nodeData.node },
        };
      }
    });

    // Update aggregate references
    Object.entries(newNodes).forEach(([newId, nodeData]) => {
      if (nodeData.node.kind === 'aggregate') {
        // Find the original node
        const oldEntry = Object.entries(template.context.nodes).find(
          ([oldId]) => nodeIdMap.get(oldId) === newId
        );
        if (oldEntry && oldEntry[1].node.kind === 'aggregate') {
          const oldAggregate = oldEntry[1].node as AggregateNode;
          (nodeData.node as AggregateNode).rootId = nodeIdMap.get(oldAggregate.rootId) || '';
          (nodeData.node as AggregateNode).memberIds = oldAggregate.memberIds
            .map((id: string) => nodeIdMap.get(id))
            .filter((id): id is string => id !== undefined);
        }
      }
    });

    // Update morphisms with new IDs
    const newMorphisms: Morphism[] = template.context.morphisms.map((m) => ({
      ...m,
      id: crypto.randomUUID(),
      sourceId: nodeIdMap.get(m.sourceId) || m.sourceId,
      targetId: nodeIdMap.get(m.targetId) || m.targetId,
    }));

    // Create the new context
    const contextId = createContext(template.context.name);

    // Get the store and update with our nodes and morphisms
    const store = useDomainStore.getState();
    store.contexts[contextId] = {
      ...store.contexts[contextId],
      nodes: newNodes,
      morphisms: newMorphisms,
    };

    setActiveContext(contextId);
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />

      {/* Modal */}
      <div className="relative bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-4xl mx-4 max-h-[90vh] flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold">Template Library</h2>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Search */}
        <div className="px-6 py-3 border-b border-slate-200 dark:border-slate-700">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search templates..."
              className="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
            />
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-6">
          {selectedTemplate ? (
            // Template Detail View
            <div>
              <button
                onClick={() => setSelectedTemplate(null)}
                className="flex items-center gap-1 text-sm text-primary hover:underline mb-4"
              >
                <ArrowRight className="w-4 h-4 rotate-180" />
                Back to templates
              </button>

              <TemplateDetail
                template={selectedTemplate}
                onUse={() => handleUseTemplate(selectedTemplate)}
              />
            </div>
          ) : (
            // Template Grid
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {filteredTemplates.map((template) => (
                <TemplateCard
                  key={template.metadata.id}
                  template={template}
                  onClick={() => setSelectedTemplate(template)}
                />
              ))}
              {filteredTemplates.length === 0 && (
                <div className="col-span-2 text-center py-8 text-slate-500 dark:text-slate-400">
                  No templates found matching "{searchQuery}"
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface TemplateCardProps {
  template: Template;
  onClick: () => void;
}

function TemplateCard({ template, onClick }: TemplateCardProps) {
  const preview = getTemplatePreview(template);
  const { metadata } = template;

  return (
    <button
      onClick={onClick}
      className="p-4 rounded-lg border-2 border-slate-200 dark:border-slate-700 hover:border-primary text-left transition-all hover:shadow-md"
    >
      <div className="flex items-start gap-3">
        <div
          className={`p-2 rounded-lg text-white ${
            categoryColors[metadata.category]
          }`}
        >
          {categoryIcons[metadata.category]}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-slate-900 dark:text-slate-100">
            {metadata.name}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 line-clamp-2 mt-1">
            {metadata.description}
          </p>
        </div>
      </div>

      {/* Stats */}
      <div className="flex items-center gap-3 mt-4 text-xs text-slate-500 dark:text-slate-400">
        <span className="flex items-center gap-1">
          <Box className="w-3 h-3" /> {preview.entityCount}
        </span>
        <span className="flex items-center gap-1">
          <Diamond className="w-3 h-3" /> {preview.valueCount}
        </span>
        <span className="flex items-center gap-1">
          <List className="w-3 h-3" /> {preview.enumCount}
        </span>
        {preview.aggregateCount > 0 && (
          <span className="flex items-center gap-1">
            <Layers className="w-3 h-3" /> {preview.aggregateCount}
          </span>
        )}
        <span className="flex items-center gap-1">
          <ArrowRight className="w-3 h-3" /> {preview.morphismCount}
        </span>
      </div>

      {/* Tags */}
      <div className="flex flex-wrap gap-1 mt-3">
        {metadata.tags.slice(0, 4).map((tag) => (
          <span
            key={tag}
            className="px-2 py-0.5 text-xs rounded-full bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400"
          >
            {tag}
          </span>
        ))}
      </div>
    </button>
  );
}

interface TemplateDetailProps {
  template: Template;
  onUse: () => void;
}

function TemplateDetail({ template, onUse }: TemplateDetailProps) {
  const preview = getTemplatePreview(template);
  const { metadata, context } = template;

  const nodeArray: DomainNodeWithPosition[] = Object.values(context.nodes);
  const entities = nodeArray.filter((n) => n.node.kind === 'entity');
  const values = nodeArray.filter((n) => n.node.kind === 'value');
  const enums = nodeArray.filter((n) => n.node.kind === 'enum');
  const aggregates = nodeArray.filter((n) => n.node.kind === 'aggregate');

  return (
    <div>
      {/* Header */}
      <div className="flex items-start gap-4 mb-6">
        <div
          className={`p-3 rounded-lg text-white ${
            categoryColors[metadata.category]
          }`}
        >
          {categoryIcons[metadata.category]}
        </div>
        <div className="flex-1">
          <h2 className="text-xl font-bold text-slate-900 dark:text-slate-100">
            {metadata.name}
          </h2>
          <p className="text-slate-600 dark:text-slate-400 mt-1">
            {metadata.description}
          </p>
          <div className="flex flex-wrap gap-2 mt-3">
            {metadata.tags.map((tag) => (
              <span
                key={tag}
                className="px-2 py-0.5 text-xs rounded-full bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400"
              >
                {tag}
              </span>
            ))}
          </div>
        </div>
        <button
          onClick={onUse}
          className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-hover font-medium"
        >
          Use Template
        </button>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-5 gap-4 mb-6">
        <div className="p-3 rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800">
          <div className="flex items-center gap-2 text-blue-600 dark:text-blue-400">
            <Box className="w-4 h-4" />
            <span className="text-lg font-bold">{preview.entityCount}</span>
          </div>
          <div className="text-xs text-blue-600 dark:text-blue-400 mt-1">
            Entities
          </div>
        </div>
        <div className="p-3 rounded-lg bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800">
          <div className="flex items-center gap-2 text-emerald-600 dark:text-emerald-400">
            <Diamond className="w-4 h-4" />
            <span className="text-lg font-bold">{preview.valueCount}</span>
          </div>
          <div className="text-xs text-emerald-600 dark:text-emerald-400 mt-1">
            Values
          </div>
        </div>
        <div className="p-3 rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
          <div className="flex items-center gap-2 text-amber-600 dark:text-amber-400">
            <List className="w-4 h-4" />
            <span className="text-lg font-bold">{preview.enumCount}</span>
          </div>
          <div className="text-xs text-amber-600 dark:text-amber-400 mt-1">
            Enums
          </div>
        </div>
        <div className="p-3 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800">
          <div className="flex items-center gap-2 text-red-600 dark:text-red-400">
            <Layers className="w-4 h-4" />
            <span className="text-lg font-bold">{preview.aggregateCount}</span>
          </div>
          <div className="text-xs text-red-600 dark:text-red-400 mt-1">
            Aggregates
          </div>
        </div>
        <div className="p-3 rounded-lg bg-slate-50 dark:bg-slate-900/50 border border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2 text-slate-600 dark:text-slate-400">
            <ArrowRight className="w-4 h-4" />
            <span className="text-lg font-bold">{preview.morphismCount}</span>
          </div>
          <div className="text-xs text-slate-600 dark:text-slate-400 mt-1">
            Relations
          </div>
        </div>
      </div>

      {/* Content Preview */}
      <div className="grid grid-cols-2 gap-4">
        {/* Entities */}
        {entities.length > 0 && (
          <div className="p-4 rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800">
            <h3 className="font-medium text-blue-800 dark:text-blue-200 mb-2 flex items-center gap-2">
              <Box className="w-4 h-4" /> Entities
            </h3>
            <ul className="space-y-1">
              {entities.map((e, i) => (
                <li key={i} className="text-sm text-blue-700 dark:text-blue-300">
                  {e.node.name}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Value Objects */}
        {values.length > 0 && (
          <div className="p-4 rounded-lg bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800">
            <h3 className="font-medium text-emerald-800 dark:text-emerald-200 mb-2 flex items-center gap-2">
              <Diamond className="w-4 h-4" /> Value Objects
            </h3>
            <ul className="space-y-1">
              {values.map((v, i) => (
                <li key={i} className="text-sm text-emerald-700 dark:text-emerald-300">
                  {v.node.name}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Enums */}
        {enums.length > 0 && (
          <div className="p-4 rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
            <h3 className="font-medium text-amber-800 dark:text-amber-200 mb-2 flex items-center gap-2">
              <List className="w-4 h-4" /> Enums
            </h3>
            <ul className="space-y-1">
              {enums.map((e, i) => (
                <li key={i} className="text-sm text-amber-700 dark:text-amber-300">
                  {e.node.name}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Aggregates */}
        {aggregates.length > 0 && (
          <div className="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800">
            <h3 className="font-medium text-red-800 dark:text-red-200 mb-2 flex items-center gap-2">
              <Layers className="w-4 h-4" /> Aggregates
            </h3>
            <ul className="space-y-1">
              {aggregates.map((a, i) => (
                <li key={i} className="text-sm text-red-700 dark:text-red-300">
                  {a.node.name}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}
