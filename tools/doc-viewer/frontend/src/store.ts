import { signal, computed } from '@preact/signals';
import type { Category, TreeNode, OpenTab, DocContent } from './types';
import { fetchDocs, fetchDoc, fetchCrates, browseCrate, fetchCrateDoc, type ModuleNode, type CrateDocResponse } from './api';

// State signals
export const categories = signal<Category[]>([]);
export const totalDocs = signal(0);
export const isLoading = signal(false);
export const error = signal<string | null>(null);

// Tab state
export const openTabs = signal<OpenTab[]>([]);
export const activeTabId = signal<string | null>(null);

// Computed: currently active document
export const activeDoc = computed(() => {
  const tab = openTabs.value.find(t => t.filename === activeTabId.value);
  return tab?.doc ?? null;
});

// Computed: is the active tab loading?
export const isActiveTabLoading = computed(() => {
  const tab = openTabs.value.find(t => t.filename === activeTabId.value);
  return tab?.isLoading ?? false;
});

// Build tree structure from categories
export const docTree = signal<TreeNode[]>([]);

function buildAgentDocsTree(cats: Category[]): TreeNode {
  return {
    id: 'agents',
    label: 'Agent Docs',
    type: 'root',
    children: cats.map(cat => ({
      id: `agent:${cat.category}`,
      label: formatCategoryName(cat.category),
      type: 'category' as const,
      category: cat.category,
      children: cat.docs.map(doc => ({
        id: doc.filename,
        label: doc.title || doc.filename,
        type: 'doc' as const,
        category: cat.category,
        data: doc,
      })),
    })),
  };
}

function buildModuleTree(modules: ModuleNode[], crateName: string): TreeNode[] {
  return modules.map(mod => ({
    id: `crate:${crateName}:${mod.path}`,
    label: mod.name,
    type: 'module' as const,
    crateName,
    modulePath: mod.path,
    hasReadme: mod.has_readme,
    children: mod.children.length > 0
      ? buildModuleTree(mod.children, crateName)
      : undefined,
  }));
}

