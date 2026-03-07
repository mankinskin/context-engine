import { logFiles, currentFile, loadLogFile, isLoading } from '../../store';
import { signal } from '@preact/signals';
import { usePanelFocus, focusedPanel } from '../../hooks';
import { TreeView, type TreeNode } from '@context-engine/viewer-api-frontend';
import { buildFileTree, getCategoryIdForFilter } from '../../store/fileTree';
import type { LogFile } from '../../types';
import { useState, useMemo, useCallback } from 'preact/hooks';

// Filter state: 'all' | 'graph' | 'search' | 'insert' | 'paths'
const activeFilter = signal<'all' | 'graph' | 'search' | 'insert' | 'paths'>('all');

interface SidebarProps {
  mobileOpen?: boolean;
  onMobileClose?: () => void;
}

export function Sidebar({ mobileOpen, onMobileClose }: SidebarProps) {
  const filter = activeFilter.value;
  const allFiles = logFiles.value;

  // Compute counts for filter buttons
  const graphCount = allFiles.filter(f => f.has_graph_snapshot).length;
  const searchCount = allFiles.filter(f => f.has_search_ops).length;
  const insertCount = allFiles.filter(f => f.has_insert_ops).length;
  const pathsCount = allFiles.filter(f => f.has_search_paths).length;
  const totalCount = allFiles.length;

  const toggleFilter = (newFilter: 'all' | 'graph' | 'search' | 'insert' | 'paths') => {
    activeFilter.value = activeFilter.value === newFilter ? 'all' : newFilter;
  };

  // Build tree from file list
  const treeNodes = useMemo(() => buildFileTree(allFiles), [allFiles]);

  // Controlled expansion: when filter is active, auto-expand that category
  const [expandedSet, setExpandedSet] = useState<Set<string>>(() => new Set<string>());

  // When filter changes, auto-expand the matching category
  const effectiveExpanded = useMemo(() => {
    if (filter === 'all') return expandedSet;
    const catId = getCategoryIdForFilter(filter);
    if (!catId) return expandedSet;
    const next = new Set(expandedSet);
    next.add(catId);
    return next;
  }, [filter, expandedSet]);

  const handleToggle = useCallback((id: string) => {
    setExpandedSet(prev => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  // Filter tree nodes when a filter is active
  const filteredNodes = useMemo(() => {
    if (filter === 'all') return treeNodes;
    const catId = getCategoryIdForFilter(filter);
    if (!catId) return treeNodes;
    // Show only the matching category folder
    return treeNodes.filter(n => n.id === catId);
  }, [treeNodes, filter]);

  // Determine selected node ID based on currentFile + active filter
  const selectedIds = useMemo(() => {
    if (!currentFile.value) return undefined;
    // Check if any category folder reference matches
    const catId = getCategoryIdForFilter(filter);
    if (catId) return `${catId}/${currentFile.value}`;
    return `file-${currentFile.value}`;
  }, [currentFile.value, filter]);

  const handleSelect = useCallback((node: TreeNode<LogFile>) => {
    if (node.data) {
      loadLogFile(node.data.name);
      onMobileClose?.();
    }
  }, [onMobileClose]);

  const panelRef = usePanelFocus('sidebar');

  const handleMouseEnter = () => {
    focusedPanel.value = 'sidebar';
    panelRef.current?.focus({ preventScroll: true });
  };

  const mobileClass = mobileOpen !== undefined
    ? (mobileOpen ? 'sidebar-mobile-open' : 'sidebar-mobile-closed')
    : '';

  return (
    <aside class={`sidebar ${mobileClass}`}>
      <div class="sidebar-header">
        <h2>Log Files</h2>
        <span class="file-count">{totalCount} files</span>
        {onMobileClose && (
          <button class="sidebar-close-btn" onClick={onMobileClose} title="Close sidebar">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" width="16" height="16">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        )}
      </div>

      <div class="sidebar-filters">
        {graphCount > 0 && (
          <button
            class={`sidebar-filter-btn ${filter === 'graph' ? 'active' : ''}`}
            onClick={() => toggleFilter('graph')}
            title={filter === 'graph' ? 'Show all logs' : 'Show only logs with graph data'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/>
              <circle cx="6" cy="18" r="3"/><circle cx="18" cy="18" r="3"/>
              <line x1="9" y1="6" x2="15" y2="6"/><line x1="6" y1="9" x2="6" y2="15"/>
              <line x1="18" y1="9" x2="18" y2="15"/><line x1="9" y1="18" x2="15" y2="18"/>
            </svg>
            <span>Graph ({graphCount})</span>
          </button>
        )}
        {searchCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-search ${filter === 'search' ? 'active' : ''}`}
            onClick={() => toggleFilter('search')}
            title={filter === 'search' ? 'Show all logs' : 'Show only logs with search ops'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
            </svg>
            <span>Search ({searchCount})</span>
          </button>
        )}
        {insertCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-insert ${filter === 'insert' ? 'active' : ''}`}
            onClick={() => toggleFilter('insert')}
            title={filter === 'insert' ? 'Show all logs' : 'Show only logs with insert ops'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            <span>Insert ({insertCount})</span>
          </button>
        )}
        {pathsCount > 0 && (
          <button
            class={`sidebar-filter-btn filter-paths ${filter === 'paths' ? 'active' : ''}`}
            onClick={() => toggleFilter('paths')}
            title={filter === 'paths' ? 'Show all logs' : 'Show only logs with search paths'}
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="4 7 4 4 20 4 20 7"/><line x1="12" y1="21" x2="12" y2="8"/>
              <polyline points="8 12 12 8 16 12"/>
            </svg>
            <span>Paths ({pathsCount})</span>
          </button>
        )}
      </div>
      
      <div
        class={`file-list ${focusedPanel.value === 'sidebar' ? 'focused' : ''}`}
        ref={panelRef as any}
        tabIndex={0}
        onMouseEnter={handleMouseEnter}
      >
        {isLoading.value && allFiles.length === 0 ? (
          <p class="loading">Loading...</p>
        ) : filteredNodes.length === 0 ? (
          <p class="placeholder">{filter !== 'all' ? `No logs with ${filter} data` : 'No log files found'}</p>
        ) : (
          <TreeView<LogFile>
            nodes={filteredNodes}
            selectedId={selectedIds}
            onSelect={handleSelect}
            expanded={effectiveExpanded}
            onToggle={handleToggle}
          />
        )}
      </div>
    </aside>
  );
}
