import { useMemo } from 'react';
import {
  AlertCircle,
  AlertTriangle,
  Info,
  ChevronRight,
  CheckCircle,
  Zap,
} from 'lucide-react';
import { useDomainStore } from '@/stores';
import { validateContext, getQuickFixes } from './validationRules';
import type { ValidationError } from '@/types';

interface ValidationPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

const severityIcons = {
  error: <AlertCircle className="w-4 h-4 text-red-500" />,
  warning: <AlertTriangle className="w-4 h-4 text-amber-500" />,
  info: <Info className="w-4 h-4 text-blue-500" />,
};

const severityColors = {
  error: 'border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-900/20',
  warning: 'border-amber-200 bg-amber-50 dark:border-amber-800 dark:bg-amber-900/20',
  info: 'border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20',
};

export function ValidationPanel({ isOpen, onClose }: ValidationPanelProps) {
  const {
    activeContextId,
    contexts,
    setSelectedNodes,
    addField,
    addVariant,
    deleteNode,
    updateNode,
  } = useDomainStore();

  const activeContext = activeContextId ? contexts[activeContextId] : null;

  const errors = useMemo(() => {
    if (!activeContext) return [];
    return validateContext(activeContext);
  }, [activeContext]);

  const errorCount = errors.filter((e) => e.severity === 'error').length;
  const warningCount = errors.filter((e) => e.severity === 'warning').length;
  const infoCount = errors.filter((e) => e.severity === 'info').length;

  const handleNavigateToNode = (nodeId: string) => {
    setSelectedNodes([nodeId]);
  };

  const handleQuickFix = (error: ValidationError) => {
    if (!activeContext || !activeContextId) return;

    const fixes = getQuickFixes(error, activeContext, {
      addField: (nodeId, field) => addField(activeContextId, nodeId, field),
      addVariant: (nodeId, variant) => addVariant(activeContextId, nodeId, variant),
      deleteNode: (nodeId) => deleteNode(activeContextId, nodeId),
      updateNode: (nodeId, updates) => updateNode(activeContextId, nodeId, updates),
    });

    if (fixes.length > 0) {
      fixes[0].action();
    }
  };

  if (!isOpen) return null;

  return (
    <div className="w-80 border-l border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 flex flex-col h-full">
      {/* Header */}
      <div className="p-4 border-b border-slate-200 dark:border-slate-700">
        <div className="flex items-center justify-between mb-3">
          <h2 className="font-semibold">Validation</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
          >
            ×
          </button>
        </div>

        {/* Summary */}
        <div className="flex items-center gap-4 text-sm">
          {errors.length === 0 ? (
            <div className="flex items-center gap-2 text-green-600 dark:text-green-400">
              <CheckCircle className="w-4 h-4" />
              <span>No issues found</span>
            </div>
          ) : (
            <>
              {errorCount > 0 && (
                <div className="flex items-center gap-1 text-red-600 dark:text-red-400">
                  <AlertCircle className="w-4 h-4" />
                  <span>{errorCount}</span>
                </div>
              )}
              {warningCount > 0 && (
                <div className="flex items-center gap-1 text-amber-600 dark:text-amber-400">
                  <AlertTriangle className="w-4 h-4" />
                  <span>{warningCount}</span>
                </div>
              )}
              {infoCount > 0 && (
                <div className="flex items-center gap-1 text-blue-600 dark:text-blue-400">
                  <Info className="w-4 h-4" />
                  <span>{infoCount}</span>
                </div>
              )}
            </>
          )}
        </div>
      </div>

      {/* Error List */}
      <div className="flex-1 overflow-auto">
        {!activeContext ? (
          <div className="p-4 text-center text-slate-500 dark:text-slate-400">
            Select a context to see validation results
          </div>
        ) : errors.length === 0 ? (
          <div className="p-8 text-center">
            <CheckCircle className="w-12 h-12 text-green-500 mx-auto mb-3" />
            <p className="text-slate-600 dark:text-slate-400">
              Your model looks good!
            </p>
            <p className="text-sm text-slate-500 dark:text-slate-500 mt-1">
              No validation issues detected
            </p>
          </div>
        ) : (
          <div className="p-2 space-y-2">
            {/* Errors first */}
            {errors
              .filter((e) => e.severity === 'error')
              .map((error, i) => (
                <ValidationItem
                  key={`error-${i}`}
                  error={error}
                  context={activeContext}
                  onNavigate={handleNavigateToNode}
                  onQuickFix={() => handleQuickFix(error)}
                />
              ))}
            {/* Then warnings */}
            {errors
              .filter((e) => e.severity === 'warning')
              .map((error, i) => (
                <ValidationItem
                  key={`warning-${i}`}
                  error={error}
                  context={activeContext}
                  onNavigate={handleNavigateToNode}
                  onQuickFix={() => handleQuickFix(error)}
                />
              ))}
            {/* Then info */}
            {errors
              .filter((e) => e.severity === 'info')
              .map((error, i) => (
                <ValidationItem
                  key={`info-${i}`}
                  error={error}
                  context={activeContext}
                  onNavigate={handleNavigateToNode}
                  onQuickFix={() => handleQuickFix(error)}
                />
              ))}
          </div>
        )}
      </div>
    </div>
  );
}

