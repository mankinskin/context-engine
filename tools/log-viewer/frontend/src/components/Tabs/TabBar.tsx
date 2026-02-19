import { activeTab, setTab, filteredEntries } from '../../store';
import type { ViewTab } from '../../types';
import { ListIcon } from '../Icons';
import type { JSX } from 'preact';

// SVG icons for tabs
function FlowIcon({ size = 14, color = 'currentColor' }: { size?: number; color?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 14 14" fill="none">
      <path d="M2 4H6L8 7L6 10H2" stroke={color} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M8 7H12" stroke={color} stroke-width="1.2" stroke-linecap="round"/>
      <circle cx="12" cy="7" r="1.5" stroke={color} stroke-width="1.2"/>
    </svg>
  );
}

function StatsIcon({ size = 14, color = 'currentColor' }: { size?: number; color?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 14 14" fill="none">
      <rect x="2" y="8" width="2.5" height="4" rx="0.5" stroke={color} stroke-width="1.2"/>
      <rect x="5.75" y="5" width="2.5" height="7" rx="0.5" stroke={color} stroke-width="1.2"/>
      <rect x="9.5" y="2" width="2.5" height="10" rx="0.5" stroke={color} stroke-width="1.2"/>
    </svg>
  );
}

function FileIcon({ size = 14, color = 'currentColor' }: { size?: number; color?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 14 14" fill="none">
      <path d="M3 2.5C3 1.95 3.45 1.5 4 1.5H8L11 4.5V11.5C11 12.05 10.55 12.5 10 12.5H4C3.45 12.5 3 12.05 3 11.5V2.5Z" stroke={color} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M8 1.5V4.5H11" stroke={color} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  );
}

const tabs: { id: ViewTab; label: string; icon: () => JSX.Element }[] = [
  { id: 'logs', label: 'Logs', icon: () => <ListIcon size={14} /> },
  { id: 'flow', label: 'Flow Graph', icon: () => <FlowIcon size={14} /> },
  { id: 'stats', label: 'Statistics', icon: () => <StatsIcon size={14} /> },
  { id: 'code', label: 'Code', icon: () => <FileIcon size={14} /> },
];

export function TabBar() {
  return (
    <div class="tab-bar">
      <div class="tabs">
        {tabs.map(tab => (
          <button
            key={tab.id}
            class={`tab ${activeTab.value === tab.id ? 'active' : ''}`}
            onClick={() => setTab(tab.id)}
          >
            <span class="tab-icon">{tab.icon()}</span>
            <span class="tab-label">{tab.label}</span>
          </button>
        ))}
      </div>
      
      <div class="tab-info">
        <span class="entry-count">{filteredEntries.value.length} entries</span>
      </div>
    </div>
  );
}
