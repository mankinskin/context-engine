import { 
  SearchIcon, 
  FilterIcon, 
  CloseIcon, 
  RefreshIcon,
  LogIcon 
} from '@context-engine/viewer-api-frontend';
import { 
  statusMessage, 
  currentFile, 
  loadLogFiles, 
  loadLogFile,
  searchQuery as searchQuerySignal,
  jqFilter as jqFilterSignal,
  performSearch,
  clearSearch
} from '../../store';
import { showFilterPanel, resetFilterPanel } from '../FilterPanel/FilterPanel';
import { fxEnabled } from '../WgpuOverlay/WgpuOverlay';

interface HeaderProps {
  onMenuToggle?: () => void;
}

export function Header({ onMenuToggle }: HeaderProps) {
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
        {onMenuToggle && (
          <button class="sidebar-hamburger" onClick={onMenuToggle} title="Toggle sidebar">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" width="20" height="20">
              <line x1="3" y1="6" x2="21" y2="6" />
              <line x1="3" y1="12" x2="21" y2="12" />
              <line x1="3" y1="18" x2="21" y2="18" />
            </svg>
          </button>
        )}
        <LogIcon size={14} color="#8b9dc3" />
        <h1 class="header-title">Log Viewer</h1>
      </div>
      
      <form class="search-form" onSubmit={handleSearch}>
        <input 
          type="text" 
          class="search-input" 
          placeholder="Search (regex supported)..."
          value={searchQuerySignal.value}
        />
        <button type="submit" class="btn btn-primary"><SearchIcon size={12} /> Search</button>
      </form>

      <button 
        class={`btn ${showFilterPanel.value ? 'btn-active' : ''}`}
        onClick={() => showFilterPanel.value = !showFilterPanel.value}
        title="Advanced Filters"
      >
        <FilterIcon size={12} /> Filters
      </button>
      
      <div class="header-filters">
        {(searchQuerySignal.value || jqFilterSignal.value) && (
          <button class="btn" onClick={() => { resetFilterPanel(); clearSearch(); }}><CloseIcon size={12} /> Clear</button>
        )}
      </div>
      
      <div class="header-right">
        <span class="status-text">{statusMessage.value}</span>
        <button
          class={`btn btn-gpu ${fxEnabled.value ? 'btn-active' : ''}`}
          title={fxEnabled.value ? 'Disable visual effects (particles, smoke, CRT)' : 'Enable visual effects (particles, smoke, CRT)'}
          onClick={() => fxEnabled.value = !fxEnabled.value}
        >
          {fxEnabled.value ? '✦' : '✧'} FX
        </button>
        <button class="btn" onClick={handleRefresh}><RefreshIcon size={12} /> Refresh</button>
      </div>
    </header>
  );
}
