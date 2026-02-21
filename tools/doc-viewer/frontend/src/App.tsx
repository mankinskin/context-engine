import { useEffect, useState, useCallback } from '@context-engine/viewer-api-frontend';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { Breadcrumbs } from './components/Breadcrumbs';
import { DocViewer } from './components/DocViewer';
import { FilterPanel } from './components/FilterPanel';
import { ResizeHandle } from './components/ResizeHandle';
import { CodeViewer } from '@context-engine/viewer-api-frontend';
import { loadDocs, initUrlListener, codeViewerFile, codeViewerContent, codeViewerLine, closeCodeViewer } from './store';
import '@context-engine/viewer-api-frontend/styles/code-viewer.css';

const MIN_SIDEBAR_WIDTH = 180;
const MAX_SIDEBAR_WIDTH = 500;

export function App() {
  const [sidebarWidth, setSidebarWidth] = useState(280);

  useEffect(() => {
    loadDocs();
    initUrlListener();
  }, []);

  const showCodeViewer = codeViewerFile.value !== null;

  const handleSidebarResize = useCallback((delta: number) => {
    setSidebarWidth(w => Math.min(MAX_SIDEBAR_WIDTH, Math.max(MIN_SIDEBAR_WIDTH, w + delta)));
  }, []);

  return (
    <div class="app">
      <Header />
      <FilterPanel />
      <div class="main-layout">
        <aside class="sidebar" style={{ width: `${sidebarWidth}px` }}>
          <Sidebar />
        </aside>
        <ResizeHandle direction="horizontal" onResize={handleSidebarResize} />
        <main class="content">
          <Breadcrumbs />
          <div class="content-panels">
            {showCodeViewer ? (
              <div class="code-panel full-width">
                <div class="code-panel-header">
                  <span class="code-panel-title">{codeViewerFile.value}</span>
                  <button class="code-panel-close" onClick={closeCodeViewer} title="Close">×</button>
                </div>
                <CodeViewer 
                  file={codeViewerFile}
                  content={codeViewerContent}
                  highlightLine={codeViewerLine}
                  placeholderMessage="Select a source file"
                  placeholderIcon="📄"
                />
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
