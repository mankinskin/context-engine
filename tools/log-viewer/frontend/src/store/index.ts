// Reactive state store using Preact Signals
// Supports per-file state for tabs, code viewer, etc.

import { signal, computed } from '@preact/signals';
import type { LogFile, LogEntry, ViewTab, LogLevel, EventType, LogStats, HypergraphSnapshot, SearchStateEvent, VizPathGraph, PathTransition } from '../types';
import { applyTransition, emptyPathGraph } from '../search-path/reconstruction';
import * as api from '../api';

// Per-file state interface
interface FileState {
    entries: LogEntry[];
    searchQuery: string;
  jqFilter: string;
    levelFilter: LogLevel | '';
    typeFilter: EventType | '';
    selectedEntry: LogEntry | null;
    codeViewerFile: string | null;
    codeViewerContent: string;
    codeViewerLine: number | null;
  activeTab: ViewTab;
  activeSearchStep: number;
  /** Currently selected path_id (null = show flat list) */
  activePathId: string | null;
  /** Step index within the active path group */
  activePathStep: number;
}

// Create default file state
function createFileState(): FileState {
    return {
        entries: [],
        searchQuery: '',
      jqFilter: '',
        levelFilter: '',
        typeFilter: '',
        selectedEntry: null,
        codeViewerFile: null,
        codeViewerContent: '',
        codeViewerLine: null,
      activeTab: 'logs',
      activeSearchStep: -1,
      activePathId: null,
      activePathStep: -1,
    };
}

// Global state
export const logFiles = signal<LogFile[]>([]);
export const currentFile = signal<string | null>(null);
export const isLoading = signal(false);
export const error = signal<string | null>(null);
export const statusMessage = signal('Ready');

// View state (shared across files)
export const activeTab = signal<ViewTab>('logs');
export const showRaw = signal(false);

// Per-file state storage
const fileStates = signal<Map<string, FileState>>(new Map());

// Get or create state for a file
function getFileState(filename: string | null): FileState {
    if (!filename) return createFileState();

    const states = fileStates.value;
    if (!states.has(filename)) {
        const newStates = new Map(states);
        newStates.set(filename, createFileState());
        fileStates.value = newStates;
    }
    return fileStates.value.get(filename)!;
}

// Update state for current file
function updateCurrentFileState(updates: Partial<FileState>) {
    const filename = currentFile.value;
    if (!filename) return;

    const states = new Map(fileStates.value);
    const current = states.get(filename) || createFileState();
    states.set(filename, { ...current, ...updates });
    fileStates.value = states;
}

// Computed: current file's state
const currentFileState = computed(() => getFileState(currentFile.value));

// Computed accessors for current file's state
export const entries = computed(() => currentFileState.value.entries);
export const searchQuery = computed(() => currentFileState.value.searchQuery);
export const jqFilter = computed(() => currentFileState.value.jqFilter);
export const levelFilter = computed(() => currentFileState.value.levelFilter);
export const typeFilter = computed(() => currentFileState.value.typeFilter);
export const selectedEntry = computed(() => currentFileState.value.selectedEntry);
export const codeViewerFile = computed(() => currentFileState.value.codeViewerFile);
export const codeViewerContent = computed(() => currentFileState.value.codeViewerContent);
export const codeViewerLine = computed(() => currentFileState.value.codeViewerLine);

// Computed values
export const filteredEntries = computed(() => {
  let result = entries.value;
    const level = levelFilter.value;
    const type = typeFilter.value;
  
    if (level) {
        result = result.filter(e => e.level.toUpperCase() === level.toUpperCase());
  }
  
    if (type) {
        result = result.filter(e => e.event_type === type);
  }
  
  return result;
});

