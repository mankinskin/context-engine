export interface DocSummary {
  filename: string;
  title: string;
  date: string;
  summary: string;
  tags: string[];
  status: string | null;
}

export interface Category {
  category: string;
  count: number;
  docs: DocSummary[];
}

export interface DocListResponse {
  total: number;
  categories: Category[];
}

export interface DocContent {
  filename: string;
  doc_type: string;
  title: string;
  date: string;
  summary: string;
  tags: string[];
  status: string | null;
  body: string | null;
}

export interface TreeNode {
  id: string;
  label: string;
  type: 'category' | 'doc';
  category?: string;
  children?: TreeNode[];
  data?: DocSummary;
}
