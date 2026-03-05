import { useEffect, useState, useCallback } from 'preact/hooks';
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
import { activeTab, loadLogFiles, initUrlListener, getStateFromUrl, loadLogFile, setTab } from './store';
import { WgpuOverlay } from './components/WgpuOverlay/WgpuOverlay';
import { useGlobalKeyboard, usePanelFocus, focusedPanel } from './hooks';
import './store/theme';  // initialize theme effects on startup

export function App() {
  useGlobalKeyboard();
  const contentRef = usePanelFocus('content');
  const [mobileOpen, setMobileOpen] = useState(false);

  useEffect(() => {
    (async () => {
      await loadLogFiles();
      // Restore state from URL after file list is loaded
      const urlState = getStateFromUrl();
      if (urlState) {
        await loadLogFile(urlState.file);
        setTab(urlState.tab);
      }
      initUrlListener();
    })();
  }, []);

  const toggleMobileSidebar = useCallback(() => {
    setMobileOpen(prev => !prev);
  }, []);

  const closeMobileSidebar = useCallback(() => {
    setMobileOpen(false);
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
      <Header onMenuToggle={toggleMobileSidebar} />
      <FilterPanel />
      <div class="main-layout">
        {mobileOpen && <div class="sidebar-overlay visible" onClick={closeMobileSidebar} />}
        <Sidebar mobileOpen={mobileOpen} onMobileClose={closeMobileSidebar} />
        <main class="content">
          <TabBar />
          <div
            class={`view-container ${focusedPanel.value === 'content' ? 'focused' : ''}`}
            ref={(el: HTMLDivElement | null) => { contentRef.current = el; }}
            tabIndex={-1}
          >
            {renderContent()}
          </div>
        </main>
      </div>
    </div>
  );
}
