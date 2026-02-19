import { useState, useEffect } from 'preact/hooks';
import type { LogEntry, SourceSnippet } from '../../types';
import { openSourceFile } from '../../store';
import * as api from '../../api';
import { CompactJsonViewer } from './JsonViewer';

interface Props {
  entry: LogEntry;
  showRaw: boolean;
  searchQuery: string;
  isSelected: boolean;
  onSelect: () => void;
}

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
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

export function LogEntryRow({ entry, showRaw, searchQuery, isSelected, onSelect }: Props) {
  const [snippet, setSnippet] = useState<SourceSnippet | null>(null);
  const [snippetError, setSnippetError] = useState<string | null>(null);
  const [showSnippet, setShowSnippet] = useState(true);

  const hasLocation = entry.file && entry.source_line;
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

  const toggleSnippet = () => {
    setShowSnippet(!showSnippet);
  };

  const handleLocationClick = (e: MouseEvent) => {
    e.stopPropagation();
    if (entry.file) {
      openSourceFile(entry.file, entry.source_line ?? undefined);
    }
  };

  // Render fields as key=value pairs
  const fieldEntries = Object.entries(entry.fields);
  const importantFields = fieldEntries.filter(([k]) => 
    ['busy', 'idle', 'start_token', 'end', 'result'].includes(k)
  );
  const otherFields = fieldEntries.filter(([k]) => 
    !['busy', 'idle', 'start_token', 'end', 'result'].includes(k)
  );

  return (
    <div 
      class={`log-entry ${isSelected ? 'selected' : ''} level-${levelClass} type-${typeClass}`}
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
        </div>
        
        {/* Main message - prominently displayed */}
        <div 
          class="entry-message"
          dangerouslySetInnerHTML={{ __html: highlightMatch(entry.message, searchQuery) }}
        />
        
        {/* Important fields inline */}
        {importantFields.length > 0 && (
          <div class="entry-fields-inline">
            {importantFields.map(([k, v]) => (
              <div key={k} class="field-inline">
                <span class="field-key">{k}</span>=
                <span class="field-value"><CompactJsonViewer value={v} /></span>
              </div>
            ))}
          </div>
        )}
        
        {/* Other fields expanded by default */}
        {otherFields.length > 0 && (
          <details class="entry-fields-details" open>
            <summary>{otherFields.length} more fields</summary>
            <div class="entry-fields-list">
              {otherFields.map(([k, v]) => (
                <div key={k} class="field-item-row">
                  <span class="field-key">{k}</span>
                  <span class="field-value"><CompactJsonViewer value={v} /></span>
                </div>
              ))}
            </div>
          </details>
        )}
        
        {/* Backtrace for panic/error entries */}
        {entry.backtrace && (
          <details class="entry-backtrace" open={entry.level === 'ERROR'}>
            <summary class="backtrace-summary">📚 Stack Trace</summary>
            <pre class="backtrace-content">{entry.backtrace}</pre>
          </details>
        )}
        
        {/* Source snippet toggle */}
        {hasLocation && (
          <div class="entry-source">
            <button class="snippet-toggle" onClick={toggleSnippet}>
              {showSnippet ? '▼' : '▶'} Source
            </button>
            {showSnippet && snippet && (
              <div class="code-snippet">
                <pre class="snippet-code">
                  {snippet.content.split('\n').map((line, i) => {
                    const lineNum = snippet.start_line + i;
                    const isHighlight = lineNum === snippet.highlight_line;
                    return (
                      <div key={i} class={`snippet-line ${isHighlight ? 'highlight' : ''}`}>
                        <span class="line-number">{lineNum}</span>
                        <code>{line}</code>
                      </div>
                    );
                  })}
                </pre>
              </div>
            )}
            {showSnippet && snippetError && (
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
