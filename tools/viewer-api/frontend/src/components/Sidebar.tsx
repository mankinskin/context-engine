import { JSX, ComponentChildren } from 'preact';
import { useState, useCallback, useRef } from 'preact/hooks';

export interface SidebarProps {
  /** Header title */
  title: string;
  /** Optional badge/count to display in header */
  badge?: string | number;
  /** Main content of the sidebar */
  children: ComponentChildren;
  /** Optional class name */
  class?: string;
  /** Optional loading state */
  loading?: boolean;
  /** Optional empty state message */
  emptyMessage?: string;
  /** Whether content is empty */
  isEmpty?: boolean;
  /** Enable collapse/expand toggle button */
  collapsible?: boolean;
  /** Controlled collapsed state (if provided, sidebar is controlled) */
  collapsed?: boolean;
  /** Callback when collapsed state changes */
  onCollapsedChange?: (collapsed: boolean) => void;
  /** Enable drag-to-resize handle on the right edge */
  resizable?: boolean;
  /** Initial width in px (default: 260) */
  initialWidth?: number;
  /** Min width in px (default: 180) */
  minWidth?: number;
  /** Max width in px (default: 500) */
  maxWidth?: number;
  /** Mobile overlay mode: when true, sidebar is shown as overlay with backdrop */
  mobileOpen?: boolean;
  /** Callback to close mobile overlay */
  onMobileClose?: () => void;
}

/**
 * Common sidebar shell component for viewer tools.
 * 
 * Provides a consistent sidebar layout with:
 * - Header row with title and optional badge
 * - Content area with optional loading/empty states
 * - Optional collapse/expand toggle
 * - Optional drag-to-resize handle
 */
export function Sidebar({ 
  title, 
  badge, 
  children, 
  class: className = '',
  loading = false,
  emptyMessage = 'No items found',
  isEmpty = false,
  collapsible = false,
  collapsed: controlledCollapsed,
  onCollapsedChange,
  resizable = false,
  initialWidth = 260,
  minWidth = 180,
  maxWidth = 500,
  mobileOpen,
  onMobileClose,
}: SidebarProps): JSX.Element {
  const [internalCollapsed, setInternalCollapsed] = useState(false);
  const [width, setWidth] = useState(initialWidth);

  const isCollapsed = controlledCollapsed ?? internalCollapsed;

  const toggleCollapse = useCallback(() => {
    const next = !isCollapsed;
    if (onCollapsedChange) {
      onCollapsedChange(next);
    } else {
      setInternalCollapsed(next);
    }
  }, [isCollapsed, onCollapsedChange]);

  const handleResize = useCallback((delta: number) => {
    setWidth(prev => Math.max(minWidth, Math.min(maxWidth, prev + delta)));
  }, [minWidth, maxWidth]);

  const sidebarStyle = isCollapsed
    ? { width: '0px', minWidth: '0px', overflow: 'hidden' as const }
    : resizable
      ? { width: `${width}px`, minWidth: `${minWidth}px` }
      : {};

  // Mobile overlay class
  const mobileClass = mobileOpen !== undefined
    ? (mobileOpen ? 'sidebar-mobile-open' : 'sidebar-mobile-closed')
    : '';

  return (
    <>
      {/* Backdrop for mobile overlay */}
      {mobileOpen && onMobileClose && (
        <div class="sidebar-overlay visible" onClick={onMobileClose} />
      )}
      <aside
        class={`sidebar ${isCollapsed ? 'sidebar-collapsed' : ''} ${mobileClass} ${className}`}
        style={sidebarStyle}
      >
        {!isCollapsed && (
          <>
            <div class="sidebar-header">
              <h2>{title}</h2>
              {badge !== undefined && <span class="sidebar-badge">{badge}</span>}
              {collapsible && (
                <button class="sidebar-collapse-btn" onClick={toggleCollapse} title="Collapse sidebar">
                  <CollapseIcon />
                </button>
              )}
              {onMobileClose && (
                <button class="sidebar-close-btn" onClick={onMobileClose} title="Close sidebar">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" width="16" height="16">
                    <line x1="18" y1="6" x2="6" y2="18" />
                    <line x1="6" y1="6" x2="18" y2="18" />
                  </svg>
                </button>
              )}
            </div>
            
            <div class="sidebar-content">
              {loading ? (
                <div class="sidebar-loading">Loading...</div>
              ) : isEmpty ? (
                <div class="sidebar-empty">{emptyMessage}</div>
              ) : (
                children
              )}
            </div>

            {resizable && <ResizeHandle direction="horizontal" onResize={handleResize} />}
          </>
        )}
        {isCollapsed && collapsible && (
          <button class="sidebar-expand-btn" onClick={toggleCollapse} title="Expand sidebar">
            <ExpandIcon />
          </button>
        )}
      </aside>
    </>
  );
}

// ── Resize Handle ──

export interface ResizeHandleProps {
  onResize: (delta: number) => void;
  onResizeStart?: () => void;
  direction: 'horizontal' | 'vertical';
}

export function ResizeHandle({ onResize, onResizeStart, direction }: ResizeHandleProps) {
  const isDragging = useRef(false);
  const lastPos = useRef(0);

  const handleMouseDown = useCallback((e: MouseEvent) => {
    e.preventDefault();
    isDragging.current = true;
    lastPos.current = direction === 'horizontal' ? e.clientX : e.clientY;
    document.body.style.cursor = direction === 'horizontal' ? 'col-resize' : 'row-resize';
    document.body.style.userSelect = 'none';
    
    onResizeStart?.();

    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging.current) return;
      const currentPos = direction === 'horizontal' ? e.clientX : e.clientY;
      const delta = currentPos - lastPos.current;
      lastPos.current = currentPos;
      onResize(delta);
    };

    const handleMouseUp = () => {
      isDragging.current = false;
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }, [onResize, onResizeStart, direction]);

  return (
    <div 
      class={`resize-handle resize-handle-${direction}`}
      onMouseDown={handleMouseDown}
    />
  );
}

// ── Icons ──

function CollapseIcon() {
  return (
    <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
      <polyline points="15 18 9 12 15 6" />
    </svg>
  );
}

function ExpandIcon() {
  return (
    <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
      <polyline points="9 18 15 12 9 6" />
    </svg>
  );
}
