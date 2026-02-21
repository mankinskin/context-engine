import { 
  Header as SharedHeader, 
  DocumentIcon, 
  FilterIcon, 
  CloseIcon, 
  RefreshIcon,
  HomeIcon 
} from '@context-engine/viewer-api-frontend';
import { 
  showFilterPanel, 
  hasActiveFilters, 
  clearFilters, 
  loadDocs,
  openCategoryPage 
} from '../store';

export function Header() {
  const handleFilterToggle = () => {
    showFilterPanel.value = !showFilterPanel.value;
  };

  const handleRefresh = () => {
    loadDocs();
  };

  const handleHome = () => {
    openCategoryPage('page:home');
  };

  const rightContent = (
    <div class="header-actions">
      <button 
        class="btn"
        onClick={handleHome}
        title="Home"
      >
        <HomeIcon size={12} /> Home
      </button>
      
      <button 
        class={`btn ${showFilterPanel.value ? 'btn-active' : ''}`}
        onClick={handleFilterToggle}
        title="Advanced Filters"
      >
        <FilterIcon size={12} /> Filters
      </button>
      
      {hasActiveFilters.value && (
        <button class="btn" onClick={clearFilters} title="Clear all filters">
          <CloseIcon size={12} /> Clear
        </button>
      )}
      
      <button class="btn" onClick={handleRefresh} title="Refresh documentation">
        <RefreshIcon size={12} /> Refresh
      </button>
    </div>
  );

  return (
    <SharedHeader 
      title="Doc Viewer"
      icon={<DocumentIcon size={20} />}
      subtitle="context-engine documentation"
      rightContent={rightContent}
    />
  );
}