function formatCategoryName(name: string): string {
  // Convert kebab-case to Title Case
  return name
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

// Actions
export async function loadDocs(): Promise<void> {
  isLoading.value = true;
  error.value = null;
  
  try {
    // Load both agent docs and crates in parallel
    const [docsData, cratesData] = await Promise.all([
      fetchDocs(),
      fetchCrates(),
    ]);

    categories.value = docsData.categories;
    totalDocs.value = docsData.total;

    // Build unified tree with roots for Agent Docs and Crates
    const tree: TreeNode[] = [];

    // Add Agent Docs root
    if (docsData.categories.length > 0) {
      tree.push(buildAgentDocsTree(docsData.categories));
    }

    // Add Crates root with lazy-loaded children
    if (cratesData.crates.length > 0) {
      tree.push({
        id: 'crates',
        label: 'Crate Docs',
        type: 'root',
        children: cratesData.crates.map(crate => ({
          id: `crate:${crate.name}`,
          label: crate.name,
          type: 'crate' as const,
          crateName: crate.name,
          hasReadme: crate.has_readme,
          // Children will be loaded when expanded
          children: [],
        })),
      });
    }

    docTree.value = tree;
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load docs';
  } finally {
    isLoading.value = false;
  }
}

// Load crate modules when crate is expanded
export async function loadCrateModules(crateName: string): Promise<void> {
  try {
    const tree = await browseCrate(crateName);

    // Update the crate node with loaded module tree
    docTree.value = docTree.value.map(root => {
      if (root.id !== 'crates') return root;
      return {
        ...root,
        children: root.children?.map(node => {
          if (node.crateName !== crateName) return node;
          return {
            ...node,
            children: buildModuleTree(tree.children, crateName),
          };
        }),
      };
    });
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load crate modules';
  }
}

// Convert crate doc response to DocContent format for tab display
function crateDocToContent(doc: CrateDocResponse): DocContent {
  const title = doc.module_path
    ? `${doc.crate_name}::${doc.module_path.replace(/\//g, '::')}`
    : doc.crate_name;

  // Combine index.yaml and readme into body
  let body = '## index.yaml\n\n```yaml\n' + doc.index_yaml + '\n```\n';
  if (doc.readme) {
    body += '\n## README\n\n' + doc.readme;
  }

  return {
    filename: `crate:${doc.crate_name}${doc.module_path ? ':' + doc.module_path : ''}`,
    doc_type: 'crate',
    title,
    date: '',
    summary: '',
    tags: [],
    status: null,
    body,
  };
}

export async function openDoc(filename: string, title: string): Promise<void> {
  // Check if already open
  const existingIndex = openTabs.value.findIndex(t => t.filename === filename);
  if (existingIndex >= 0) {
    // Just switch to this tab
    activeTabId.value = filename;
    return;
  }

  // Add new tab with loading state
  const newTab: OpenTab = {
    filename,
    title,
    doc: null,
    isLoading: true,
  };

  openTabs.value = [...openTabs.value, newTab];
  activeTabId.value = filename;
  
  try {
    const doc = await fetchDoc(filename);
    // Update the tab with loaded content
    openTabs.value = openTabs.value.map(t =>
      t.filename === filename
        ? { ...t, doc, isLoading: false }
        : t
    );
  } catch (e) {
    // Update tab to show error state
    openTabs.value = openTabs.value.map(t =>
      t.filename === filename
        ? { ...t, isLoading: false }
        : t
    );
    error.value = e instanceof Error ? e.message : 'Failed to load document';
  }
}

// Open crate or module documentation
export async function openCrateDoc(crateName: string, modulePath?: string): Promise<void> {
  const filename = `crate:${crateName}${modulePath ? ':' + modulePath : ''}`;
  const title = modulePath
    ? `${crateName}::${modulePath.replace(/\//g, '::')}`
    : crateName;

  // Check if already open
  const existingIndex = openTabs.value.findIndex(t => t.filename === filename);
  if (existingIndex >= 0) {
    activeTabId.value = filename;
    return;
  }

  // Add new tab with loading state
  const newTab: OpenTab = {
    filename,
    title,
    doc: null,
    isLoading: true,
  };

  openTabs.value = [...openTabs.value, newTab];
  activeTabId.value = filename;

  try {
    const doc = await fetchCrateDoc(crateName, modulePath);
    const content = crateDocToContent(doc);
    openTabs.value = openTabs.value.map(t =>
      t.filename === filename
        ? { ...t, doc: content, isLoading: false }
        : t
    );
  } catch (e) {
    openTabs.value = openTabs.value.map(t =>
      t.filename === filename
        ? { ...t, isLoading: false }
        : t
    );
    error.value = e instanceof Error ? e.message : 'Failed to load crate documentation';
  }
}

export function closeTab(filename: string): void {
  const tabs = openTabs.value;
  const index = tabs.findIndex(t => t.filename === filename);
  if (index < 0) return;

  // Remove the tab
  const newTabs = tabs.filter(t => t.filename !== filename);
  openTabs.value = newTabs;

  // If this was the active tab, switch to another
  if (activeTabId.value === filename) {
    if (newTabs.length === 0) {
      activeTabId.value = null;
    } else if (index >= newTabs.length) {
      // Closed last tab, select previous
      activeTabId.value = newTabs[newTabs.length - 1].filename;
    } else {
      // Select the tab at same position
      activeTabId.value = newTabs[index].filename;
    }
  }
}

export function setActiveTab(filename: string): void {
  activeTabId.value = filename;
}

// Legacy: for backwards compatibility with Sidebar
export const selectedFilename = computed(() => activeTabId.value);

export function selectDoc(filename: string): void {
  // Find the doc info from tree - need to search recursively now
  function findInTree(nodes: TreeNode[]): TreeNode | undefined {
    for (const node of nodes) {
      if (node.id === filename) return node;
      if (node.children) {
        const found = findInTree(node.children);
        if (found) return found;
      }
    }
    return undefined;
  }

  const found = findInTree(docTree.value);
  if (found) {
    openDoc(filename, found.label);
  } else {
    // Fallback: use filename as title
    openDoc(filename, filename);
  }
}
