import { useState } from 'react';
import { Header, Sidebar } from '@/components/layout';
import { Palette } from '@/components/palette';
import { Canvas } from '@/components/canvas';
import { PropertiesPanel } from '@/components/panels';
import { ValidationPanel } from '@/components/validation';

function App() {
  const [showValidation, setShowValidation] = useState(false);

  return (
    <div className="h-screen flex flex-col bg-slate-50 dark:bg-slate-900">
      <Header onValidationToggle={() => setShowValidation(!showValidation)} />
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />
        <Palette />
        <Canvas />
        <PropertiesPanel />
        <ValidationPanel
          isOpen={showValidation}
          onClose={() => setShowValidation(false)}
        />
      </div>
    </div>
  );
}

export default App;
