import { logFiles, currentFile, loadLogFile, isLoading } from '../../store';
import { signal } from '@preact/signals';

const graphFilterOn = signal(false);

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function Sidebar() {
  const files = graphFilterOn.value
    ? logFiles.value.filter(f => f.has_graph_snapshot)
    : logFiles.value;
  const graphCount = logFiles.value.filter(f => f.has_graph_snapshot).length;

  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>Log Files</h2>
        <span class="file-count">{files.length} files</span>
      </div>

      {graphCount > 0 && (
        <button
          class={`sidebar-filter-btn ${graphFilterOn.value ? 'active' : ''}`}
          onClick={() => { graphFilterOn.value = !graphFilterOn.value; }}
          title={graphFilterOn.value ? 'Show all logs' : 'Show only logs with graph data'}
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/>
            <circle cx="6" cy="18" r="3"/><circle cx="18" cy="18" r="3"/>
            <line x1="9" y1="6" x2="15" y2="6"/><line x1="6" y1="9" x2="6" y2="15"/>
            <line x1="18" y1="9" x2="18" y2="15"/><line x1="9" y1="18" x2="15" y2="18"/>
          </svg>
          <span>Graph data ({graphCount})</span>
        </button>
      )}
      
      <div class="file-list">
        {isLoading.value && logFiles.value.length === 0 ? (
          <p class="loading">Loading...</p>
        ) : files.length === 0 ? (
          <p class="placeholder">{graphFilterOn.value ? 'No logs with graph data' : 'No log files found'}</p>
        ) : (
          files.map(file => (
            <div 
              key={file.name}
              class={`file-item ${file.name === currentFile.value ? 'active' : ''}`}
              onClick={() => loadLogFile(file.name)}
            >
              <div class="file-name" title={file.name}>
                {file.has_graph_snapshot && <span class="graph-badge" title="Contains graph snapshot">⬡</span>}
                {file.name}
              </div>
              <div class="file-meta">
                {formatSize(file.size)} • {file.modified || 'Unknown'}
              </div>
            </div>
          ))
        )}
      </div>
    </aside>
  );
}
