import { useMemo } from 'preact/hooks';
import { activeDoc, isActiveTabLoading, error, openTabs } from '../store';
import { marked } from 'marked';

// Configure marked for safe rendering
marked.setOptions({
  gfm: true,
  breaks: false,
});

export function DocViewer() {
  const doc = activeDoc.value;
  const isLoading = isActiveTabLoading.value;
  const hasOpenTabs = openTabs.value.length > 0;
  
  const htmlContent = useMemo(() => {
    if (!doc?.body) return '';
    return marked.parse(doc.body) as string;
  }, [doc?.body]);
  
  if (isLoading) {
    return (
      <div class="doc-viewer">
        <div class="loading">
          <svg class="spinner" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" stroke-opacity="0.25" />
            <path d="M12 2a10 10 0 0 1 10 10" stroke-linecap="round" />
          </svg>
          <span>Loading...</span>
        </div>
      </div>
    );
  }
  
  if (error.value && hasOpenTabs) {
    return (
      <div class="doc-viewer">
        <div class="empty-state">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="8" x2="12" y2="12" />
            <line x1="12" y1="16" x2="12.01" y2="16" />
          </svg>
          <span>{error.value}</span>
        </div>
      </div>
    );
  }
  
  if (!doc) {
    return (
      <div class="doc-viewer">
        <div class="empty-state">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z" />
            <polyline points="14 2 14 8 20 8" />
          </svg>
          <span>Select a document to view</span>
        </div>
      </div>
    );
  }
  
  return (
    <div class="doc-viewer">
      <div class="doc-header">
        <div class="doc-header-info">
          <h1 class="doc-title">{doc.title}</h1>
          <div class="doc-meta">
            <span class="doc-date">{formatDate(doc.date)}</span>
            {doc.tags.length > 0 && (
              <div class="doc-tags">
                {doc.tags.map(tag => (
                  <span key={tag} class="doc-tag">#{tag}</span>
                ))}
              </div>
            )}
            {doc.status && (
              <span class="doc-tag">{doc.status}</span>
            )}
          </div>
        </div>
      </div>
      <div class="doc-body">
        <div 
          class="markdown-body" 
          dangerouslySetInnerHTML={{ __html: htmlContent }}
        />
      </div>
    </div>
  );
}

function formatDate(date: string): string {
  if (!date || date.length !== 8) return date;
  // Convert YYYYMMDD to YYYY-MM-DD
  return `${date.slice(0, 4)}-${date.slice(4, 6)}-${date.slice(6, 8)}`;
}
