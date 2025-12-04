import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { NodeProps } from '@xyflow/react';
import { Diamond } from 'lucide-react';
import type { ValueNode as ValueNodeType } from '@/types';

export const ValueNode = memo(function ValueNode({ data, selected }: NodeProps) {
  const valueData = data as unknown as ValueNodeType;

  return (
    <div
      className={`
        min-w-[180px] rounded-lg border-2 bg-emerald-50 dark:bg-emerald-950
        ${selected ? 'border-emerald-600 shadow-lg' : 'border-emerald-500'}
      `}
    >
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 bg-emerald-500 text-white rounded-t-md">
        <Diamond className="w-4 h-4" />
        <span className="font-semibold text-sm">{valueData.name}</span>
        <span className="ml-auto text-xs opacity-75">Value</span>
      </div>

      {/* Fields */}
      <div className="p-2">
        {valueData.fields.length === 0 ? (
          <div className="text-xs text-slate-500 dark:text-slate-400 italic py-1">
            No fields defined
          </div>
        ) : (
          <ul className="space-y-1">
            {valueData.fields.map((field) => (
              <li
                key={field.id}
                className="flex items-center gap-2 text-xs px-1 py-0.5 rounded hover:bg-emerald-100 dark:hover:bg-emerald-900/50"
              >
                <span className="font-medium text-slate-700 dark:text-slate-300">
                  {field.name}
                </span>
                <span className="text-slate-500 dark:text-slate-400">:</span>
                <span className="text-emerald-600 dark:text-emerald-400">
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
        className="w-3 h-3 bg-emerald-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-emerald-500 border-2 border-white"
      />
    </div>
  );
});
