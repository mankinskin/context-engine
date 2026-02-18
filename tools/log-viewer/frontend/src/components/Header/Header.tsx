import { 
  statusMessage, 
  currentFile, 
  loadLogFiles, 
  loadLogFile,
  searchQuery as searchQuerySignal,
  performSearch,
  levelFilter,
  typeFilter,
  showRaw
} from '../../store';
import type { LogLevel, EventType } from '../../types';

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
      
      <div class="header-filters">
        <select 
          class="filter-select"
          value={levelFilter.value}
          onChange={(e) => levelFilter.value = (e.target as HTMLSelectElement).value as LogLevel | ''}
        >
          <option value="">All Levels</option>
          <option value="TRACE">TRACE</option>
          <option value="DEBUG">DEBUG</option>
          <option value="INFO">INFO</option>
          <option value="WARN">WARN</option>
          <option value="ERROR">ERROR</option>
        </select>
        
        <select 
          class="filter-select"
          value={typeFilter.value}
          onChange={(e) => typeFilter.value = (e.target as HTMLSelectElement).value as EventType | ''}
        >
          <option value="">All Types</option>
          <option value="event">Event</option>
          <option value="span_enter">Span Enter</option>
          <option value="span_exit">Span Exit</option>
        </select>
        
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
