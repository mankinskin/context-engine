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

  const IconLogs = () => (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#8b9dc3" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
      <polyline points="14 2 14 8 20 8"/>
      <line x1="16" y1="13" x2="8" y2="13"/>
      <line x1="16" y1="17" x2="8" y2="17"/>
      <line x1="10" y1="9" x2="8" y2="9"/>
    </svg>
  );

  const IconSearch = () => (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="11" cy="11" r="8"/>
      <line x1="21" y1="21" x2="16.65" y2="16.65"/>
    </svg>
  );

  const IconFilter = () => (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
    </svg>
  );

  const IconX = () => (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <line x1="18" y1="6" x2="6" y2="18"/>
      <line x1="6" y1="6" x2="18" y2="18"/>
    </svg>
  );

  const IconRefresh = () => (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="23 4 23 10 17 10"/>
      <polyline points="1 20 1 14 7 14"/>
      <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
    </svg>
  );

  return (
    <header class="header">
      <div class="header-left">
        <IconLogs />
        <h1 class="header-title">Log Viewer</h1>
      </div>
      
      <form class="search-form" onSubmit={handleSearch}>
        <input 
          type="text" 
          class="search-input" 
          placeholder="Search (regex supported)..."
          value={searchQuerySignal.value}
        />
        <button type="submit" class="btn btn-primary"><IconSearch /> Search</button>
      </form>

      <button 
        class={`btn ${showFilterPanel.value ? 'btn-active' : ''}`}
        onClick={() => showFilterPanel.value = !showFilterPanel.value}
        title="Advanced Filters"
      >
        <IconFilter /> Filters
      </button>
      
      <div class="header-filters">
        {(searchQuerySignal.value || jqFilterSignal.value) && (
          <button class="btn" onClick={() => { resetFilterPanel(); clearSearch(); }}><IconX /> Clear</button>
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
        <button class="btn" onClick={handleRefresh}><IconRefresh /> Refresh</button>
      </div>
    </header>
  );
}
