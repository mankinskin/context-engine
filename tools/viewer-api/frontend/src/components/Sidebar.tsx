import { JSX, ComponentChildren } from 'preact';

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
}

/**
 * Common sidebar shell component for viewer tools.
 * 
 * Provides a consistent sidebar layout with:
 * - Header row with title and optional badge
 * - Content area with optional loading/empty states
 */
export function Sidebar({ 
  title, 
  badge, 
  children, 
  class: className = '',
  loading = false,
  emptyMessage = 'No items found',
  isEmpty = false
}: SidebarProps): JSX.Element {
  return (
    <aside class={`sidebar ${className}`}>
      <div class="sidebar-header">
        <h2>{title}</h2>
        {badge !== undefined && <span class="sidebar-badge">{badge}</span>}
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
    </aside>
  );
}
