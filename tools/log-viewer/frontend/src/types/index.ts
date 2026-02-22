// Types for the Log Viewer application

export interface LogFile {
  name: string;
  size: number;
  modified: string | null;
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

export type ViewTab = 'logs' | 'flow' | 'stats' | 'code' | 'debug';

export interface FlowNode {
  id: string;
  entry: LogEntry;
  type: 'event' | 'span';
}

export interface FlowEdge {
  source: string;
  target: string;
}

export interface LogStats {
  levelCounts: Record<LogLevel, number>;
  typeCounts: Record<EventType, number>;
  timelineData: { timestamp: number; count: number }[];
  topSpans: { name: string; count: number; avgDuration: number }[];
}
