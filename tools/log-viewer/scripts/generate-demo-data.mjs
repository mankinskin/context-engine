#!/usr/bin/env node
/**
 * Generate pre-parsed JSON demo data from raw .log files.
 *
 * This mirrors the Rust LogParser logic so the static frontend
 * can load data without a running backend server.
 *
 * Usage:
 *   node scripts/generate-demo-data.mjs [input-dir] [output-dir]
 *
 * Defaults:
 *   input-dir:  ../demo-logs
 *   output-dir: frontend/public/data
 */

import { readFileSync, writeFileSync, readdirSync, mkdirSync, existsSync } from 'fs';
import { join, basename, resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const inputDir = resolve(process.argv[2] || join(__dirname, '..', 'demo-logs'));
const outputDir = resolve(process.argv[3] || join(__dirname, '..', 'frontend', 'public', 'data'));

// ── ANSI strip ──
const ANSI_RE = /\x1b\[[0-9;]*m/g;
const strip = s => s.replace(ANSI_RE, '');

// ── Panic / assertion parsing ──
const PANIC_RE = /panicked at ([^:]+):(\d+):(\d+):/;
const DIFF_RE = /assertion failed: `\(left == right\)`[\s\S]*?Diff < (\w+) \/ (\w+) >\s*:\s*([\s\S]+)/;

function parsePanicLocation(msg) {
  const m = PANIC_RE.exec(msg);
  return m ? { file: m[1], line: +m[2] } : null;
}

function parseAssertionDiff(msg) {
  const m = DIFF_RE.exec(msg);
  if (!m) return null;
  const lines = m[3].split('\n');
  const left = [], right = [];
  for (const l of lines) {
    if (l.startsWith('<')) left.push(l.slice(1));
    else if (l.startsWith('>')) right.push(l.slice(1));
  }
  return {
    title: 'assertion failed: `(left == right)`',
    left_label: m[1], right_label: m[2],
    left_value: left.join('\n'), right_value: right.join('\n'),
  };
}

// ── Parse one tracing JSON object into a LogEntry ──
function toLogEntry(index, json) {
  const level = (json.level || 'INFO').toUpperCase();
  const fields = {};
  let message = '';
  let backtrace = null;
  let panicFile = null, panicLine = null;

  // Extract from fields
  if (json.fields && typeof json.fields === 'object') {
    for (const [k, v] of Object.entries(json.fields)) {
      if (k === 'message') {
        message = strip(typeof v === 'string' ? v : JSON.stringify(v));
      } else if (k === 'backtrace') {
        backtrace = typeof v === 'string' ? v : JSON.stringify(v);
      } else if (k === 'panic_file') {
        panicFile = typeof v === 'string' ? v : null;
      } else if (k === 'panic_line') {
        panicLine = typeof v === 'number' ? v : null;
      } else if (k !== 'panic_column') {
        fields[k] = v;
      }
    }
  }

  // Span info
  const span = json.span || null;
  const spans = json.spans || null;
  const spanName = span?.name || (spans && spans.length > 0 ? spans[spans.length - 1]?.name : null) || null;
  const depth = spans ? spans.length : 0;

  // Merge span fields
  if (span) {
    for (const [k, v] of Object.entries(span)) {
      if (k !== 'name') fields[k] = v;
    }
  }

  // Event type detection
  let eventType = 'event';
  if (span?.name) {
    if (message.includes('new')) eventType = 'span_new';
    else if (message.includes('enter')) eventType = 'span_enter';
    else if (message.includes('close') || message.includes('exit')) eventType = 'span_exit';
  }

  if (!message) {
    if (spanName) { message = `Span: ${spanName}`; eventType = 'span_enter'; }
    else message = json.target || 'Unknown';
  }

  // Panic location
  if (!panicFile && message.includes('panicked at')) {
    const loc = parsePanicLocation(message);
    if (loc) { panicFile = loc.file; panicLine = loc.line; }
  }

  return {
    line_number: index,
    level,
    timestamp: json.timestamp || null,
    message,
    event_type: eventType,
    span_name: spanName,
    depth,
    fields,
    file: json.filename || null,
    source_line: json.line_number || null,
    panic_file: panicFile,
    panic_line: panicLine,
    assertion_diff: parseAssertionDiff(message),
    backtrace,
    raw: `[${level}] ${message}`,
  };
}

// ── Streaming JSON parse (handles pretty-printed consecutive objects) ──
function parseLogContent(text) {
  const entries = [];
  // Quick approach: split on top-level JSON boundaries
  // The format is consecutive JSON objects (possibly pretty-printed)
  let depth = 0;
  let start = -1;
  let inString = false;
  let escape = false;
  let entryIndex = 0;

  for (let i = 0; i < text.length; i++) {
    const ch = text[i];
    if (escape) { escape = false; continue; }
    if (ch === '\\' && inString) { escape = true; continue; }
    if (ch === '"') { inString = !inString; continue; }
    if (inString) continue;
    if (ch === '{') {
      if (depth === 0) start = i;
      depth++;
    } else if (ch === '}') {
      depth--;
      if (depth === 0 && start >= 0) {
        const block = text.slice(start, i + 1);
        try {
          const json = JSON.parse(block);
          entryIndex++;
          entries.push(toLogEntry(entryIndex, json));
        } catch {
          entryIndex++;
          entries.push({
            line_number: entryIndex,
            level: 'ERROR', timestamp: null,
            message: `JSON parse error at byte ${start}`,
            event_type: 'error', span_name: null, depth: 0,
            fields: {}, file: null, source_line: null,
            panic_file: null, panic_line: null,
            assertion_diff: null, backtrace: null,
            raw: `Parse error at entry ${entryIndex}`,
          });
          break;
        }
        start = -1;
      }
    }
  }
  return entries;
}

// ── Main ──
if (!existsSync(inputDir)) {
  console.error(`Input directory not found: ${inputDir}`);
  process.exit(1);
}

mkdirSync(outputDir, { recursive: true });

const logFiles = readdirSync(inputDir).filter(f => f.endsWith('.log')).sort();
const manifest = [];

for (const file of logFiles) {
  const content = readFileSync(join(inputDir, file), 'utf-8');
  const entries = parseLogContent(content);
  const hasGraphSnapshot = entries.some(e => e.message === 'graph_snapshot');

  const name = file;
  const response = { name, entries, total_lines: content.split('\n').length };

  const outFile = file.replace(/\.log$/, '.json');
  writeFileSync(join(outputDir, outFile), JSON.stringify(response));

  manifest.push({
    name,
    size: content.length,
    modified: null,
    has_graph_snapshot: hasGraphSnapshot,
  });

  console.log(`  ${file} → ${outFile}  (${entries.length} entries, graph: ${hasGraphSnapshot})`);
}

writeFileSync(join(outputDir, 'manifest.json'), JSON.stringify(manifest, null, 2));
console.log(`\nGenerated ${logFiles.length} files + manifest.json in ${outputDir}`);