interface ValidationItemProps {
  error: ValidationError;
  context: {
    nodes: Record<string, { node: { name: string } }>;
  };
  onNavigate: (nodeId: string) => void;
  onQuickFix: () => void;
}

function ValidationItem({ error, context, onNavigate, onQuickFix }: ValidationItemProps) {
  const nodeName = error.nodeId ? context.nodes[error.nodeId]?.node.name : null;
  const hasQuickFix = ['ENTITY_NO_IDENTITY', 'ENUM_NO_VARIANTS', 'ORPHAN_NODE'].includes(error.code);

  return (
    <div
      className={`p-3 rounded-lg border ${severityColors[error.severity]}`}
    >
      <div className="flex items-start gap-2">
        {severityIcons[error.severity]}
        <div className="flex-1 min-w-0">
          <p className="text-sm text-slate-800 dark:text-slate-200">
            {error.message}
          </p>
          {nodeName && (
            <button
              onClick={() => error.nodeId && onNavigate(error.nodeId)}
              className="flex items-center gap-1 mt-2 text-xs text-primary hover:underline"
            >
              <ChevronRight className="w-3 h-3" />
              Go to {nodeName}
            </button>
          )}
          {hasQuickFix && (
            <button
              onClick={onQuickFix}
              className="flex items-center gap-1 mt-2 text-xs text-emerald-600 dark:text-emerald-400 hover:underline"
            >
              <Zap className="w-3 h-3" />
              Quick fix
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

// Compact validation status for header/footer
export function ValidationStatus() {
  const { activeContextId, contexts } = useDomainStore();
  const activeContext = activeContextId ? contexts[activeContextId] : null;

  const errors = useMemo(() => {
    if (!activeContext) return [];
    return validateContext(activeContext);
  }, [activeContext]);

  const errorCount = errors.filter((e) => e.severity === 'error').length;
  const warningCount = errors.filter((e) => e.severity === 'warning').length;

  if (!activeContext) return null;

  if (errors.length === 0) {
    return (
      <div className="flex items-center gap-1 text-green-600 dark:text-green-400 text-sm">
        <CheckCircle className="w-4 h-4" />
        <span>Valid</span>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2 text-sm">
      {errorCount > 0 && (
        <div className="flex items-center gap-1 text-red-600 dark:text-red-400">
          <AlertCircle className="w-4 h-4" />
          <span>{errorCount}</span>
        </div>
      )}
      {warningCount > 0 && (
        <div className="flex items-center gap-1 text-amber-600 dark:text-amber-400">
          <AlertTriangle className="w-4 h-4" />
          <span>{warningCount}</span>
        </div>
      )}
    </div>
  );
}