// Computed: extract hypergraph snapshot from log entries (first graph_snapshot event)
export const hypergraphSnapshot = computed((): HypergraphSnapshot | null => {
  const allEntries = entries.value;
  for (const entry of allEntries) {
    if (entry.message === 'graph_snapshot' && entry.fields?.graph_data) {
      try {
        const data = typeof entry.fields.graph_data === 'string'
          ? JSON.parse(entry.fields.graph_data)
          : entry.fields.graph_data;
        if (data && Array.isArray(data.nodes) && Array.isArray(data.edges)) {
          return data as HypergraphSnapshot;
        }
      } catch {
        // skip invalid JSON
      }
    }
  }
  return null;
});

// ── Graph Operation Visualization ──

// Internal: graph op events paired with their log entry index for
// chronological association with search path events.
interface IndexedGraphOp {
  entryIdx: number;
  event: SearchStateEvent;
}

const _graphOpEventsIndexed = computed((): IndexedGraphOp[] => {
  const allEntries = entries.value;
  const events: IndexedGraphOp[] = [];
  for (let i = 0; i < allEntries.length; i++) {
    const entry = allEntries[i]!;
    // New format: graph_op events
    if (entry.message === 'graph_op' && entry.fields?.graph_op) {
      try {
        const data = typeof entry.fields.graph_op === 'string'
          ? JSON.parse(entry.fields.graph_op)
          : entry.fields.graph_op;
        if (data && typeof data.step === 'number') {
          events.push({ entryIdx: i, event: data as SearchStateEvent });
        }
      } catch {
        // skip invalid JSON
      }
    }
    // Legacy format: search_state events (for old log files)
    else if (entry.message === 'search_state' && entry.fields?.search_state) {
      try {
        const data = typeof entry.fields.search_state === 'string'
          ? JSON.parse(entry.fields.search_state)
          : entry.fields.search_state;
        if (data && typeof data.step === 'number') {
          events.push({ entryIdx: i, event: data as SearchStateEvent });
        }
      } catch {
        // skip invalid JSON
      }
    }
  }
  return events.sort((a, b) => a.event.step - b.event.step);
});

// Computed: extract all graph_op events from log entries (sorted by step)
// Also supports legacy search_state events for backwards compatibility
export const graphOpEvents = computed((): SearchStateEvent[] => {
  return _graphOpEventsIndexed.value.map(e => e.event);
});

// Alias for backwards compatibility
export const searchStates = graphOpEvents;

// Currently active step (per-file, controlled by slider/playback)
export const activeSearchStep = computed(() => currentFileState.value.activeSearchStep);

// The active graph op event
export const activeSearchState = computed((): SearchStateEvent | null => {
  const step = activeSearchStep.value;
  const events = graphOpEvents.value;
  if (step < 0 || step >= events.length) return null;
  return events[step] ?? null;
});

// Action to set the active step (cached per-file)
export function setActiveSearchStep(step: number) {
  updateCurrentFileState({ activeSearchStep: step });
}

// ── Path Group Navigation ──

/** A group of events sharing the same path_id. */
export interface PathGroup {
  pathId: string;
  events: SearchStateEvent[];
  /** Global indices into graphOpEvents for each event in this group */
  globalIndices: number[];
}

// Computed: all distinct path groups, ordered by first appearance
export const pathGroups = computed((): PathGroup[] => {
  const all = graphOpEvents.value;
  const groupMap = new Map<string, { events: SearchStateEvent[]; globalIndices: number[] }>();
  const order: string[] = [];

  for (let i = 0; i < all.length; i++) {
    const ev = all[i]!;
    const pid = ev.path_id;
    let group = groupMap.get(pid);
    if (!group) {
      group = { events: [], globalIndices: [] };
      groupMap.set(pid, group);
      order.push(pid);
    }
    group.events.push(ev);
    group.globalIndices.push(i);
  }

  return order.map(pid => {
    const g = groupMap.get(pid)!;
    return { pathId: pid, events: g.events, globalIndices: g.globalIndices };
  });
});

// Per-file: currently selected path_id
export const activePathId = computed(() => currentFileState.value.activePathId);

// Per-file: step within active path group
export const activePathStep = computed(() => currentFileState.value.activePathStep);

