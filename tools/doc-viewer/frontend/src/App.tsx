import { useEffect } from 'preact/hooks';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { DocumentTabs } from './components/DocumentTabs';
import { DocViewer } from './components/DocViewer';
import { loadDocs } from './store';

export function App() {
  useEffect(() => {
    loadDocs();
  }, []);

  return (
    <div class="app">
      <Header />
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
