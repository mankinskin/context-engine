import type { DocListResponse, DocContent } from './types';

const API_BASE = '/api';

export async function fetchDocs(): Promise<DocListResponse> {
  const response = await fetch(`${API_BASE}/docs`);
  if (!response.ok) {
    throw new Error(`Failed to fetch docs: ${response.statusText}`);
  }
  return response.json();
}

export async function fetchDoc(filename: string): Promise<DocContent> {
  const response = await fetch(`${API_BASE}/docs/${encodeURIComponent(filename)}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch doc: ${response.statusText}`);
  }
  return response.json();
}
