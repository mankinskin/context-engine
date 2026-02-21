import { useState, useEffect } from 'preact/hooks';
import { docTree, totalDocs, isLoading, selectedFilename, selectDoc, loadCrateModules, openCrateDoc, openCategoryPage, preloadVisibleCrateTrees, expandedNodes, toggleNodeExpanded } from '../store';
import type { TreeNode } from '../types';

export function Sidebar() {
  // Watch for tree changes and preload crate trees that aren't loaded yet
  useEffect(() => {
    const cratesRoot = docTree.value.find(n => n.id === 'crates');
    if (cratesRoot?.children) {
      const unloadedCrates = cratesRoot.children
        .filter(child => child.type === 'crate' && (!child.children || child.children.length === 0))
        .map(child => child.crateName!)
        .filter(Boolean);
      if (unloadedCrates.length > 0) {
        preloadVisibleCrateTrees(unloadedCrates);
      }
    }
  }, [docTree.value]);

  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>Documentation</h2>
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
              <TreeItem key={node.id} node={node} level={0} />
            ))}
          </div>
        )}
      </div>
    </aside>
  );
}

interface TreeItemProps {
  node: TreeNode;
  level: number;
}

function TreeItem({ node, level }: TreeItemProps) {
  const [loading, setLoading] = useState(false);
  
  // Use global expanded state
  const expanded = expandedNodes.value.has(node.id);
  
  const hasChildren = node.children && node.children.length > 0;
  const canExpand = hasChildren || node.type === 'crate'; // Crates can always expand
  const isSelected = (node.type === 'doc' || node.type === 'module' || node.type === 'crate') 
    && selectedFilename.value === node.id;
  
  const handleClick = async () => {
    if (node.type === 'doc') {
      // Open agent doc
      selectDoc(node.id);
    } else if (node.type === 'root') {
      // Open category page for root nodes
      if (node.id === 'agents') {
        openCategoryPage('page:agent-docs');
      } else if (node.id === 'crates') {
        openCategoryPage('page:crate-docs');
      }
      toggleNodeExpanded(node.id);
    } else if (node.type === 'crate') {
      // Load crate modules if not loaded, and open crate doc
      if (!node.children || node.children.length === 0) {
        setLoading(true);
        try {
          await loadCrateModules(node.crateName!);
        } finally {
          setLoading(false);
        }
      }
      toggleNodeExpanded(node.id);
      // Open crate root doc
      openCrateDoc(node.crateName!);
    } else if (node.type === 'module') {
      // Open module doc
      openCrateDoc(node.crateName!, node.modulePath);
      if (hasChildren) {
        toggleNodeExpanded(node.id);
      }
    } else if (canExpand) {
      // Toggle expand for category nodes
      toggleNodeExpanded(node.id);
    }
  };
  
  // Get appropriate icon based on node type
  const getIcon = () => {
    switch (node.type) {
      case 'root':
        return <FolderIcon />;
      case 'category':
        return <FolderIcon />;
      case 'crate':
        return <CrateIcon />;
      case 'module':
        return hasChildren ? <FolderIcon /> : <ModuleIcon />;
      case 'doc':
        return <FileIcon />;
      default:
        return <FileIcon />;
    }
  };
  
  // Get icon class based on node type
  const iconClass = () => {
    switch (node.type) {
      case 'root':
        return 'folder';
      case 'category':
        return 'folder';
      case 'crate':
        return 'crate';
      case 'module':
        return 'module';
      case 'doc':
        return 'file';
      default:
        return 'file';
    }
  };
  
  return (
    <div class="tree-item">
      <div 
        class={`tree-row ${isSelected ? 'selected' : ''}`}
        onClick={handleClick}
      >
        <span class={`tree-toggle ${expanded ? 'expanded' : ''} ${!canExpand ? 'empty' : ''}`}>
          {loading ? <SpinnerIcon /> : <ChevronIcon />}
        </span>
        <span class={`tree-icon ${iconClass()}`}>
          {getIcon()}
        </span>
        <span class="tree-label">{node.label}</span>
        {hasChildren && <span class="tree-badge">{node.children!.length}</span>}
      </div>
      {canExpand && expanded && (
        <div class="tree-children">
          {node.children?.map(child => (
            <TreeItem key={child.id} node={child} level={level + 1} />
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

function SpinnerIcon() {
  return (
    <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" stroke-dasharray="32" stroke-dashoffset="16" />
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

function CrateIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor">
      <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
      <polyline points="3.27 6.96 12 12.01 20.73 6.96" fill="none" stroke="var(--bg-primary)" stroke-width="1.5" />
      <line x1="12" y1="22.08" x2="12" y2="12" stroke="var(--bg-primary)" stroke-width="1.5" />
    </svg>
  );
}

function ModuleIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z" />
      <polyline points="14 2 14 8 20 8" />
      <path d="M8 13h8M8 17h8" />
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
