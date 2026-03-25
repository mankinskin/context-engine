import type { ComponentChildren } from 'preact';
import { h } from 'preact';
import type { TreeNode } from '@context-engine/viewer-api-frontend';
import type { LogFile } from '../types';

/** Category definition for virtual top-level folders */
interface Category {
  id: string;
  label: string;
  filter: (f: LogFile) => boolean;
  icon: ComponentChildren;
  color: string;
}

const CATEGORIES: Category[] = [
  {
    id: 'cat-search',
    label: 'Search',
    filter: (f) => f.has_search_ops,
    icon: h('svg', { width: 14, height: 14, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' },
      h('circle', { cx: 11, cy: 11, r: 8 }), h('path', { d: 'm21 21-4.35-4.35' }),
    ),
    color: 'var(--accent-blue)',
  },
  {
    id: 'cat-insert',
    label: 'Insert',
    filter: (f) => f.has_insert_ops,
    icon: h('svg', { width: 14, height: 14, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' },
      h('path', { d: 'M12 5v14M5 12h14' }),
    ),
    color: 'var(--accent-green)',
  },
  {
    id: 'cat-graph',
    label: 'Graph',
    filter: (f) => f.has_graph_snapshot,
    icon: h('svg', { width: 14, height: 14, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' },
      h('circle', { cx: 6, cy: 6, r: 3 }), h('circle', { cx: 18, cy: 6, r: 3 }),
      h('circle', { cx: 6, cy: 18, r: 3 }), h('circle', { cx: 18, cy: 18, r: 3 }),
      h('line', { x1: 9, y1: 6, x2: 15, y2: 6 }), h('line', { x1: 6, y1: 9, x2: 6, y2: 15 }),
      h('line', { x1: 18, y1: 9, x2: 18, y2: 15 }), h('line', { x1: 9, y1: 18, x2: 15, y2: 18 }),
    ),
    color: 'var(--accent-purple)',
  },
  {
    id: 'cat-paths',
    label: 'Paths',
    filter: (f) => f.has_search_paths,
    icon: h('svg', { width: 14, height: 14, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '2' },
      h('polyline', { points: '4 7 4 4 20 4 20 7' }),
      h('line', { x1: 12, y1: 21, x2: 12, y2: 8 }),
      h('polyline', { points: '8 12 12 8 16 12' }),
    ),
    color: 'var(--accent-cyan, #22d3ee)',
  },
];

export { CATEGORIES };

/** Format byte size to human-readable string */
export function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

/** Build a tooltip element for a log file */
function buildTooltip(file: LogFile): ComponentChildren {
  const badges: string[] = [];
  if (file.has_graph_snapshot) badges.push('Graph');
  if (file.has_search_ops) badges.push('Search');
  if (file.has_insert_ops) badges.push('Insert');
  if (file.has_search_paths) badges.push('Paths');

  return h('div', { class: 'file-tooltip' },
    h('div', { class: 'file-tooltip-name' }, file.name),
    h('div', { class: 'file-tooltip-meta' },
      `Size: ${formatSize(Number(file.size))}`,
      file.modified ? ` · ${file.modified}` : '',
    ),
    badges.length > 0 && h('div', { class: 'file-tooltip-badges' }, badges.join(', ')),
  );
}

/**
 * Build TreeNode array from flat log file list.
 *
 * Structure:
 *   - Virtual category folders (Graph, Search, Insert, Paths) at top — only if files match
 *   - Directory tree of all files below
 */
export function buildFileTree(files: LogFile[]): TreeNode<LogFile>[] {
  const nodes: TreeNode<LogFile>[] = [];

  // 1. Virtual category folders
  for (const cat of CATEGORIES) {
    const matching = files.filter(cat.filter);
    if (matching.length === 0) continue;

    const children: TreeNode<LogFile>[] = matching.map((f) => ({
      id: `${cat.id}/${f.name}`,
      label: f.name,
      icon: 'file' as const,
      data: f,
      tooltip: buildTooltip(f),
    }));

    nodes.push({
      id: cat.id,
      label: cat.label,
      icon: cat.icon,
      badge: matching.length,
      children,
    });
  }

  // 2. Directory tree of all files
  const dirTree = buildDirectoryTree(files);
  nodes.push(...dirTree);

  return nodes;
}

/** Build directory-grouped tree from flat file paths */
function buildDirectoryTree(files: LogFile[]): TreeNode<LogFile>[] {
  // Group files by their directory prefix
  const dirMap = new Map<string, LogFile[]>();
  const rootFiles: LogFile[] = [];

  for (const file of files) {
    const lastSlash = file.name.lastIndexOf('/');
    if (lastSlash === -1) {
      rootFiles.push(file);
    } else {
      const dir = file.name.slice(0, lastSlash);
      const existing = dirMap.get(dir);
      if (existing) {
        existing.push(file);
      } else {
        dirMap.set(dir, [file]);
      }
    }
  }

  const result: TreeNode<LogFile>[] = [];

  // If everything is flat (no directories), just return file nodes directly
  if (dirMap.size === 0) {
    return rootFiles.map(fileToNode);
  }

  // Build nested directory structure
  const sortedDirs = [...dirMap.keys()].sort();
  for (const dir of sortedDirs) {
    const dirFiles = dirMap.get(dir)!;
    result.push({
      id: `dir-${dir}`,
      label: dir,
      icon: 'folder' as const,
      badge: dirFiles.length,
      children: dirFiles.map(fileToNode),
    });
  }

  // Root-level files
  for (const file of rootFiles) {
    result.push(fileToNode(file));
  }

  return result;
}

function fileToNode(file: LogFile): TreeNode<LogFile> {
  const basename = file.name.includes('/') ? file.name.slice(file.name.lastIndexOf('/') + 1) : file.name;
  return {
    id: `file-${file.name}`,
    label: basename,
    icon: 'file' as const,
    data: file,
    tooltip: buildTooltip(file),
  };
}

/**
 * Get category filter IDs relevant to a filter mode.
 * Returns category IDs that should be expanded when a filter is active.
 */
export function getCategoryIdForFilter(filter: string): string | null {
  switch (filter) {
    case 'graph': return 'cat-graph';
    case 'search': return 'cat-search';
    case 'insert': return 'cat-insert';
    case 'paths': return 'cat-paths';
    default: return null;
  }
}