// The active path group (if any)
export const activePathGroup = computed((): PathGroup | null => {
  const pid = activePathId.value;
  if (!pid) return null;
  return pathGroups.value.find(g => g.pathId === pid) ?? null;
});

// The active event from the active path group
export const activePathEvent = computed((): SearchStateEvent | null => {
  const group = activePathGroup.value;
  const step = activePathStep.value;
  if (!group || step < 0 || step >= group.events.length) return null;
  return group.events[step] ?? null;
});

// Actions for path navigation
export function setActivePathId(pathId: string | null) {
  updateCurrentFileState({ activePathId: pathId, activePathStep: pathId ? 0 : -1 });
}

export function setActivePathStep(step: number) {
  updateCurrentFileState({ activePathStep: step });
}

// ── Search Path Visualization (unified in GraphOpEvent) ──

/**
 * Reconstruct VizPathGraph by replaying all path_transition events
 * in the active path group up to and including the activePathStep.
 */
export const activeSearchPath = computed((): VizPathGraph | null => {
  const group = activePathGroup.value;
  const step = activePathStep.value;
  if (!group || step < 0) return null;

  const graph = emptyPathGraph();
  const limit = Math.min(step, group.events.length - 1);
  for (let i = 0; i <= limit; i++) {
    const pt = group.events[i]?.path_transition;
    if (!pt) continue; // Skip informational events (no path change)
    try {
      applyTransition(graph, pt);
    } catch {
    // Skip invalid transitions gracefully
    }
  }
  return graph;
});

export const logStats = computed((): LogStats => {
  const allEntries = entries.value;
  
  const levelCounts = { TRACE: 0, DEBUG: 0, INFO: 0, WARN: 0, ERROR: 0 } as Record<LogLevel, number>;
  const typeCounts = { event: 0, span_enter: 0, span_exit: 0, unknown: 0 } as Record<EventType, number>;
  const spanDurations: Record<string, { count: number; totalDuration: number }> = {};
  
  for (const entry of allEntries) {
    const level = entry.level.toUpperCase() as LogLevel;
    if (level in levelCounts) levelCounts[level]++;
    
    const type = entry.event_type as EventType;
    if (type in typeCounts) typeCounts[type]++;
    
    // Track span durations
    if (entry.event_type === 'span_exit' && entry.span_name) {
      const busyField = entry.fields['busy'];
      if (typeof busyField === 'string') {
        const durationMatch = busyField.match(/([\d.]+)(µs|ms|s)/);
        if (durationMatch) {
          const durationStr = durationMatch[1];
          const unit = durationMatch[2];
          if (!durationStr || !unit) continue;
          
          let duration = parseFloat(durationStr);
          if (unit === 'µs') duration /= 1000000;
          else if (unit === 'ms') duration /= 1000;
          
          if (!spanDurations[entry.span_name]) {
            spanDurations[entry.span_name] = { count: 0, totalDuration: 0 };
          }
          const spanData = spanDurations[entry.span_name];
          if (spanData) {
            spanData.count++;
            spanData.totalDuration += duration;
          }
        }
      }
    }
  }
  
  // Build timeline data
  const timelineMap = new Map<number, number>();
  for (const entry of allEntries) {
    if (entry.timestamp) {
      const ts = parseFloat(entry.timestamp);
      const bucket = Math.floor(ts * 10) / 10; // 100ms buckets
      timelineMap.set(bucket, (timelineMap.get(bucket) || 0) + 1);
    }
  }
  const timelineData = Array.from(timelineMap.entries())
    .map(([timestamp, count]) => ({ timestamp, count }))
    .sort((a, b) => a.timestamp - b.timestamp);
  
  // Top spans by count
  const topSpans = Object.entries(spanDurations)
    .map(([name, data]) => ({
      name,
      count: data.count,
      avgDuration: data.totalDuration / data.count
    }))
    .sort((a, b) => b.count - a.count)
    .slice(0, 10);
  
  return { levelCounts, typeCounts, timelineData, topSpans };
});

