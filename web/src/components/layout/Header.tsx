import { useState } from 'react';
import {
  Menu,
  Save,
  Upload,
  Download,
  Undo2,
  Redo2,
  Settings,
  FileCode,
  Play,
  LayoutTemplate,
} from 'lucide-react';
import { useDomainStore } from '@/stores';
import { TemplateBrowser } from '../templates';

export function Header() {
  const { canUndo, canRedo, undo, redo, exportModel, togglePalette } = useDomainStore();
  const [showTemplateBrowser, setShowTemplateBrowser] = useState(false);

  const handleExport = () => {
    const sddd = exportModel();
    const blob = new Blob([sddd], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'domain.sddd.json';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <header className="h-12 bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between px-4">
      {/* Left section */}
      <div className="flex items-center gap-4">
        <button
          onClick={togglePalette}
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Toggle Palette"
        >
          <Menu className="w-5 h-5" />
        </button>

        <div className="flex items-center gap-2">
          <FileCode className="w-5 h-5 text-primary" />
          <span className="font-semibold">SketchDDD</span>
        </div>
      </div>

      {/* Center section - Undo/Redo */}
      <div className="flex items-center gap-1">
        <button
          onClick={undo}
          disabled={!canUndo()}
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed"
          title="Undo (Ctrl+Z)"
        >
          <Undo2 className="w-4 h-4" />
        </button>
        <button
          onClick={redo}
          disabled={!canRedo()}
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed"
          title="Redo (Ctrl+Shift+Z)"
        >
          <Redo2 className="w-4 h-4" />
        </button>
      </div>

      {/* Right section */}
      <div className="flex items-center gap-2">
        <button
          onClick={() => setShowTemplateBrowser(true)}
          className="flex items-center gap-2 px-3 py-1.5 rounded border border-slate-300 dark:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Browse Templates"
        >
          <LayoutTemplate className="w-4 h-4" />
          <span className="text-sm">Templates</span>
        </button>

        <button
          className="flex items-center gap-2 px-3 py-1.5 rounded bg-primary text-white hover:bg-primary-hover"
          title="Validate Model"
        >
          <Play className="w-4 h-4" />
          <span className="text-sm">Validate</span>
        </button>

        <div className="w-px h-6 bg-slate-200 dark:bg-slate-700 mx-2" />

        <button
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Import"
        >
          <Upload className="w-4 h-4" />
        </button>
        <button
          onClick={handleExport}
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Export"
        >
          <Download className="w-4 h-4" />
        </button>
        <button
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Save"
        >
          <Save className="w-4 h-4" />
        </button>
        <button
          className="p-2 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          title="Settings"
        >
          <Settings className="w-4 h-4" />
        </button>
      </div>

      {/* Template Browser Modal */}
      <TemplateBrowser
        isOpen={showTemplateBrowser}
        onClose={() => setShowTemplateBrowser(false)}
      />
    </header>
  );
}
