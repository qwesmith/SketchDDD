import { useDomainStore } from '@/stores';
import { X, Plus, Trash2 } from 'lucide-react';
import type { Field, EnumVariant } from '@/types';

export function PropertiesPanel() {
  const {
    isPropertiesPanelOpen,
    togglePropertiesPanel,
    activeContextId,
    contexts,
    selectedNodeIds,
    updateNode,
    addField,
    updateField,
    deleteField,
    addVariant,
    updateVariant,
    deleteVariant,
  } = useDomainStore();

  if (!isPropertiesPanelOpen) return null;

  const activeContext = activeContextId ? contexts[activeContextId] : null;
  const selectedNodeId = selectedNodeIds[0];
  const selectedNode = activeContext && selectedNodeId
    ? activeContext.nodes[selectedNodeId]
    : null;

  return (
    <aside className="w-72 bg-white dark:bg-slate-800 border-l border-slate-200 dark:border-slate-700 flex flex-col">
      <div className="flex items-center justify-between p-3 border-b border-slate-200 dark:border-slate-700">
        <span className="text-sm font-semibold">Properties</span>
        <button
          onClick={togglePropertiesPanel}
          className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-3">
        {!selectedNode ? (
          <div className="text-sm text-slate-500 dark:text-slate-400 text-center py-8">
            Select a node to edit its properties
          </div>
        ) : (
          <div className="space-y-4">
            {/* Name */}
            <div>
              <label className="block text-xs font-medium text-slate-500 dark:text-slate-400 mb-1">
                Name
              </label>
              <input
                type="text"
                value={selectedNode.node.name}
                onChange={(e) =>
                  updateNode(activeContextId!, selectedNodeId, { name: e.target.value })
                }
                className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
              />
            </div>

            {/* Type badge */}
            <div>
              <label className="block text-xs font-medium text-slate-500 dark:text-slate-400 mb-1">
                Type
              </label>
              <span
                className={`
                  inline-block px-2 py-0.5 text-xs font-medium rounded
                  ${selectedNode.node.kind === 'entity' && 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'}
                  ${selectedNode.node.kind === 'value' && 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-200'}
                  ${selectedNode.node.kind === 'enum' && 'bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200'}
                  ${selectedNode.node.kind === 'aggregate' && 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'}
                `}
              >
                {selectedNode.node.kind}
              </span>
            </div>

            {/* Fields for entity/value */}
            {(selectedNode.node.kind === 'entity' || selectedNode.node.kind === 'value') && (
              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="text-xs font-medium text-slate-500 dark:text-slate-400">
                    Fields
                  </label>
                  <button
                    onClick={() =>
                      addField(activeContextId!, selectedNodeId, {
                        name: 'newField',
                        type: 'String',
                        optional: false,
                      })
                    }
                    className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
                    title="Add Field"
                  >
                    <Plus className="w-4 h-4" />
                  </button>
                </div>
                <div className="space-y-2">
                  {selectedNode.node.fields.map((field: Field) => (
                    <div
                      key={field.id}
                      className="flex items-center gap-2 p-2 bg-slate-50 dark:bg-slate-700/50 rounded"
                    >
                      <input
                        type="text"
                        value={field.name}
                        onChange={(e) =>
                          updateField(activeContextId!, selectedNodeId, field.id, {
                            name: e.target.value,
                          })
                        }
                        className="flex-1 px-1.5 py-0.5 text-xs border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                        placeholder="name"
                      />
                      <input
                        type="text"
                        value={field.type}
                        onChange={(e) =>
                          updateField(activeContextId!, selectedNodeId, field.id, {
                            type: e.target.value,
                          })
                        }
                        className="w-20 px-1.5 py-0.5 text-xs border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                        placeholder="type"
                      />
                      <label className="flex items-center gap-1 text-xs">
                        <input
                          type="checkbox"
                          checked={field.optional}
                          onChange={(e) =>
                            updateField(activeContextId!, selectedNodeId, field.id, {
                              optional: e.target.checked,
                            })
                          }
                        />
                        ?
                      </label>
                      <button
                        onClick={() =>
                          deleteField(activeContextId!, selectedNodeId, field.id)
                        }
                        className="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/30 hover:text-red-600"
                      >
                        <Trash2 className="w-3 h-3" />
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Variants for enum */}
            {selectedNode.node.kind === 'enum' && (
              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="text-xs font-medium text-slate-500 dark:text-slate-400">
                    Variants
                  </label>
                  <button
                    onClick={() =>
                      addVariant(activeContextId!, selectedNodeId, {
                        name: 'NewVariant',
                      })
                    }
                    className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
                    title="Add Variant"
                  >
                    <Plus className="w-4 h-4" />
                  </button>
                </div>
                <div className="space-y-2">
                  {selectedNode.node.variants.map((variant: EnumVariant) => (
                    <div
                      key={variant.id}
                      className="flex items-center gap-2 p-2 bg-slate-50 dark:bg-slate-700/50 rounded"
                    >
                      <input
                        type="text"
                        value={variant.name}
                        onChange={(e) =>
                          updateVariant(activeContextId!, selectedNodeId, variant.id, {
                            name: e.target.value,
                          })
                        }
                        className="flex-1 px-1.5 py-0.5 text-xs border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                        placeholder="variant"
                      />
                      <input
                        type="text"
                        value={variant.payload || ''}
                        onChange={(e) =>
                          updateVariant(activeContextId!, selectedNodeId, variant.id, {
                            payload: e.target.value || undefined,
                          })
                        }
                        className="w-24 px-1.5 py-0.5 text-xs border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                        placeholder="payload"
                      />
                      <button
                        onClick={() =>
                          deleteVariant(activeContextId!, selectedNodeId, variant.id)
                        }
                        className="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/30 hover:text-red-600"
                      >
                        <Trash2 className="w-3 h-3" />
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </aside>
  );
}
