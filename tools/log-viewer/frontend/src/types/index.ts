// Types for the Log Viewer application

export interface LogFile {
  name: string;
  size: number;
  modified: string | null;
  has_graph_snapshot: boolean;
}

export interface AssertionDiff {
  title: string;
  left_label: string;
  right_label: string;
  left_value: string;
  right_value: string;
}

export interface LogEntry {
  line_number: number;
  level: LogLevel;
  timestamp: string | null;
  message: string;
  event_type: EventType;
  span_name: string | null;
  depth: number;
  fields: Record<string, unknown>;
  file: string | null;
  source_line: number | null;
  panic_file: string | null;
  panic_line: number | null;
  assertion_diff: AssertionDiff | null;
  backtrace: string | null;
  raw: string;
}

export type LogLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
export type EventType = 'event' | 'span_enter' | 'span_exit' | 'unknown';

export interface LogContentResponse {
  name: string;
  entries: LogEntry[];
  total_lines: number;
}

export interface SearchResponse {
  query: string;
  matches: LogEntry[];
  total_matches: number;
}

export interface JqQueryResponse {
  query: string;
  matches: LogEntry[];
  total_matches: number;
}

export interface SourceFileResponse {
  path: string;
  content: string;
  language: string;
  total_lines: number;
}

export interface SourceSnippet {
  path: string;
  content: string;
  start_line: number;
  end_line: number;
  highlight_line: number;
  language: string;
}

export type ViewTab = 'logs' | 'stats' | 'code' | 'debug' | 'scene3d' | 'hypergraph' | 'settings';

// ── Hypergraph snapshot types (from Rust graph serialization) ──

export interface HypergraphSnapshot {
  nodes: HypergraphNode[];
  edges: HypergraphEdge[];
}

export interface HypergraphNode {
  index: number;
  label: string;
  width: number;
  is_atom: boolean;
}

export interface HypergraphEdge {
  from: number;
  to: number;
  pattern_idx: number;
  sub_index: number;
}

export interface FlowNode {
  id: string;
  entry: LogEntry;
  type: 'event' | 'span';
}

export interface FlowEdge {
  source: string;
  target: string;
}

// ── Graph Operation Visualization Types ──

export type OperationType = 'search' | 'insert' | 'read';

// Transition kinds (tagged union discriminated by 'kind')
export type Transition =
  | { kind: 'start_node'; node: number }
  | { kind: 'visit_parent'; from: number; to: number; entry_pos: number }
  | { kind: 'visit_child'; from: number; to: number; child_index: number }
  | { kind: 'child_match'; node: number; cursor_pos: number }
  | { kind: 'child_mismatch'; node: number; cursor_pos: number; expected: number; actual: number }
  | { kind: 'done'; final_node: number | null; success: boolean }
  | { kind: 'dequeue'; node: number; queue_remaining: number; is_parent: boolean }
  | { kind: 'root_explore'; root: number }
  | { kind: 'match_advance'; root: number; prev_pos: number; new_pos: number }
  | { kind: 'parent_explore'; current_root: number; parent_candidates: number[] }
  | { kind: 'split_start'; node: number; split_position: number }
  | { kind: 'split_complete'; original_node: number; left_fragment: number | null; right_fragment: number | null }
  | { kind: 'join_start'; nodes: number[] }
  | { kind: 'join_step'; left: number; right: number; result: number }
  | { kind: 'join_complete'; result_node: number }
  | { kind: 'create_pattern'; parent: number; pattern_id: number; children: number[] }
  | { kind: 'create_root'; node: number; width: number }
  | { kind: 'update_pattern'; parent: number; pattern_id: number; old_children: number[]; new_children: number[] };

export interface LocationInfo {
  selected_node: number | null;
  root_node: number | null;
  trace_path: number[];
  completed_nodes: number[];
  pending_parents: number[];
  pending_children: number[];
}

export interface QueryInfo {
  query_tokens: number[];
  cursor_position: number;
  query_width: number;
}

export interface GraphOpEvent {
  step: number;
  op_type: OperationType;
  transition: Transition;
  location: LocationInfo;
  query: QueryInfo;
  description: string;
}

// Legacy alias for backwards compatibility
export type SearchStateEvent = GraphOpEvent;

export interface LogStats {
  levelCounts: Record<LogLevel, number>;
  typeCounts: Record<EventType, number>;
  timelineData: { timestamp: number; count: number }[];
  topSpans: { name: string; count: number; avgDuration: number }[];
}
