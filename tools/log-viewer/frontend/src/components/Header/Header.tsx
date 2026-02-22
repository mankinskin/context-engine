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
  clearSearch,
  showRaw
} from '../../store';
import { showFilterPanel, resetFilterPanel } from '../FilterPanel/FilterPanel';
import { gpuOverlayEnabled } from '../WgpuOverlay/WgpuOverlay';

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
        <button
          class={`btn btn-gpu ${gpuOverlayEnabled.value ? 'btn-active' : ''}`}
          title={gpuOverlayEnabled.value ? 'Disable GPU overlay (WebGPU / wgpu WGSL shaders)' : 'Enable GPU overlay (WebGPU / wgpu WGSL shaders)'}
          onClick={() => gpuOverlayEnabled.value = !gpuOverlayEnabled.value}
        >
          {gpuOverlayEnabled.value ? '⬢' : '⬡'} GPU
        </button>
        <button class="btn" onClick={handleRefresh}><RefreshIcon size={12} /> Refresh</button>
      </div>
    </header>
  );
}
