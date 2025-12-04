import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { NodeProps } from '@xyflow/react';
import { Layers } from 'lucide-react';
import type { AggregateNode as AggregateNodeType } from '@/types';

export const AggregateNode = memo(function AggregateNode({ data, selected }: NodeProps) {
  const aggregateData = data as unknown as AggregateNodeType;

  return (
    <div
      className={`
        min-w-[200px] rounded-lg border-2 bg-red-50 dark:bg-red-950
        ${selected ? 'border-red-600 shadow-lg' : 'border-red-500'}
      `}
    >
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 bg-red-500 text-white rounded-t-md">
        <Layers className="w-4 h-4" />
        <span className="font-semibold text-sm">{aggregateData.name}</span>
        <span className="ml-auto text-xs opacity-75">Aggregate</span>
      </div>

      {/* Content */}
      <div className="p-2 space-y-2">
        {/* Root */}
        <div className="text-xs">
          <span className="text-slate-500 dark:text-slate-400">Root: </span>
          {aggregateData.rootId ? (
            <span className="font-medium text-red-600 dark:text-red-400">
              {aggregateData.rootId}
            </span>
          ) : (
            <span className="italic text-slate-400 dark:text-slate-500">
              Not set
            </span>
          )}
        </div>

        {/* Members */}
        <div className="text-xs">
          <span className="text-slate-500 dark:text-slate-400">Members: </span>
          {aggregateData.memberIds.length > 0 ? (
            <span className="font-medium text-slate-700 dark:text-slate-300">
              {aggregateData.memberIds.length} entities
            </span>
          ) : (
            <span className="italic text-slate-400 dark:text-slate-500">
              None
            </span>
          )}
        </div>

        {/* Invariants */}
        {aggregateData.invariants.length > 0 && (
          <div className="pt-2 border-t border-red-200 dark:border-red-800">
            <div className="text-xs text-slate-500 dark:text-slate-400 mb-1">
              Invariants:
            </div>
            <ul className="space-y-0.5">
              {aggregateData.invariants.map((inv, i) => (
                <li
                  key={i}
                  className="text-xs text-slate-600 dark:text-slate-300 bg-red-100 dark:bg-red-900/50 px-1.5 py-0.5 rounded"
                >
                  {inv}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Handles for connections */}
      <Handle
        type="target"
        position={Position.Left}
        className="w-3 h-3 bg-red-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-red-500 border-2 border-white"
      />
    </div>
  );
});
