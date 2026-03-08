import { openTabs, activeTabId, closeTab, setActiveTab } from '../store';

export function DocumentTabs() {
  const tabs = openTabs.value;
  const activeId = activeTabId.value;
  
  if (tabs.length === 0) {
    return null;
  }
  
  return (
    <div class="tab-bar">
      <div class="tabs">
        {tabs.map(tab => (
          <button
            key={tab.filename}
            class={`tab ${activeId === tab.filename ? 'active' : ''}`}
            onClick={() => setActiveTab(tab.filename)}
            title={tab.filename}
          >
            <span class="tab-icon">
              <DocIcon />
            </span>
            <span class="tab-label">{tab.title}</span>
            <span 
              class="tab-close" 
              onClick={(e) => {
                e.stopPropagation();
                closeTab(tab.filename);
              }}
            >
              <CloseIcon />
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}

function DocIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z" />
      <polyline points="14 2 14 8 20 8" />
      <line x1="16" y1="13" x2="8" y2="13" />
      <line x1="16" y1="17" x2="8" y2="17" />
    </svg>
  );
}

function CloseIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <line x1="18" y1="6" x2="6" y2="18" />
      <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  );
}