// Actions
export async function loadLogFiles() {
  isLoading.value = true;
  error.value = null;
  
  try {
    logFiles.value = await api.fetchLogFiles();
    statusMessage.value = `Found ${logFiles.value.length} log files`;
  } catch (e) {
    error.value = String(e);
    statusMessage.value = 'Error loading files';
  } finally {
    isLoading.value = false;
  }
}

export async function loadLogFile(name: string) {
    // Check if we already have entries for this file
    const existingState = fileStates.value.get(name);
    if (existingState && existingState.entries.length > 0) {
      // Just switch to the file, keep the currently active tab
      currentFile.value = name;
        statusMessage.value = `Loaded ${name} (${existingState.entries.length} entries)`;
        return;
    }

  isLoading.value = true;
  error.value = null;
  statusMessage.value = `Loading ${name}...`;
  
  try {
    const data = await api.fetchLogContent(name);

    // Create state for this file, inheriting the currently active tab
      const states = new Map(fileStates.value);
      states.set(name, {
          ...createFileState(),
          entries: data.entries,
        activeTab: activeTab.value,
      });
      fileStates.value = states;

      currentFile.value = name;
    // Keep the currently active tab type when opening a new file
    statusMessage.value = `Loaded ${name} (${data.entries.length} entries)`;
  } catch (e) {
    error.value = String(e);
    statusMessage.value = 'Error loading file';
  } finally {
    isLoading.value = false;
  }
}

export async function performSearch(query: string) {
  if (!query || !currentFile.value) {
      updateCurrentFileState({ searchQuery: '' });
    return;
  }
  
  isLoading.value = true;
  statusMessage.value = `Searching for "${query}"...`;
  
  try {
    const data = await api.searchLogs(
      currentFile.value,
      query,
      levelFilter.value || undefined
    );
      updateCurrentFileState({
          entries: data.matches,
          searchQuery: query,
      });
    statusMessage.value = `Found ${data.total_matches} matches`;
  } catch (e) {
    error.value = String(e);
    statusMessage.value = `Search error: ${e}`;
  } finally {
    isLoading.value = false;
  }
}

export async function performJqQuery(filter: string) {
  if (!filter || !currentFile.value) {
    updateCurrentFileState({ jqFilter: '' });
    return;
  }

  isLoading.value = true;
  statusMessage.value = `Applying JQ filter...`;

  try {
    const data = await api.queryLogs(
      currentFile.value,
      filter
    );
    updateCurrentFileState({
      entries: data.matches,
      jqFilter: filter,
      searchQuery: '', // Clear text search when using JQ
    });
    statusMessage.value = `JQ filter matched ${data.total_matches} entries`;
  } catch (e) {
    error.value = String(e);
    statusMessage.value = `JQ error: ${e}`;
  } finally {
    isLoading.value = false;
  }
}

export async function openSourceFile(path: string, line?: number) {
  try {
    const data = await api.fetchSourceFile(path);
      updateCurrentFileState({
          codeViewerFile: path,
          codeViewerContent: data.content,
          codeViewerLine: line ?? null,
      });
    activeTab.value = 'code';
    updateCurrentFileState({ activeTab: 'code' });
  } catch (e) {
    error.value = `Failed to load source: ${e}`;
  }
}

export function selectEntry(entry: LogEntry | null) {
    updateCurrentFileState({ selectedEntry: entry });
}

export function setTab(tab: ViewTab) {
  activeTab.value = tab;
  updateCurrentFileState({ activeTab: tab });
}

export function setLevelFilter(level: LogLevel | '') {
    updateCurrentFileState({ levelFilter: level });
}

export function setTypeFilter(type: EventType | '') {
    updateCurrentFileState({ typeFilter: type });
}

export function clearSearch() {
    // Reload original entries for current file
    const filename = currentFile.value;
    if (!filename) return;

    // Force reload
    const states = new Map(fileStates.value);
    states.delete(filename);
    fileStates.value = states;
    loadLogFile(filename);
}
