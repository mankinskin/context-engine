import { useEffect, useState, useCallback } from '@context-engine/viewer-api-frontend';
import { Sidebar as SharedSidebar } from '@context-engine/viewer-api-frontend';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { Breadcrumbs } from './components/Breadcrumbs';
import { DocViewer } from './components/DocViewer';
import { FilterPanel } from './components/FilterPanel';
import { FileViewer } from './components/FileViewer';
import { loadDocs, initUrlListener, codeViewerFile, closeCodeViewer, totalDocs, isLoading, docTree } from './store';
import '@context-engine/viewer-api-frontend/styles/code-viewer.css';

export function App() {
  const [mobileOpen, setMobileOpen] = useState(false);

  useEffect(() => {
    loadDocs();
    initUrlListener();
  }, []);

  const showCodeViewer = codeViewerFile.value !== null;

  const toggleMobileSidebar = useCallback(() => {
    setMobileOpen(prev => !prev);
  }, []);

  const closeMobileSidebar = useCallback(() => {
    setMobileOpen(false);
  }, []);

  return (
    <div class="app">
      <Header onMenuToggle={toggleMobileSidebar} />
      <FilterPanel />
      <div class="main-layout">
        <SharedSidebar
          title="Documentation"
          badge={totalDocs.value}
          collapsible
          resizable
          initialWidth={280}
          loading={isLoading.value && docTree.value.length === 0}
          isEmpty={!isLoading.value && docTree.value.length === 0}
          emptyMessage="No documents found"
          mobileOpen={mobileOpen}
          onMobileClose={closeMobileSidebar}
        >
          <Sidebar />
        </SharedSidebar>
        <main class="content">
          <Breadcrumbs />
          <div class="content-panels">
            {showCodeViewer ? (
              <div class="code-panel full-width">
                <div class="code-panel-header">
                  <span class="code-panel-title">{codeViewerFile.value}</span>
                  <button class="code-panel-close" onClick={closeCodeViewer} title="Close">×</button>
                </div>
                <FileViewer />
              </div>
            ) : (
              <DocViewer />
            )}
          </div>
        </main>
      </div>
    </div>
  );
}
