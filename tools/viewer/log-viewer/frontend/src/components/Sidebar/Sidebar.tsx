import { logFiles, currentFile, loadLogFile, isLoading } from '../../store';
import { signal } from '@preact/signals';
import { usePanelFocus, focusedPanel } from '../../hooks';
import { FileTree, ResizeHandle, type TreeNode, type FilterOption } from '@context-engine/viewer-api-frontend';
import { buildFileTree, CATEGORIES } from '../../store/fileTree';
import type { LogFile } from '../../types';
import { useState, useMemo, useCallback, useRef } from 'preact/hooks';

// Filter state: 'all' | 'graph' | 'search' | 'insert' | 'paths'
const activeFilter = signal<string | null>(null);

interface SidebarProps {
  mobileOpen?: boolean;
  onMobileClose?: () => void;
  resizeRightEdge?: boolean;
}

export function Sidebar({
  mobileOpen,
  onMobileClose,
  resizeRightEdge = true,
}: SidebarProps) {
  const filter = activeFilter.value;
  const allFiles = logFiles.value;
  const [width, setWidth] = useState(280);
  const sidebarRef = useRef<HTMLElement | null>(null);
  const liveWidthRef = useRef(280);

  const totalCount = allFiles.length;

  // Build filter options from CATEGORIES
  const filterOpts = useMemo<FilterOption[]>(() =>
    CATEGORIES.map(cat => ({
      key: cat.id,
      label: cat.label,
      icon: cat.icon,
      count: allFiles.filter(cat.filter).length,
      activeColor: cat.color,
    })),
    [allFiles],
  );

  const handleFilterChange = useCallback((key: string | null) => {
    activeFilter.value = key;
  }, []);

  // Build tree from file list
  const treeNodes = useMemo(() => buildFileTree(allFiles), [allFiles]);

  // Controlled expansion: when filter is active, auto-expand that category
  const [expandedSet, setExpandedSet] = useState<Set<string>>(() => new Set<string>());

  // When filter changes, auto-expand the matching category
  const effectiveExpanded = useMemo(() => {
    if (!filter) return expandedSet;
    const next = new Set(expandedSet);
    next.add(filter);
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
    if (!filter) return treeNodes;
    // Show only the matching category folder
    return treeNodes.filter(n => n.id === filter);
  }, [treeNodes, filter]);

  // Determine selected node ID based on currentFile + active filter
  const selectedIds = useMemo(() => {
    if (!currentFile.value) return undefined;
    if (filter) return `${filter}/${currentFile.value}`;
    return `file-${currentFile.value}`;
  }, [currentFile.value, filter]);

  const handleSelect = useCallback((node: TreeNode<LogFile>) => {
    if (node.data) {
      loadLogFile(node.data.name);
      onMobileClose?.();
    }
  }, [onMobileClose]);

  usePanelFocus('sidebar');

  const mobileClass = mobileOpen !== undefined
    ? (mobileOpen ? 'sidebar-mobile-open' : 'sidebar-mobile-closed')
    : '';

  const onResizeStart = useCallback(() => {
    const el = sidebarRef.current;
    if (!el) return;
    liveWidthRef.current = Math.max(0, el.getBoundingClientRect().width);
  }, []);

  const onResizeSidebar = useCallback((delta: number) => {
    const el = sidebarRef.current;
    if (!el) return;
    const next = Math.max(0, liveWidthRef.current + delta);
    liveWidthRef.current = next;
    el.style.width = `${next}px`;
  }, []);

  const onResizeEnd = useCallback(() => {
    setWidth(liveWidthRef.current);
  }, []);

  // Map filter key back to human label for empty message
  const filterLabel = filter
    ? CATEGORIES.find(c => c.id === filter)?.label?.toLowerCase() ?? filter
    : null;

  return (
    <aside ref={sidebarRef as any} class={`sidebar ${mobileClass}`} style={{ width: `${width}px` }}>
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

      <FileTree<LogFile>
        class={`sidebar-file-tree ${focusedPanel.value === 'sidebar' ? 'focused' : ''}`}
        nodes={filteredNodes}
        selectedId={selectedIds}
        onSelect={handleSelect}
        expanded={effectiveExpanded}
        onToggle={handleToggle}
        loading={isLoading.value && allFiles.length === 0}
        emptyMessage={
          filterLabel
            ? `No logs with ${filterLabel} data`
            : 'No log files found'
        }
        filterOptions={filterOpts}
        activeFilter={filter}
        onFilterChange={handleFilterChange}
      />

      {resizeRightEdge && (
        <ResizeHandle
          direction="horizontal"
          edge="right"
          onResizeStart={onResizeStart}
          onResize={onResizeSidebar}
          onResizeEnd={onResizeEnd}
        />
      )}
    </aside>
  );
}
