import { logFiles, currentFile, loadLogFile, isLoading } from '../../store';
import { signal } from '@preact/signals';

// Filter state: 'all' | 'graph' | 'search' | 'insert' | 'paths'
const activeFilter = signal<'all' | 'graph' | 'search' | 'insert' | 'paths'>('all');

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function Sidebar() {
  const filter = activeFilter.value;
  const files = filter === 'all' 
    ? logFiles.value
    : filter === 'graph'
    ? logFiles.value.filter(f => f.has_graph_snapshot)
    : filter === 'search'
    ? logFiles.value.filter(f => f.has_search_ops)
    : filter === 'paths'
    ? logFiles.value.filter(f => f.has_search_paths)
    : logFiles.value.filter(f => f.has_insert_ops);
  
  const graphCount = logFiles.value.filter(f => f.has_graph_snapshot).length;
  const searchCount = logFiles.value.filter(f => f.has_search_ops).length;
  const insertCount = logFiles.value.filter(f => f.has_insert_ops).length;
  const pathsCount = logFiles.value.filter(f => f.has_search_paths).length;

  const toggleFilter = (newFilter: 'all' | 'graph' | 'search' | 'insert' | 'paths') => {
    activeFilter.value = activeFilter.value === newFilter ? 'all' : newFilter;
  };

  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>Log Files</h2>
        <span class="file-count">{files.length} files</span>
      </div>

      <div class="sidebar-filters">
        {graphCount > 0 && (
          <button
            class={`sidebar-filter-btn ${filter === 'graph' ? 'active' : ''}`}
            onClick={() => toggleFilter('graph')}
            title={filter === 'graph' ? 'Show all logs' : 'Show only logs with graph data'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/>
              <circle cx="6" cy="18" r="3"/><circle cx="18" cy="18" r="3"/>
              <line x1="9" y1="6" x2="15" y2="6"/><line x1="6" y1="9" x2="6" y2="15"/>
              <line x1="18" y1="9" x2="18" y2="15"/><line x1="9" y1="18" x2="15" y2="18"/>
            </svg>
            <span>Graph ({graphCount})</span>
          </button>
        )}
        {searchCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-search ${filter === 'search' ? 'active' : ''}`}
            onClick={() => toggleFilter('search')}
            title={filter === 'search' ? 'Show all logs' : 'Show only logs with search ops'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
            </svg>
            <span>Search ({searchCount})</span>
          </button>
        )}
        {insertCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-insert ${filter === 'insert' ? 'active' : ''}`}
            onClick={() => toggleFilter('insert')}
            title={filter === 'insert' ? 'Show all logs' : 'Show only logs with insert ops'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            <span>Insert ({insertCount})</span>
          </button>
        )}
        {pathsCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-paths ${filter === 'paths' ? 'active' : ''}`}
            onClick={() => toggleFilter('paths')}
            title={filter === 'paths' ? 'Show all logs' : 'Show only logs with search paths'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="4 7 4 4 20 4 20 7"/><line x1="12" y1="21" x2="12" y2="8"/>
              <polyline points="8 12 12 8 16 12"/>
            </svg>
            <span>Paths ({pathsCount})</span>
          </button>
        )}
      </div>
      
      <div class="file-list">
        {isLoading.value && logFiles.value.length === 0 ? (
          <p class="loading">Loading...</p>
        ) : files.length === 0 ? (
          <p class="placeholder">{filter !== 'all' ? `No logs with ${filter} data` : 'No log files found'}</p>
        ) : (
          files.map(file => (
            <div 
              key={file.name}
              class={`file-item ${file.name === currentFile.value ? 'active' : ''}`}
              onClick={() => loadLogFile(file.name)}
            >
              <div class="file-name" title={file.name}>
                {file.has_graph_snapshot && <span class="graph-badge" title="Contains graph snapshot">⬡</span>}
                {file.has_search_ops && <span class="search-badge" title="Contains search ops">🔍</span>}
                {file.has_insert_ops && <span class="insert-badge" title="Contains insert ops">+</span>}
                {file.has_search_paths && <span class="paths-badge" title="Contains search paths">⤴</span>}
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
