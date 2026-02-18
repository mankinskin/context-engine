import { activeTab, setTab, filteredEntries } from '../../store';
import type { ViewTab } from '../../types';

const tabs: { id: ViewTab; label: string; icon: string }[] = [
  { id: 'logs', label: 'Logs', icon: '📋' },
  { id: 'flow', label: 'Flow Graph', icon: '🔀' },
  { id: 'stats', label: 'Statistics', icon: '📊' },
  { id: 'code', label: 'Code', icon: '📄' },
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
            <span class="tab-icon">{tab.icon}</span>
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
