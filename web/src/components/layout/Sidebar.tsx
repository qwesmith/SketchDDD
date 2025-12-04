import { useState } from 'react';
import { Plus, Trash2, ChevronRight, FolderOpen } from 'lucide-react';
import { useDomainStore } from '@/stores';

export function Sidebar() {
  const {
    contexts,
    activeContextId,
    setActiveContext,
    createContext,
    deleteContext,
    renameContext,
  } = useDomainStore();

  const [editingId, setEditingId] = useState<string | null>(null);
  const [editingName, setEditingName] = useState('');

  const handleCreateContext = () => {
    const name = `Context${Object.keys(contexts).length + 1}`;
    createContext(name);
  };

  const handleStartRename = (id: string, currentName: string) => {
    setEditingId(id);
    setEditingName(currentName);
  };

  const handleFinishRename = (id: string) => {
    if (editingName.trim()) {
      renameContext(id, editingName.trim());
    }
    setEditingId(null);
  };

  const handleDelete = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (confirm('Are you sure you want to delete this context?')) {
      deleteContext(id);
    }
  };

  const contextList = Object.values(contexts);

  return (
    <aside className="w-56 bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700 flex flex-col">
      {/* Header */}
      <div className="p-3 border-b border-slate-200 dark:border-slate-700">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium text-slate-600 dark:text-slate-400">
            Bounded Contexts
          </span>
          <button
            onClick={handleCreateContext}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
            title="New Context"
          >
            <Plus className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Context List */}
      <div className="flex-1 overflow-y-auto p-2">
        {contextList.length === 0 ? (
          <div className="text-center text-sm text-slate-500 dark:text-slate-400 py-8">
            <FolderOpen className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <p>No contexts yet</p>
            <button
              onClick={handleCreateContext}
              className="mt-2 text-primary hover:underline"
            >
              Create your first context
            </button>
          </div>
        ) : (
          <ul className="space-y-1">
            {contextList.map((context) => (
              <li key={context.id}>
                <div
                  onClick={() => setActiveContext(context.id)}
                  className={`
                    flex items-center gap-2 px-2 py-1.5 rounded cursor-pointer group
                    ${
                      activeContextId === context.id
                        ? 'bg-primary/10 text-primary'
                        : 'hover:bg-slate-100 dark:hover:bg-slate-700'
                    }
                  `}
                >
                  <ChevronRight
                    className={`w-4 h-4 transition-transform ${
                      activeContextId === context.id ? 'rotate-90' : ''
                    }`}
                  />

                  {editingId === context.id ? (
                    <input
                      type="text"
                      value={editingName}
                      onChange={(e) => setEditingName(e.target.value)}
                      onBlur={() => handleFinishRename(context.id)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter') handleFinishRename(context.id);
                        if (e.key === 'Escape') setEditingId(null);
                      }}
                      className="flex-1 px-1 py-0.5 text-sm bg-white dark:bg-slate-700 border border-primary rounded"
                      autoFocus
                      onClick={(e) => e.stopPropagation()}
                    />
                  ) : (
                    <span
                      className="flex-1 text-sm truncate"
                      onDoubleClick={() => handleStartRename(context.id, context.name)}
                    >
                      {context.name}
                    </span>
                  )}

                  <button
                    onClick={(e) => handleDelete(context.id, e)}
                    className="p-1 rounded opacity-0 group-hover:opacity-100 hover:bg-red-100 dark:hover:bg-red-900/30 hover:text-red-600"
                    title="Delete Context"
                  >
                    <Trash2 className="w-3 h-3" />
                  </button>
                </div>

                {/* Node count */}
                {activeContextId === context.id && (
                  <div className="ml-6 mt-1 text-xs text-slate-500 dark:text-slate-400">
                    {Object.keys(context.nodes).length} nodes,{' '}
                    {context.morphisms.length} morphisms
                  </div>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>
    </aside>
  );
}
