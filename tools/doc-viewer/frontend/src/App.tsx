import { useEffect } from 'preact/hooks';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { Breadcrumbs } from './components/Breadcrumbs';
import { DocViewer } from './components/DocViewer';
import { FilterPanel } from './components/FilterPanel';
import { loadDocs, initUrlListener } from './store';

export function App() {
  useEffect(() => {
    loadDocs();
    initUrlListener();
  }, []);

  return (
    <div class="app">
      <Header />
      <FilterPanel />
      <div class="main-layout">
        <Sidebar />
        <main class="content">
          <Breadcrumbs />
          <DocViewer />
        </main>
      </div>
    </div>
  );
}
