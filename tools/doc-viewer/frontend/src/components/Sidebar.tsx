import { useState } from 'preact/hooks';
import { docTree, totalDocs, isLoading, selectedFilename, selectDoc } from '../store';
import type { TreeNode } from '../types';

export function Sidebar() {
  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>Documents</h2>
        <span class="doc-count">{totalDocs.value}</span>
      </div>
      
      <div class="sidebar-content">
        {isLoading.value && docTree.value.length === 0 ? (
          <div class="loading">Loading...</div>
        ) : docTree.value.length === 0 ? (
          <div class="empty-state">No documents found</div>
        ) : (
          <div class="tree-view">
            {docTree.value.map(node => (
              <TreeItem key={node.id} node={node} />
            ))}
          </div>
        )}
      </div>
    </aside>
  );
}

interface TreeItemProps {
  node: TreeNode;
}

function TreeItem({ node }: TreeItemProps) {
  const [expanded, setExpanded] = useState(false);
  const hasChildren = node.children && node.children.length > 0;
  const isSelected = node.type === 'doc' && selectedFilename.value === node.id;
  
  const handleClick = () => {
    if (hasChildren) {
      setExpanded(!expanded);
    } else if (node.type === 'doc') {
      selectDoc(node.id);
    }
  };
  
  return (
    <div class="tree-item">
      <div 
        class={`tree-row ${isSelected ? 'selected' : ''}`}
        onClick={handleClick}
      >
        <span class={`tree-toggle ${expanded ? 'expanded' : ''} ${!hasChildren ? 'empty' : ''}`}>
          <ChevronIcon />
        </span>
        <span class={`tree-icon ${hasChildren ? 'folder' : 'file'}`}>
          {hasChildren ? <FolderIcon /> : <FileIcon />}
        </span>
        <span class="tree-label">{node.label}</span>
        {hasChildren && <span class="tree-badge">{node.children!.length}</span>}
      </div>
      {hasChildren && expanded && (
        <div class="tree-children">
          {node.children!.map(child => (
            <TreeItem key={child.id} node={child} />
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
      <line x1="16" y1="13" x2="8" y2="13" />
      <line x1="16" y1="17" x2="8" y2="17" />
    </svg>
  );
}
