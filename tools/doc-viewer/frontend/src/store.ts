import { signal } from '@preact/signals';
import type { Category, DocContent, TreeNode } from './types';
import { fetchDocs, fetchDoc } from './api';

// State signals
export const categories = signal<Category[]>([]);
export const totalDocs = signal(0);
export const selectedDoc = signal<DocContent | null>(null);
export const selectedFilename = signal<string | null>(null);
export const isLoading = signal(false);
export const error = signal<string | null>(null);

// Build tree structure from categories
export const docTree = signal<TreeNode[]>([]);

function buildTree(cats: Category[]): TreeNode[] {
  return cats.map(cat => ({
    id: cat.category,
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
    const data = await fetchDocs();
    categories.value = data.categories;
    totalDocs.value = data.total;
    docTree.value = buildTree(data.categories);
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load docs';
  } finally {
    isLoading.value = false;
  }
}

export async function selectDoc(filename: string): Promise<void> {
  if (selectedFilename.value === filename) return;
  
  selectedFilename.value = filename;
  isLoading.value = true;
  error.value = null;
  
  try {
    const doc = await fetchDoc(filename);
    selectedDoc.value = doc;
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load document';
    selectedDoc.value = null;
  } finally {
    isLoading.value = false;
  }
}
