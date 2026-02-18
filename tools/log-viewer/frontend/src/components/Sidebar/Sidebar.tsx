import { logFiles, currentFile, loadLogFile, isLoading } from '../../store';

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function Sidebar() {
  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>Log Files</h2>
        <span class="file-count">{logFiles.value.length} files</span>
      </div>
      
      <div class="file-list">
        {isLoading.value && logFiles.value.length === 0 ? (
          <p class="loading">Loading...</p>
        ) : logFiles.value.length === 0 ? (
          <p class="placeholder">No log files found</p>
        ) : (
          logFiles.value.map(file => (
            <div 
              key={file.name}
              class={`file-item ${file.name === currentFile.value ? 'active' : ''}`}
              onClick={() => loadLogFile(file.name)}
            >
              <div class="file-name" title={file.name}>{file.name}</div>
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
