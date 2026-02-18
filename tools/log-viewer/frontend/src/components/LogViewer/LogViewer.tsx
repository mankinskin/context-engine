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

export function LogViewer() {
  if (!currentFile.value) {
    return (
      <div class="log-viewer empty">
        <div class="placeholder-message">
          <span class="placeholder-icon">📁</span>
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
      <div class="log-entries">
        {filteredEntries.value.map(entry => (
          <LogEntryRow
            key={entry.line_number}
            entry={entry}
            showRaw={showRaw.value}
            searchQuery={searchQuery.value}
            isSelected={selectedEntry.value?.line_number === entry.line_number}
            onSelect={() => selectEntry(entry)}
          />
        ))}
      </div>
    </div>
  );
}
