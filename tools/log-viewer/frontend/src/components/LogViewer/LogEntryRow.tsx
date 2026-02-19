import { useState, useEffect } from 'preact/hooks';
import type { LogEntry, SourceSnippet } from '../../types';
import { openSourceFile } from '../../store';
import * as api from '../../api';
import Prism from 'prismjs';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-json';
import { FieldsRenderer } from './RustValueRenderer';

interface Props {
  entry: LogEntry;
  showRaw: boolean;
  searchQuery: string;
  isSelected: boolean;
  onSelect: () => void;
  expandAll: boolean | null; // null = default behavior, true = expand all, false = collapse all
}

interface BacktraceFrame {
  index: number;
  function: string;
  location?: string;
}

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

/**
 * Parse backtrace to extract relevant frames (skip runtime/std frames)
 */
function parseBacktrace(backtrace: string): BacktraceFrame[] {
  const frames: BacktraceFrame[] = [];
  const lines = backtrace.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    // Match frame lines like "   0: rust_begin_unwind" or "   1: std::panicking::..."
    const frameMatch = line.match(/^(\d+):\s+(.+)$/);
    if (frameMatch) {
      const index = parseInt(frameMatch[1], 10);
      const func = frameMatch[2];
      
      // Check next line for location
      let location: string | undefined;
      if (i + 1 < lines.length) {
        const nextLine = lines[i + 1].trim();
        if (nextLine.startsWith('at ')) {
          location = nextLine.slice(3);
          i++; // Skip location line
        }
      }
      
      frames.push({ index, function: func, location });
    }
  }
  
  return frames;
}

/**
 * Filter backtrace frames to show only relevant ones (user code, not std/runtime)
 */
function getRelevantFrames(frames: BacktraceFrame[]): BacktraceFrame[] {
  // Skip frames from std, core, rust runtime, test harness
  const skipPatterns = [
    /^rust_begin_unwind/,
    /^core::/,
    /^std::/,
    /^<.*as core::/,
    /^test::/,
    /^__rust_/,
    /^<alloc::/,
    /^alloc::/,
  ];
  
  return frames.filter(f => {
    // Skip runtime frames
    if (skipPatterns.some(p => p.test(f.function))) {
      return false;
    }
    // Keep frames with user code locations (not in rustc or stdlib)
    if (f.location && !f.location.includes('/rustc/') && !f.location.includes('\\rustc\\')) {
      return true;
    }
    // Keep frames that look like user code
    return !f.function.includes('::{{closure}}') || f.location;
  }).slice(0, 5); // Limit to first 5 relevant frames
}

import type { AssertionDiff } from '../../types';

/**
 * Render a panic message with assertion diff from backend
 */
