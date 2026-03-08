// Types for the Log Viewer application
//
// Generated types (from Rust via ts-rs): import from './generated'
// Regenerate with: cargo test -p context-trace -p log-viewer export_bindings

// ── Re-export generated types ──
export type {
  AssertionDiff,
  EdgeRef,
  GraphOpEvent,
  GraphSnapshot,
  JqQueryResponse,
  LocationInfo,
  LogContentResponse,
  LogEntry,
  LogFileInfo,
  OperationType,
  PathNode,
  PathTransition,
  QueryInfo,
  SearchResponse,
  SnapshotEdge,
  SnapshotNode,
  Transition,
  VizPathGraph,
} from './generated';

// ── Frontend-only types (not generated from Rust) ──

// Alias: LogFileInfo was previously called LogFile in the frontend
export type { LogFileInfo as LogFile } from './generated';

export type LogLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
export type EventType = 'event' | 'span_enter' | 'span_exit' | 'unknown';

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

// Snapshot aliases (match the old naming convention used across the frontend)
export type { GraphSnapshot as HypergraphSnapshot } from './generated';
export type { SnapshotNode as HypergraphNode } from './generated';
export type { SnapshotEdge as HypergraphEdge } from './generated';

export interface FlowNode {
  id: string;
  entry: import('./generated').LogEntry;
  type: 'event' | 'span';
}

export interface FlowEdge {
  source: string;
  target: string;
}

// Legacy alias for backwards compatibility
export type SearchStateEvent = import('./generated').GraphOpEvent;

export interface LogStats {
  levelCounts: Record<LogLevel, number>;
  typeCounts: Record<EventType, number>;
  timelineData: { timestamp: number; count: number }[];
  topSpans: { name: string; count: number; avgDuration: number }[];
}
