// Static API adapter — serves pre-parsed JSON from public/data/ instead of a live backend.
// Used when building for GitHub Pages (VITE_STATIC_MODE=true).

import type { LogFile, LogContentResponse, SearchResponse, JqQueryResponse, SourceFileResponse, SourceSnippet } from '../types';
import type { SessionConfig, SessionConfigUpdate } from './index';

const DATA_BASE = import.meta.env.BASE_URL + 'data';

// ── Session (no-op in static mode) ──

export async function getSessionConfig(): Promise<SessionConfig> {
  return { session_id: 'static', verbose: false, source_request_count: 0 };
}

export async function updateSessionConfig(_update: SessionConfigUpdate): Promise<SessionConfig> {
  return getSessionConfig();
}

// ── Log listing ──

export async function fetchLogFiles(): Promise<LogFile[]> {
  const resp = await fetch(`${DATA_BASE}/manifest.json`);
  if (!resp.ok) throw new Error('Failed to load manifest.json');
  return resp.json();
}

// ── Log content ──

// Cache loaded log content so search/filter can reuse it
const contentCache = new Map<string, LogContentResponse>();

export async function fetchLogContent(name: string): Promise<LogContentResponse> {
  const cached = contentCache.get(name);
  if (cached) return cached;

  const jsonName = name.replace(/\.log$/, '.json');
  const resp = await fetch(`${DATA_BASE}/${encodeURIComponent(jsonName)}`);
  if (!resp.ok) throw new Error(`Failed to load ${jsonName}`);
  const data: LogContentResponse = await resp.json();
  contentCache.set(name, data);
  return data;
}

// ── Search (client-side regex on cached entries) ──

export async function searchLogs(
  name: string,
  query: string,
  level?: string,
  _limit?: number,
): Promise<SearchResponse> {
  const content = await fetchLogContent(name);
  let regex: RegExp;
  try {
    regex = new RegExp(query, 'i');
  } catch {
    throw new Error(`Invalid regex: ${query}`);
  }

  let matches = content.entries.filter(e => {
    if (level && e.level.toUpperCase() !== level.toUpperCase()) return false;
    // Search message, raw, and field values
    if (regex.test(e.message)) return true;
    if (regex.test(e.raw)) return true;
    for (const v of Object.values(e.fields)) {
      if (regex.test(typeof v === 'string' ? v : JSON.stringify(v))) return true;
    }
    return false;
  });

  return { query, matches, total_matches: matches.length };
}

// ── JQ query (not supported in static mode) ──

export async function queryLogs(
  _name: string,
  _jqFilter: string,
  _limit?: number,
): Promise<JqQueryResponse> {
  throw new Error('JQ queries are not available in static demo mode');
}

// ── Source files (not available in static mode) ──

export async function fetchSourceFile(_path: string): Promise<SourceFileResponse> {
  throw new Error('Source file viewing is not available in static demo mode');
}

export async function fetchSourceSnippet(
  _path: string,
  _line: number,
  _context?: number,
): Promise<SourceSnippet> {
  throw new Error('Source snippets are not available in static demo mode');
}
