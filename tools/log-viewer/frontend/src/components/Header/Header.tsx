import { 
  statusMessage, 
  currentFile, 
  loadLogFiles, 
  loadLogFile,
  searchQuery as searchQuerySignal,
  jqFilter as jqFilterSignal,
  performSearch,
  clearSearch,
  showRaw
} from '../../store';
import { showFilterPanel, resetFilterPanel } from '../FilterPanel/FilterPanel';

export function Header() {
  const handleSearch = (e: Event) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const input = form.querySelector('input') as HTMLInputElement;
    performSearch(input.value);
  };

  const handleRefresh = () => {
    loadLogFiles();
    if (currentFile.value) {
      loadLogFile(currentFile.value);
    }
  };

  return (
    <header class="header">
      <div class="header-left">
        <h1 class="header-title">📋 Log Viewer</h1>
      </div>
      
      <form class="search-form" onSubmit={handleSearch}>
        <input 
          type="text" 
          class="search-input" 
          placeholder="Search (regex supported)..."
          value={searchQuerySignal.value}
        />
        <button type="submit" class="btn btn-primary">🔍 Search</button>
      </form>

      <button 
        class={`btn ${showFilterPanel.value ? 'btn-active' : ''}`}
        onClick={() => showFilterPanel.value = !showFilterPanel.value}
        title="Advanced Filters"
      >
        🎛️ Filters
      </button>
      
      <div class="header-filters">
        {(searchQuerySignal.value || jqFilterSignal.value) && (
          <button class="btn" onClick={() => { resetFilterPanel(); clearSearch(); }}>✕ Clear Filter</button>
        )}
        
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            checked={showRaw.value}
            onChange={(e) => showRaw.value = (e.target as HTMLInputElement).checked}
          />
          Show Raw
        </label>
      </div>
      
      <div class="header-right">
        <span class="status-text">{statusMessage.value}</span>
        <button class="btn" onClick={handleRefresh}>🔄 Refresh</button>
      </div>
    </header>
  );
}
