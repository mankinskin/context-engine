import { JSX } from 'preact';

export interface Tab {
  id: string;
  label: string;
  icon?: JSX.Element;
  closeable?: boolean;
  modified?: boolean;
}

export interface TabBarProps {
  tabs: Tab[];
  activeTabId: string | null;
  onSelect: (id: string) => void;
  onClose?: (id: string) => void;
  rightContent?: JSX.Element;
}

export function TabBar({ tabs, activeTabId, onSelect, onClose, rightContent }: TabBarProps): JSX.Element {
  return (
    <div class="tab-bar">
      <div class="tabs">
        {tabs.map(tab => (
          <button
            key={tab.id}
            class={`tab ${activeTabId === tab.id ? 'active' : ''}`}
            onClick={() => onSelect(tab.id)}
            title={tab.label}
          >
            {tab.icon && <span class="tab-icon">{tab.icon}</span>}
            <span class="tab-label">{tab.label}</span>
            {tab.modified && <span class="tab-modified">•</span>}
            {tab.closeable && onClose && (
              <span 
                class="tab-close" 
                onClick={(e) => {
                  e.stopPropagation();
                  onClose(tab.id);
                }}
              >
                <CloseIcon />
              </span>
            )}
          </button>
        ))}
      </div>
      {rightContent && <div class="tab-info">{rightContent}</div>}
    </div>
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
