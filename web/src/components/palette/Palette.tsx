import { useDomainStore } from '@/stores';
import { BuildingBlocks } from './BuildingBlocks';
import { X } from 'lucide-react';

export function Palette() {
  const { isPaletteOpen, togglePalette } = useDomainStore();

  if (!isPaletteOpen) {
    return null;
  }

  return (
    <aside className="w-64 bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700 flex flex-col">
      <div className="flex items-center justify-between p-3 border-b border-slate-200 dark:border-slate-700">
        <span className="text-sm font-semibold">Palette</span>
        <button
          onClick={togglePalette}
          className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
        >
          <X className="w-4 h-4" />
        </button>
      </div>
      <div className="flex-1 overflow-y-auto">
        <BuildingBlocks />
      </div>
    </aside>
  );
}
