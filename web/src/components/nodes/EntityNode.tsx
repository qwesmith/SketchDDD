import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { NodeProps } from '@xyflow/react';
import { Box } from 'lucide-react';
import type { EntityNode as EntityNodeType } from '@/types';

export const EntityNode = memo(function EntityNode({ data, selected }: NodeProps) {
  const entityData = data as unknown as EntityNodeType;

  return (
    <div
      className={`
        min-w-[180px] rounded-lg border-2 bg-blue-50 dark:bg-blue-950
        ${selected ? 'border-blue-600 shadow-lg' : 'border-blue-500'}
      `}
    >
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 bg-blue-500 text-white rounded-t-md">
        <Box className="w-4 h-4" />
        <span className="font-semibold text-sm">{entityData.name}</span>
        <span className="ml-auto text-xs opacity-75">Entity</span>
      </div>

      {/* Fields */}
      <div className="p-2">
        {entityData.fields.length === 0 ? (
          <div className="text-xs text-slate-500 dark:text-slate-400 italic py-1">
            No fields defined
          </div>
        ) : (
          <ul className="space-y-1">
            {entityData.fields.map((field) => (
              <li
                key={field.id}
                className="flex items-center gap-2 text-xs px-1 py-0.5 rounded hover:bg-blue-100 dark:hover:bg-blue-900/50"
              >
                <span className="font-medium text-slate-700 dark:text-slate-300">
                  {field.name}
                </span>
                <span className="text-slate-500 dark:text-slate-400">:</span>
                <span className="text-blue-600 dark:text-blue-400">
                  {field.type}
                  {field.optional && '?'}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* Handles for connections */}
      <Handle
        type="target"
        position={Position.Left}
        className="w-3 h-3 bg-blue-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-blue-500 border-2 border-white"
      />
    </div>
  );
});
