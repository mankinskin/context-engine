// Reactive state store using Preact Signals
// Supports per-file state for tabs, code viewer, etc.

import { signal, computed } from '@preact/signals';
import type { LogFile, LogEntry, ViewTab, LogLevel, EventType, LogStats } from '../types';
import * as api from '../api';

// Per-file state interface
interface FileState {
    entries: LogEntry[];
    searchQuery: string;
    levelFilter: LogLevel | '';
    typeFilter: EventType | '';
    selectedEntry: LogEntry | null;
    codeViewerFile: string | null;
    codeViewerContent: string;
    codeViewerLine: number | null;
}

// Create default file state
function createFileState(): FileState {
    return {
        entries: [],
        searchQuery: '',
        levelFilter: '',
        typeFilter: '',
        selectedEntry: null,
        codeViewerFile: null,
        codeViewerContent: '',
        codeViewerLine: null,
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
      if (busyField) {
        const durationMatch = busyField.match(/(\d+(?:\.\d+)?)(µs|ms|s)/);
        if (durationMatch) {
          let duration = parseFloat(durationMatch[1]);
          const unit = durationMatch[2];
          if (unit === 'µs') duration /= 1000000;
          else if (unit === 'ms') duration /= 1000;
          
          if (!spanDurations[entry.span_name]) {
            spanDurations[entry.span_name] = { count: 0, totalDuration: 0 };
          }
          spanDurations[entry.span_name].count++;
          spanDurations[entry.span_name].totalDuration += duration;
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
        // Just switch to the file, state is preserved
        currentFile.value = name;
        statusMessage.value = `Loaded ${name} (${existingState.entries.length} entries)`;
        return;
    }

  isLoading.value = true;
  error.value = null;
  statusMessage.value = `Loading ${name}...`;
  
  try {
    const data = await api.fetchLogContent(name);

      // Create state for this file
      const states = new Map(fileStates.value);
      states.set(name, {
          ...createFileState(),
          entries: data.entries,
      });
      fileStates.value = states;

      currentFile.value = name;
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

export async function openSourceFile(path: string, line?: number) {
  try {
    const data = await api.fetchSourceFile(path);
      updateCurrentFileState({
          codeViewerFile: path,
          codeViewerContent: data.content,
          codeViewerLine: line ?? null,
      });
    activeTab.value = 'code';
  } catch (e) {
    error.value = `Failed to load source: ${e}`;
  }
}

export function selectEntry(entry: LogEntry | null) {
    updateCurrentFileState({ selectedEntry: entry });
}

export function setTab(tab: ViewTab) {
  activeTab.value = tab;
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
