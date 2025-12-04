import { Header, Sidebar } from '@/components/layout';
import { Palette } from '@/components/palette';
import { Canvas } from '@/components/canvas';
import { PropertiesPanel } from '@/components/panels';

function App() {
  return (
    <div className="h-screen flex flex-col bg-slate-50 dark:bg-slate-900">
      <Header />
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />
        <Palette />
        <Canvas />
        <PropertiesPanel />
      </div>
    </div>
  );
}

export default App;
