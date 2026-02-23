import { useEffect } from 'preact/hooks';
import { Header } from './components/Header/Header';
import { FilterPanel } from './components/FilterPanel/FilterPanel';
import { Sidebar } from './components/Sidebar/Sidebar';
import { TabBar } from './components/Tabs/TabBar';
import { LogViewer } from './components/LogViewer/LogViewer';
import { CodeViewer } from './components/CodeViewer/CodeViewer';
import { Stats } from './components/Stats/Stats';
import { EffectsDebug } from './components/EffectsDebug/EffectsDebug';
import { Scene3D } from './components/Scene3D/Scene3D';
import { HypergraphView } from './components/HypergraphView/HypergraphView';
import { ThemeSettings } from './components/ThemeSettings/ThemeSettings';
import { activeTab, loadLogFiles } from './store';
import { WgpuOverlay } from './components/WgpuOverlay/WgpuOverlay';
import './store/theme';  // initialize theme effects on startup

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
      case 'hypergraph':
        return <HypergraphView />;
      case 'stats':
        return <Stats />;
      case 'debug':
        return <EffectsDebug />;
      case 'scene3d':
        return <Scene3D />;
      case 'settings':
        return <ThemeSettings />;
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
