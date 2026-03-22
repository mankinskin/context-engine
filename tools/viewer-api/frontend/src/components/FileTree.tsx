import { JSX } from 'preact';
import { TreeView, type TreeNode, type TreeViewProps } from './TreeView';

export interface FileTreeProps<T = unknown> {
  nodes: TreeNode<T>[];
  selectedId?: string;
  onSelect?: (node: TreeNode<T>) => void;
  onContextMenu?: (node: TreeNode<T>, event: MouseEvent) => void;
  defaultExpanded?: string[];
  expanded?: Set<string>;
  onToggle?: (id: string) => void;
  loading?: boolean;
  emptyMessage?: string;
  class?: string;
}

/**
 * Shared file tree shell for viewer tools.
 *
 * This wraps `TreeView` with common loading/empty states so viewers can keep
 * their own filtering/data logic while sharing a consistent tree container.
 */
export function FileTree<T = unknown>({
  nodes,
  selectedId,
  onSelect,
  onContextMenu,
  defaultExpanded = [],
  expanded,
  onToggle,
  loading = false,
  emptyMessage = 'No files found',
  class: className = '',
}: FileTreeProps<T>): JSX.Element {
  const treeProps: TreeViewProps<T> = {
    nodes,
    selectedId,
    onSelect,
    onContextMenu,
    defaultExpanded,
    expanded,
    onToggle,
  };

  return (
    <div class={`file-tree ${className}`.trim()}>
      {loading ? (
        <div class="file-tree__loading">Loading...</div>
      ) : nodes.length === 0 ? (
        <div class="file-tree__empty">{emptyMessage}</div>
      ) : (
        <TreeView<T> {...treeProps} />
      )}
    </div>
  );
}
