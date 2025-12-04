import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { NodeProps } from '@xyflow/react';
import { List } from 'lucide-react';
import type { EnumNode as EnumNodeType } from '@/types';

export const EnumNode = memo(function EnumNode({ data, selected }: NodeProps) {
  const enumData = data as unknown as EnumNodeType;

  return (
    <div
      className={`
        min-w-[160px] rounded-lg border-2 bg-amber-50 dark:bg-amber-950
        ${selected ? 'border-amber-600 shadow-lg' : 'border-amber-500'}
      `}
    >
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 bg-amber-500 text-white rounded-t-md">
        <List className="w-4 h-4" />
        <span className="font-semibold text-sm">{enumData.name}</span>
        <span className="ml-auto text-xs opacity-75">Enum</span>
      </div>

      {/* Variants */}
      <div className="p-2">
        {enumData.variants.length === 0 ? (
          <div className="text-xs text-slate-500 dark:text-slate-400 italic py-1">
            No variants defined
          </div>
        ) : (
          <ul className="space-y-1">
            {enumData.variants.map((variant, index) => (
              <li
                key={variant.id}
                className="flex items-center gap-2 text-xs px-1 py-0.5 rounded hover:bg-amber-100 dark:hover:bg-amber-900/50"
              >
                {index > 0 && (
                  <span className="text-amber-400 dark:text-amber-600">|</span>
                )}
                <span className="font-medium text-slate-700 dark:text-slate-300">
                  {variant.name}
                </span>
                {variant.payload && (
                  <span className="text-amber-600 dark:text-amber-400">
                    ({variant.payload})
                  </span>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* Handles for connections */}
      <Handle
        type="target"
        position={Position.Left}
        className="w-3 h-3 bg-amber-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-amber-500 border-2 border-white"
      />
    </div>
  );
});
