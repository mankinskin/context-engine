import { useEffect } from 'preact/hooks';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { DocumentTabs } from './components/DocumentTabs';
import { DocViewer } from './components/DocViewer';
import { FilterPanel } from './components/FilterPanel';
import { loadDocs } from './store';

export function App() {
  useEffect(() => {
    loadDocs();
  }, []);

  return (
    <div class="app">
      <Header />
      <FilterPanel />
      <div class="main-layout">
        <Sidebar />
        <main class="content">
          <DocumentTabs />
          <DocViewer />
        </main>
      </div>
    </div>
  );
}
