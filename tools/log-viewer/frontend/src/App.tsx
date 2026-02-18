import { useEffect } from 'preact/hooks';
import { Header } from './components/Header/Header';
import { Sidebar } from './components/Sidebar/Sidebar';
import { TabBar } from './components/Tabs/TabBar';
import { LogViewer } from './components/LogViewer/LogViewer';
import { CodeViewer } from './components/CodeViewer/CodeViewer';
import { FlowGraph } from './components/FlowGraph/FlowGraph';
import { Stats } from './components/Stats/Stats';
import { activeTab, loadLogFiles } from './store';

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
      case 'stats':
        return <Stats />;
      default:
        return <LogViewer />;
    }
  };

  return (
    <div class="app">
      <Header />
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
