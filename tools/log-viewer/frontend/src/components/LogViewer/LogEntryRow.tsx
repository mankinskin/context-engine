import { useState, useEffect } from 'preact/hooks';
import type { LogEntry, SourceSnippet } from '../../types';
import { openSourceFile } from '../../store';
import * as api from '../../api';
import Prism from 'prismjs';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-json';
import { FieldsRenderer } from './RustValueRenderer';
import { Flame, LocationPin, ChevronDown, ChevronRight } from '../Icons';

interface Props {
  entry: LogEntry;
  showRaw: boolean;
  searchQuery: string;
  isSelected: boolean;
  onSelect: () => void;
  expandAll: boolean;
  isExpanded?: boolean; // Shared expand state from parent
  onToggleExpand?: () => void; // Toggle expand callback
  headerCellRef?: (el: HTMLDivElement | null) => void; // Ref callback for header scroll sync
  headerScrollLeft?: number; // Current horizontal scroll position
  headerColWidth?: number; // Header column width
  onHeaderWheel?: (e: WheelEvent) => void; // Wheel scroll handler for header column
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

export function LogEntryRow({ entry, showRaw, searchQuery, isSelected, onSelect, expandAll, isExpanded, onToggleExpand, headerCellRef, headerScrollLeft = 0, headerColWidth: _headerColWidth = 300, onHeaderWheel }: Props) {
  const [snippet, setSnippet] = useState<SourceSnippet | null>(null);
  const [snippetError, setSnippetError] = useState<string | null>(null);
  const [panicSnippet, setPanicSnippet] = useState<SourceSnippet | null>(null);
  const [panicSnippetError, setPanicSnippetError] = useState<string | null>(null);
  // Local state for legacy mode
  const [localExpanded, setLocalExpanded] = useState(false);
  
  // Use shared state if available, otherwise local
  const expanded = isExpanded !== undefined ? isExpanded : localExpanded;
  
  // Ref for measuring content width
  const headerContentRef = { current: null as HTMLDivElement | null };

  const hasLocation = entry.file && entry.source_line;
  const hasPanicLocation = entry.panic_file && entry.panic_line;
  const levelClass = entry.level.toLowerCase();
  const typeClass = entry.event_type.replace('_', '-');
  
  // Calculate indentation for spans (1.2em per level, ~13px at 11px font)
  const indentLevel = Math.min(entry.depth, 10);
  const indentEm = indentLevel * 1.2;

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

  // Unified expanded state
  const showDetails = expandAll || expanded;

  // Has expandable header content? (fields or backtrace for non-panics)
  const hasHeaderExpand = hasFields || (entry.backtrace && !isPanic);
  // Has expandable viewport content? (snippets, panic info, raw)
  const hasViewportExpand = (hasLocation && snippet) || (isPanic && (panicSnippet || entry.assertion_diff || backtraceFrames.length > 0)) || showRaw;
  
  // Any expandable content at all?
  const hasAnyExpandable = hasHeaderExpand || hasViewportExpand;

  // Click handler: expand all expandables + select
  const handleEntryClick = () => {
    onSelect();
    if (hasAnyExpandable) {
      if (onToggleExpand) {
        onToggleExpand();
      } else {
        setLocalExpanded(!localExpanded);
      }
    }
  };
  
  // Toggle handler for expand buttons
  const handleToggle = (e: MouseEvent) => {
    e.stopPropagation();
    if (onToggleExpand) {
      onToggleExpand();
    } else {
      setLocalExpanded(!localExpanded);
    }
  };

  // Render as flex row with both columns
  return (
    <div 
      class={`log-entry ${isSelected ? 'selected' : ''} level-${levelClass} type-${typeClass} ${isPanic ? 'panic-entry' : ''} ${hasAnyExpandable ? 'expandable' : ''}`}
      onClick={handleEntryClick}
    >
      {/* Header Column - entry metadata and message */}
      <div class="entry-header-cell" onWheel={onHeaderWheel as any}>
        <div 
          class="entry-header-content"
          ref={(el) => {
            headerContentRef.current = el;
            if (headerCellRef) headerCellRef(el);
          }}
          style={{ transform: `translateX(-${headerScrollLeft}px)` }}
        >
          <div class="entry-header-col" style={{ paddingLeft: `${indentEm + 0.5}em` }}>
            {/* Span depth indicator */}
            {indentLevel > 0 && (
              <div class="depth-indicator" style={{ left: '0', width: `${indentEm}em` }}>
                {Array.from({ length: indentLevel }).map((_, i) => (
                  <span key={i} class="depth-line"></span>
                ))}
              </div>
            )}
            <div class="header-row1">
              <span class={`level-badge ${levelClass}`}>{entry.level}</span>
              <span class={`type-badge ${typeClass}`}>{entry.event_type === 'span_enter' ? 'ENTER' : entry.event_type === 'span_exit' ? 'EXIT' : 'EVENT'}</span>
              {isPanic && <span class="panic-badge"><Flame size={8} /></span>}
              <span class="entry-meta">#{entry.line_number}</span>
              {entry.timestamp && <span class="entry-meta">{formatTimestamp(entry.timestamp)}</span>}
            </div>
            <div class="header-row2">
              {entry.span_name && <span class="span-name">{entry.span_name}</span>}
              {isPanic ? (
                <span class="entry-message panic-msg" dangerouslySetInnerHTML={{ __html: highlightMatch(entry.message, searchQuery) }} />
              ) : (
                <span class="entry-message" dangerouslySetInnerHTML={{ __html: highlightMatch(entry.message, searchQuery) }} />
              )}
            </div>
            <div class="header-row3">
              {hasFields && <span class="content-meta">{fieldEntries.length} {fieldEntries.length === 1 ? 'field' : 'fields'}</span>}
              {hasLocation && (
                <button class="header-location" onClick={handleLocationClick} title={`${entry.file}:${entry.source_line}`}>
                  <LocationPin size={8} />{entry.file?.split(/[/\\]/).pop()}:{entry.source_line}
                </button>
              )}
            </div>
            {showDetails && (
              <div class="header-details" onClick={(e) => e.stopPropagation()}>
                {/* Fields */}
                {hasFields && (
                  <div class="fields-rust-container">
                    <FieldsRenderer fields={entry.fields} defaultExpanded={true} />
                  </div>
                )}
                {/* Backtrace in header for non-panics */}
                {entry.backtrace && !isPanic && (
                  <pre class="backtrace-content">{entry.backtrace}</pre>
                )}
              </div>
            )}
          </div>
        </div>
        {/* Expand toggle - positioned outside scrolling content to stick at right */}
        {hasAnyExpandable && (
          <button class="header-expand-toggle" onClick={handleToggle}>
            {showDetails ? <ChevronDown size={8} /> : <ChevronRight size={8} />}
          </button>
        )}
      </div>
      
      {/* Viewport Column - source code and visualizations */}
      <div class="entry-viewport-cell">
        <div class="entry-viewport-col">
          {hasViewportExpand && (
            <div class="viewport-header">
              <span class="viewport-label">Source</span>
              <button class="col-toggle" onClick={handleToggle}>
                {showDetails ? <ChevronDown size={8} /> : <ChevronRight size={8} />}
              </button>
            </div>
          )}
          {showDetails && (
            <div class="viewport-content" onClick={(e) => e.stopPropagation()}>
              {/* Panic assertion diff */}
              {isPanic && entry.assertion_diff && (
                <PanicMessageRenderer 
                  message={entry.message} 
                  assertionDiff={entry.assertion_diff}
                  searchQuery={searchQuery} 
                />
              )}
              
              {/* Panic source snippet */}
              {isPanic && hasPanicLocation && panicSnippet && (
                <div class="panic-source">
                  <div class="panic-location">
                    <LocationPin size={8} /> {entry.panic_file}:{entry.panic_line}
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
              
              {/* Panic call stack */}
              {isPanic && backtraceFrames.length > 0 && (
                <div class="callers-list">
                  {backtraceFrames.map((frame, i) => (
                    <div key={i} class="caller-frame">
                      <span class="frame-index">{frame.index}</span>
                      <span class="frame-function">{frame.function}</span>
                      {frame.location && <span class="frame-location">at {frame.location}</span>}
                    </div>
                  ))}
                </div>
              )}
              
              {/* Source snippet */}
              {hasLocation && snippet && (
                <div class="code-snippet">
                  <pre class="snippet-code">
                    {(() => {
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
              {hasLocation && snippetError && <span class="snippet-error">{snippetError}</span>}
              
              {/* Raw */}
              {showRaw && <pre class="entry-raw">{entry.raw}</pre>}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
