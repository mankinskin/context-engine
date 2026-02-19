import { useState } from 'preact/hooks';
import { 
  filteredEntries, 
  showRaw, 
  searchQuery, 
  selectedEntry, 
  selectEntry,
  isLoading,
  currentFile 
} from '../../store';
import { LogEntryRow } from './LogEntryRow';
import { ChevronDown, ChevronRight } from '../Icons';

// Minimal folder icon
function FolderIcon({ size = 32, color = 'currentColor' }: { size?: number; color?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 32 32" fill="none" style={{ opacity: 0.5 }}>
      <path d="M4 8C4 6.9 4.9 6 6 6H12L14 9H26C27.1 9 28 9.9 28 11V24C28 25.1 27.1 26 26 26H6C4.9 26 4 25.1 4 24V8Z" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  );
}

export function LogViewer() {
  const [expandAll, setExpandAll] = useState<boolean>(false);
  if (!currentFile.value) {
    return (
      <div class="log-viewer empty">
        <div class="placeholder-message">
          <FolderIcon size={32} />
          <p>Select a log file to view</p>
        </div>
      </div>
    );
  }

  if (isLoading.value) {
    return (
      <div class="log-viewer loading">
        <div class="spinner"></div>
        <p>Loading...</p>
      </div>
    );
  }

  if (filteredEntries.value.length === 0) {
    return (
      <div class="log-viewer empty">
        <div class="placeholder-message">
          <span class="placeholder-icon">🔍</span>
          <p>No entries match the current filters</p>
        </div>
      </div>
    );
  }

  return (
    <div class="log-viewer">
      <div class="log-viewer-toolbar">
        <span class="toolbar-count">{filteredEntries.value.length} entries</span>
        <button class="expand-toggle" onClick={() => setExpandAll(!expandAll)} title={expandAll ? "Collapse all" : "Expand all"}>
          {expandAll ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
        </button>
      </div>
      <div class="log-entries">
        {filteredEntries.value.map(entry => (
          <LogEntryRow
            key={entry.line_number}
            entry={entry}
            showRaw={showRaw.value}
            searchQuery={searchQuery.value}
            isSelected={selectedEntry.value?.line_number === entry.line_number}
            onSelect={() => selectEntry(entry)}
            expandAll={expandAll}
          />
        ))}
      </div>
    </div>
  );
}
