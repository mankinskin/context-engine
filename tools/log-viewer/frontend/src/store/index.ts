// Reactive state store using Preact Signals

import { signal, computed } from '@preact/signals';
import type { LogFile, LogEntry, ViewTab, LogLevel, EventType, LogStats } from '../types';
import * as api from '../api';

// Core state
export const logFiles = signal<LogFile[]>([]);
export const currentFile = signal<string | null>(null);
export const entries = signal<LogEntry[]>([]);
export const isLoading = signal(false);
export const error = signal<string | null>(null);
export const statusMessage = signal('Ready');

// View state
export const activeTab = signal<ViewTab>('logs');
export const searchQuery = signal('');
export const levelFilter = signal<LogLevel | ''>('');
export const typeFilter = signal<EventType | ''>('');
export const showRaw = signal(false);
export const selectedEntry = signal<LogEntry | null>(null);

// Code viewer state
export const codeViewerFile = signal<string | null>(null);
export const codeViewerContent = signal<string>('');
export const codeViewerLine = signal<number | null>(null);

// Computed values
export const filteredEntries = computed(() => {
  let result = entries.value;
  
  if (levelFilter.value) {
    result = result.filter(e => e.level.toUpperCase() === levelFilter.value.toUpperCase());
  }
  
  if (typeFilter.value) {
    result = result.filter(e => e.event_type === typeFilter.value);
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
  isLoading.value = true;
  error.value = null;
  statusMessage.value = `Loading ${name}...`;
  
  try {
    const data = await api.fetchLogContent(name);
    currentFile.value = name;
    entries.value = data.entries;
    searchQuery.value = '';
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
    searchQuery.value = '';
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
    entries.value = data.matches;
    searchQuery.value = query;
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
    codeViewerFile.value = path;
    codeViewerContent.value = data.content;
    codeViewerLine.value = line ?? null;
    activeTab.value = 'code';
  } catch (e) {
    error.value = `Failed to load source: ${e}`;
  }
}

export function selectEntry(entry: LogEntry | null) {
  selectedEntry.value = entry;
}

export function setTab(tab: ViewTab) {
  activeTab.value = tab;
}
