import { useEffect } from 'preact/hooks';
import { Header } from './components/Header/Header';
import { FilterPanel } from './components/FilterPanel/FilterPanel';
import { Sidebar } from './components/Sidebar/Sidebar';
import { TabBar } from './components/Tabs/TabBar';
import { LogViewer } from './components/LogViewer/LogViewer';
import { CodeViewer } from './components/CodeViewer/CodeViewer';
import { FlowGraph } from './components/FlowGraph/FlowGraph';
import { Stats } from './components/Stats/Stats';
import { EffectsDebug } from './components/EffectsDebug/EffectsDebug';
import { Scene3D } from './components/Scene3D/Scene3D';
import { HypergraphView } from './components/HypergraphView/HypergraphView';
import { activeTab, loadLogFiles } from './store';
import { WgpuOverlay } from './components/WgpuOverlay/WgpuOverlay';

export function App() {
  useEffect(() => {
    loadLogFiles();
  }, []);

  const renderContent = () => {
    switch (activeTab.value) {
      case 'logs':
        return <LogViewer />;
      case 'code':
        return <CodeViewer />;
      case 'flow':
        return <FlowGraph />;
      case 'hypergraph':
        return <HypergraphView />;
      case 'stats':
        return <Stats />;
      case 'debug':
        return <EffectsDebug />;
      case 'scene3d':
        return <Scene3D />;
      default:
        return <LogViewer />;
    }
  };

  return (
    <div class="app">
      <WgpuOverlay />
      <Header />
      <FilterPanel />
      <div class="main-layout">
        <Sidebar />
        <main class="content">
          <TabBar />
          <div class="view-container">
            {renderContent()}
          </div>
        </main>
      </div>
    </div>
  );
}