function PanicMessageRenderer({ 
  message, 
  assertionDiff,
  searchQuery 
}: { 
  message: string; 
  assertionDiff: AssertionDiff | null;
  searchQuery: string;
}) {
  if (!assertionDiff) {
    // Fallback to simple display for non-assertion panics
    return (
      <div class="panic-message" dangerouslySetInnerHTML={{ __html: highlightMatch(message, searchQuery) }} />
    );
  }
  
  return (
    <div class="panic-message-formatted">
      <div class="panic-title">{assertionDiff.title}</div>
      <div class="panic-diff">
        <div class="diff-header">
          <span class="diff-label diff-left">← {assertionDiff.left_label}</span>
          <span class="diff-vs">vs</span>
          <span class="diff-label diff-right">{assertionDiff.right_label} →</span>
        </div>
        <div class="diff-content">
          <div class="diff-column diff-left-column">
            {assertionDiff.left_value ? (
              <pre class="diff-value">{assertionDiff.left_value}</pre>
            ) : (
              <div class="diff-empty">(empty)</div>
            )}
          </div>
          <div class="diff-column diff-right-column">
            {assertionDiff.right_value ? (
              <pre class="diff-value">{assertionDiff.right_value}</pre>
            ) : (
              <div class="diff-empty">(empty)</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

function highlightMatch(text: string, query: string): string {
  if (!query) return escapeHtml(text);
  try {
    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    return escapeHtml(text).replace(regex, '<mark class="highlight">$1</mark>');
  } catch {
    return escapeHtml(text);
  }
}

function formatTimestamp(ts: string | null): string {
  if (!ts) return '';
  const num = parseFloat(ts);
  if (num < 1) return `${(num * 1000).toFixed(0)}ms`;
  return `${num.toFixed(2)}s`;
}

/**
 * Highlight code with Prism, returns HTML string
 */
function highlightCode(code: string, language: string): string {
  try {
    const grammar = Prism.languages[language];
    if (grammar) {
      return Prism.highlight(code, grammar, language);
    }
  } catch {
    // Fall back to plain text
  }
  return escapeHtml(code);
}

export function LogEntryRow({ entry, showRaw, searchQuery, isSelected, onSelect, expandAll }: Props) {
  const [snippet, setSnippet] = useState<SourceSnippet | null>(null);
  const [snippetError, setSnippetError] = useState<string | null>(null);
  const [panicSnippet, setPanicSnippet] = useState<SourceSnippet | null>(null);
  const [panicSnippetError, setPanicSnippetError] = useState<string | null>(null);
  const [showSnippet, setShowSnippet] = useState(true);
  // Track if user has manually toggled snippet after expandAll changed
  const [snippetOverride, setSnippetOverride] = useState<boolean | null>(null);
  
  // Reset override when expandAll changes
  useEffect(() => {
    setSnippetOverride(null);
  }, [expandAll]);
  
  // Determine if details should be open based on expandAll override or default
  const detailsOpen = expandAll !== null ? expandAll : true;
  // For snippet: use override if set, otherwise respect expandAll, otherwise use local state
  const snippetVisible = snippetOverride !== null ? snippetOverride : (expandAll !== null ? expandAll : showSnippet);

  const hasLocation = entry.file && entry.source_line;
  const hasPanicLocation = entry.panic_file && entry.panic_line;
  const levelClass = entry.level.toLowerCase();
  const typeClass = entry.event_type.replace('_', '-');
  
  // Calculate indentation for spans
  const indentLevel = Math.min(entry.depth, 10);
  const indentPx = indentLevel * 20;

  // Auto-load snippet on mount if location is available
  useEffect(() => {
    if (hasLocation && !snippet && !snippetError) {
      api.fetchSourceSnippet(entry.file!, entry.source_line!, 3)
        .then(setSnippet)
        .catch(e => setSnippetError(String(e)));
    }
  }, [entry.file, entry.source_line]);
  
  // Auto-load panic snippet if panic location is available
  useEffect(() => {
    if (hasPanicLocation && !panicSnippet && !panicSnippetError) {
      api.fetchSourceSnippet(entry.panic_file!, entry.panic_line!, 3)
        .then(setPanicSnippet)
        .catch(e => setPanicSnippetError(String(e)));
    }
  }, [entry.panic_file, entry.panic_line]);

  const toggleSnippet = () => {
    // When user manually toggles, set override to opposite of current visible state
    setSnippetOverride(!snippetVisible);
    setShowSnippet(!snippetVisible);
  };

  const handleLocationClick = (e: MouseEvent) => {
    e.stopPropagation();
    if (entry.file) {
      openSourceFile(entry.file, entry.source_line ?? undefined);
    }
  };

  // Check if we have fields to display (exclude 'message' from count)
  const fieldEntries = Object.entries(entry.fields).filter(([k]) => k !== 'message');
  const hasFields = fieldEntries.length > 0;
  
  // Check if this is a panic entry
  const isPanic = entry.message.startsWith('PANIC:');
  // Parse backtrace for relevant frames
  const backtraceFrames = entry.backtrace ? getRelevantFrames(parseBacktrace(entry.backtrace)) : [];

  return (
    <div 
      class={`log-entry ${isSelected ? 'selected' : ''} level-${levelClass} type-${typeClass} ${isPanic ? 'panic-entry' : ''}`}
      onClick={onSelect}
      style={{ paddingLeft: `${indentPx + 12}px` }}
    >
      {/* Span depth indicator */}
      {indentLevel > 0 && (
        <div class="depth-indicator" style={{ left: '0', width: `${indentPx}px` }}>
          {Array.from({ length: indentLevel }).map((_, i) => (
            <span key={i} class="depth-line"></span>
          ))}
        </div>
      )}
      
      {/* Main content area */}
      <div class="entry-content">
        {/* Header row with type badge and span name */}
        <div class="entry-header">
          <span class={`level-badge ${levelClass}`}>{entry.level}</span>
          <span class={`type-badge ${typeClass}`}>{entry.event_type.replace('_', ' ')}</span>
          {entry.span_name && (
            <span class="span-name">{entry.span_name}</span>
          )}
          {isPanic && <span class="panic-badge">🔥 PANIC</span>}
        </div>
        
        {/* Main message - prominently displayed */}
        {isPanic ? (
          <PanicMessageRenderer 
            message={entry.message} 
            assertionDiff={entry.assertion_diff}
            searchQuery={searchQuery} 
          />
        ) : (
          <div 
            class="entry-message"
            dangerouslySetInnerHTML={{ __html: highlightMatch(entry.message, searchQuery) }}
          />
        )}
        
        {/* For panics, show source snippet prominently right after the message */}
        {isPanic && hasPanicLocation && panicSnippet && (
          <div class="panic-source">
            <div class="panic-location">
              📍 {entry.panic_file}:{entry.panic_line}
            </div>
            <div class="code-snippet panic-snippet">
              <pre class="snippet-code">
                {(() => {
                  const language = entry.panic_file?.endsWith('.rs') ? 'rust' : 'plaintext';
                  const highlightedLines = panicSnippet.content.split('\n').map(line => 
                    highlightCode(line, language)
                  );
                  return highlightedLines.map((line, i) => {
                    const lineNum = panicSnippet.start_line + i;
                    const isHighlight = lineNum === panicSnippet.highlight_line;
                    return (
                      <div key={i} class={`snippet-line ${isHighlight ? 'highlight' : ''}`}>
                        <span class="line-number">{lineNum}</span>
                        <code dangerouslySetInnerHTML={{ __html: line || ' ' }} />
                      </div>
                    );
                  });
                })()}
              </pre>
            </div>
          </div>
        )}
        
        {/* Show relevant caller frames for panics (instead of full backtrace) */}
        {isPanic && backtraceFrames.length > 0 && (
          <details class="entry-callers" open={expandAll !== null ? expandAll : true}>
            <summary class="callers-summary">📋 Call Stack ({backtraceFrames.length} frames)</summary>
            <div class="callers-list">
              {backtraceFrames.map((frame, i) => (
                <div key={i} class="caller-frame">
                  <span class="frame-index">{frame.index}</span>
                  <span class="frame-function">{frame.function}</span>
                  {frame.location && (
                    <span class="frame-location">at {frame.location}</span>
                  )}
                </div>
              ))}
            </div>
          </details>
        )}
        
        {/* Full backtrace - only for non-panic errors */}
        {entry.backtrace && !isPanic && (
          <details class="entry-backtrace" open={expandAll !== null ? expandAll : entry.level === 'ERROR'}>
            <summary class="backtrace-summary">📚 Stack Trace</summary>
            <pre class="backtrace-content">{entry.backtrace}</pre>
          </details>
        )}
        
        {/* Fields rendered as Rust-style objects */}
        {hasFields && (
          <details class="entry-fields-rust" open={detailsOpen}>
            <summary>{fieldEntries.length} field{fieldEntries.length !== 1 ? 's' : ''}</summary>
            <div class="fields-rust-container">
              <FieldsRenderer fields={entry.fields} defaultExpanded={detailsOpen} />
            </div>
          </details>
        )}
        
        {/* Source snippet toggle */}
        {hasLocation && (
          <div class="entry-source">
            <button class="snippet-toggle" onClick={toggleSnippet}>
              {snippetVisible ? '▼' : '▶'} Source
            </button>
            {snippetVisible && snippet && (
              <div class="code-snippet">
                <pre class="snippet-code">
                  {(() => {
                    // Highlight the entire snippet as Rust
                    const language = entry.file?.endsWith('.rs') ? 'rust' : 'plaintext';
                    const highlightedLines = snippet.content.split('\n').map(line => 
                      highlightCode(line, language)
                    );
                    return highlightedLines.map((line, i) => {
                      const lineNum = snippet.start_line + i;
                      const isHighlight = lineNum === snippet.highlight_line;
                      return (
                        <div key={i} class={`snippet-line ${isHighlight ? 'highlight' : ''}`}>
                          <span class="line-number">{lineNum}</span>
                          <code dangerouslySetInnerHTML={{ __html: line || ' ' }} />
                        </div>
                      );
                    });
                  })()}
                </pre>
              </div>
            )}
            {snippetVisible && snippetError && (
              <div class="snippet-error">{snippetError}</div>
            )}
          </div>
        )}
        
        {/* Raw output */}
        {showRaw && (
          <pre class="entry-raw">{entry.raw}</pre>
        )}
      </div>
      
      {/* Right side metadata */}
      <div class="entry-meta">
        <span class="meta-line">#{entry.line_number}</span>
        {entry.timestamp && (
          <span class="meta-timestamp">{formatTimestamp(entry.timestamp)}</span>
        )}
        {hasLocation && (
          <button class="meta-location" onClick={handleLocationClick} title={`${entry.file}:${entry.source_line}`}>
            📍 {entry.file?.split(/[/\\]/).pop()}:{entry.source_line}
          </button>
        )}
      </div>
    </div>
  );
}
