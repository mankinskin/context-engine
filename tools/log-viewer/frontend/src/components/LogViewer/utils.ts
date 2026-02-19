/**
 * Utility functions for LogViewer components
 */

export function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

export function highlightMatch(text: string, query: string): string {
  if (!query) return escapeHtml(text);
  try {
    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    return escapeHtml(text).replace(regex, '<mark class="highlight">$1</mark>');
  } catch {
    return escapeHtml(text);
  }
}

export function formatTimestamp(ts: string | null): string {
  if (!ts) return '';
  const num = parseFloat(ts);
  if (num < 1) return `${(num * 1000).toFixed(0)}ms`;
  return `${num.toFixed(2)}s`;
}

export interface BacktraceFrame {
  index: number;
  function: string;
  location?: string;
}

/**
 * Parse backtrace to extract frames
 */
export function parseBacktrace(backtrace: string): BacktraceFrame[] {
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
export function getRelevantFrames(frames: BacktraceFrame[]): BacktraceFrame[] {
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
