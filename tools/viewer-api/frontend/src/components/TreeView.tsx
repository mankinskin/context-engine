import { JSX } from 'preact';
import { useState, useCallback } from 'preact/hooks';

export interface TreeNode {
  id: string;
  label: string;
  icon?: 'folder' | 'file' | 'doc';
  children?: TreeNode[];
  data?: unknown;
}

export interface TreeViewProps {
  nodes: TreeNode[];
  selectedId?: string;
  onSelect?: (node: TreeNode) => void;
  defaultExpanded?: string[];
}

export function TreeView({ nodes, selectedId, onSelect, defaultExpanded = [] }: TreeViewProps): JSX.Element {
  const [expanded, setExpanded] = useState<Set<string>>(new Set(defaultExpanded));

  const toggleExpanded = useCallback((id: string) => {
    setExpanded(prev => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  return (
    <div class="tree-view">
      {nodes.map(node => (
        <TreeItem
          key={node.id}
          node={node}
          selectedId={selectedId}
          expanded={expanded}
          onToggle={toggleExpanded}
          onSelect={onSelect}
          depth={0}
        />
      ))}
    </div>
  );
}

interface TreeItemProps {
  node: TreeNode;
  selectedId?: string;
  expanded: Set<string>;
  onToggle: (id: string) => void;
  onSelect?: (node: TreeNode) => void;
  depth: number;
}

function TreeItem({ node, selectedId, expanded, onToggle, onSelect, depth }: TreeItemProps): JSX.Element {
  const hasChildren = node.children && node.children.length > 0;
  const isExpanded = expanded.has(node.id);
  const isSelected = node.id === selectedId;

  const handleClick = () => {
    if (hasChildren) {
      onToggle(node.id);
    }
    onSelect?.(node);
  };

  const handleToggle = (e: Event) => {
    e.stopPropagation();
    onToggle(node.id);
  };

  return (
    <div class="tree-item" style={{ paddingLeft: `${depth * 8}px` }}>
      <div 
        class={`tree-item-row ${isSelected ? 'selected' : ''}`}
        onClick={handleClick}
      >
        <span 
          class={`tree-toggle ${isExpanded ? 'expanded' : ''} ${!hasChildren ? 'empty' : ''}`}
          onClick={hasChildren ? handleToggle : undefined}
        >
          <ChevronIcon />
        </span>
        <span class={`tree-icon ${node.icon || (hasChildren ? 'folder' : 'file')}`}>
          {node.icon === 'doc' ? <DocIcon /> : (hasChildren ? <FolderIcon /> : <FileIcon />)}
        </span>
        <span class="tree-label">{node.label}</span>
        {hasChildren && <span class="tree-badge">{node.children!.length}</span>}
      </div>
      {hasChildren && isExpanded && (
        <div class="tree-children">
          {node.children!.map(child => (
            <TreeItem
              key={child.id}
              node={child}
              selectedId={selectedId}
              expanded={expanded}
              onToggle={onToggle}
              onSelect={onSelect}
              depth={depth + 1}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function ChevronIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <polyline points="9 18 15 12 9 6" />
    </svg>
  );
}

function FolderIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor">
      <path d="M10 4H4a2 2 0 00-2 2v12a2 2 0 002 2h16a2 2 0 002-2V8a2 2 0 00-2-2h-8l-2-2z" />
    </svg>
  );
}

function FileIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z" />
      <polyline points="14 2 14 8 20 8" />
    </svg>
  );
}

function DocIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z" />
      <polyline points="14 2 14 8 20 8" />
      <line x1="16" y1="13" x2="8" y2="13" />
      <line x1="16" y1="17" x2="8" y2="17" />
      <line x1="10" y1="9" x2="8" y2="9" />
    </svg>
  );
}
