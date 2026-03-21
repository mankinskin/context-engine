// API types matching the ticket serve REST responses.

export interface WorkspacesResponse {
  request_id: string;
  workspaces: WorkspaceInfo[];
}

export interface WorkspaceInfo {
  name: string;
}

export interface TicketsResponse {
  request_id: string;
  workspace: string;
  items: TicketSummary[];
  next_cursor: string | null;
}

export interface TicketSummary {
  id: string;
  type: string;
  title: string | null;
  state: string | null;
  updated_at: string;
  fields: Record<string, unknown>;
}

export interface TicketDetailResponse {
  request_id: string;
  workspace: string;
  ticket: TicketDetail;
}

export interface TicketDetail {
  id: string;
  created_at: string;
  fields: Record<string, unknown>;
}

export interface TicketDescriptionResponse {
  request_id: string;
  workspace: string;
  id: string;
  description: string | null;
}

export interface EdgeRecord {
  from: string;
  to: string;
  kind: string;
}

export interface EdgesResponse {
  request_id: string;
  workspace: string;
  edges: EdgeRecord[];
}

// ── Tab state ─────────────────────────────────────────────────────────────────

export type TabId = 'description' | 'fields';

export interface OpenTicket {
  id: string;
  title: string | null;
}
