---
tags: `#context-api` `#phase3.1` `#tracing` `#logs` `#cli` `#mcp` `#jq`
summary: Phase 3.1 — Per-command tracing log capture in context-cli and context-mcp, plus log query/analysis tools
status: design
---

# Plan: context-api Phase 3.1 — Per-Command Tracing & Log Access

## Objective

Add per-command tracing log capture to the `context-cli` and `context-mcp` binaries, and expose log query/analysis commands through the existing `Command`/`CommandResult` system. Every `execute()` call should optionally produce a structured JSON log file in the workspace directory. Agents and users can then list, query (JQ), analyze, and read those logs through the same interfaces they use for graph operations.

## Context

### Problem Statement

Currently, the engine crates (`context-search`, `context-insert`) emit rich structured tracing events via `GraphOpEvent::emit()` — search state transitions, split/join phases, visualization data with path graphs and graph mutations. However:

1. **CLI users see nothing** — `context-cli` initializes a basic `tracing_subscriber::fmt()` with `EnvFilter`, so logs only appear if `RUST_LOG` is set, and they go to stderr as unstructured text.
2. **MCP agents see nothing** — `context-mcp` sends tracing to stderr. Agents have no access to the rich structured events that explain *why* an insert produced certain results or *how* a search traversed the graph.
3. **No per-command log isolation** — Even when tracing is enabled, all events go to a single output. There's no way to get "show me the trace for that specific insert_sequence call I just made."
4. **Existing log-viewer is disconnected** — `tools/log-viewer` can query log files, but only from `target/test-logs/`. There's no integration with workspace-scoped logs.

### What Already Exists

| Component | What it does | Where |
|-----------|-------------|-------|
| `GraphOpEvent::emit()` | Emits `tracing::info!(graph_op = %json, ...)` for search/insert/read visualization events | `crates/context-trace/src/graph/visualization.rs` |
| `TestTracing` | Per-test JSON log files via `PrettyJsonWriter`, auto-cleanup, `SignatureStore` | `crates/context-trace/src/logging/tracing_utils/test_tracing.rs` |
| `PrettyJsonWriter` | Wraps `tracing-subscriber` JSON output with indentation; currently `pub(super)` | `crates/context-trace/src/logging/tracing_utils/writers.rs` |
| `FlushingWriter` | Auto-flushing `std::fs::File` wrapper; currently `pub(super)` | `crates/context-trace/src/logging/tracing_utils/writers.rs` |
| `LogParser` | Parses pretty-printed JSON log files using `serde_json::Deserializer::from_str().into_iter()` → `Vec<LogEntry>` | `tools/log-viewer/src/log_parser.rs` |
| `JqFilter` | Compiles and runs JQ expressions via `jaq-core`/`jaq-interpret`/`jaq-std` | `tools/viewer-api/src/query.rs` |
| `LogServer` (MCP) | 6-tool MCP server for querying test log files | `tools/log-viewer/src/mcp_server.rs` |
| `context-cli` REPL | `rustyline` REPL with `split_whitespace` parsing, `current_workspace` tracking | `tools/context-cli/src/repl.rs` |
| `context-mcp` | 3-tool MCP server: `execute`, `help`, `workflow` | `tools/context-mcp/src/server.rs` |
| Insert visualization thread-locals | `INSERT_STEP`, `INSERT_PATH_ID`, `INSERT_VIZ_PATH` — per-thread state for step counting and path accumulation | `crates/context-insert/src/visualization.rs` |

### Key Insight: Reuse the Existing Tracing Infrastructure

The test tracing system already knows how to:
- Set up a per-scope `tracing::Dispatch` with JSON file output via `PrettyJsonWriter`
- Capture `GraphOpEvent` structured events (search transitions, insert split/join, etc.)
- Produce the exact log format that `LogParser` can parse

The log-viewer already knows how to:
- Parse those JSON log files into `Vec<LogEntry>`
- Run JQ queries against them
- Produce analysis summaries (counts by level, span summaries, errors)

We don't need to build a new log format or parser. We need to:
1. **Expose** the tracing capture infrastructure from `context-trace` so `context-api` can build scoped dispatchers
2. **Wire** per-command capture into the CLI/MCP execution path
3. **Add** log access commands to the `Command` enum, with the log parser and types living in `context-api` (the canonical domain crate)
4. **Have** `viewer-api` and `log-viewer` consume the log types from `context-api` instead of defining their own

### Prerequisites

- **Phase 3 complete** — `context-mcp` exists with `execute`, `help`, `workflow` tools.
- **Phase 1 & 2 complete** — `context-api` with full `Command`/`CommandResult`/`execute()` dispatch.

### Relationship to Other Phases

**Phase 4 (HTTP adapter):** The log commands added here to the `Command` enum will automatically be available through the HTTP adapter via `POST /api/execute`. Phase 4 can add REST convenience endpoints:
- `GET /api/workspaces/:name/logs` → `ListLogs`
- `GET /api/workspaces/:name/logs/:filename` → `GetLog`
- `POST /api/workspaces/:name/logs/:filename/query` → `QueryLog`

Phase 4's plan calls for `context-http` depending on both `context-api` and `viewer-api`. Since `viewer-api` will itself depend on `context-api` after Phase 3.1, the HTTP adapter gets log parsing and JQ for free.

**Phase 5 (TypeScript types):** The new log types (`LogFileInfo`, `LogEntryInfo`, `LogAnalysis`, `TraceSummary`, etc.) should receive `#[cfg_attr(feature = "ts-gen", derive(TS))]` annotations consistent with Phase 5's hybrid approach for ts-rs generation. These types live in `context-api/src/types.rs` and will be exported to `@context-engine/types`.

### Key Dependency Direction Decision

The original draft proposed `context-api → viewer-api` (API depends on the viewer shared lib). This is **wrong**. The correct dependency direction is:

```text
viewer-api ──→ context-api     (viewer-api consumes log types from the API)
log-viewer ──→ viewer-api      (log-viewer uses viewer-api's JQ + the re-exported types)
```

**Rationale:**
- `context-api` is the **canonical domain crate** — all domain types (including log types and the log parser) belong there.
- `viewer-api` is a **shared infrastructure crate** for viewer tools. It currently has zero domain knowledge. Adding a dependency on `context-api` is clean — no circular dependency exists. `viewer-api` gains access to the log types and can re-export them alongside `JqFilter`.
- This means `context-api` does **not** depend on `viewer-api` at all. It has its own log parser (moved from `log-viewer`) and its own JQ dependency (the `jaq-*` crates are small and pure Rust).
- `log-viewer` switches from its local `LogParser`/`LogEntry` to importing from `viewer-api`, which re-exports from `context-api`.

```text
DEPENDENCY GRAPH (after Phase 3.1):

crates/context-api
  ├── context-trace  (graph types, tracing infra, PrettyJsonWriter)
  ├── context-search
  ├── context-insert
  ├── context-read
  ├── jaq-core, jaq-std, jaq-interpret  (JQ engine, directly)
  └── serde_json  (streaming log parser)

tools/viewer-api
  ├── context-api  (NEW — for LogEntry, LogParser, log types)
  ├── jaq-* (still here for JqFilter, shared with context-api)
  ├── axum, tokio, rmcp, etc.
  └── ...

tools/log-viewer
  └── viewer-api  (gets LogParser + JqFilter + log types transitively)

tools/context-cli
  └── context-api  (gets everything: commands, log parser, tracing capture)

tools/context-mcp
  ├── context-api
  └── rmcp
```

### Files Affected

**Modified (existing):**
- `crates/context-trace/src/logging/tracing_utils/writers.rs` — change `pub(super)` → `pub` on `PrettyJsonWriter`, `FlushingWriter`
- `crates/context-trace/src/logging/tracing_utils/debug_to_json.rs` — change `pub(super)` → `pub` on `SignatureStore` type alias, `new_signature_store()`
- `crates/context-trace/src/logging/tracing_utils/mod.rs` — re-export writers and debug_to_json
- `crates/context-trace/src/logging/mod.rs` — re-export the new public items
- `crates/context-api/Cargo.toml` — add `jaq-core`, `jaq-std`, `jaq-interpret`, `jaq-syn`
- `crates/context-api/src/commands/mod.rs` — add log `Command` variants, `CommandResult` variants, dispatch arms
- `crates/context-api/src/types.rs` — add log-related types
- `crates/context-api/src/workspace/` — add log directory management
- `tools/viewer-api/Cargo.toml` — add `context-api` dependency
- `tools/viewer-api/src/lib.rs` — re-export log parser and types from `context-api`
- `tools/log-viewer/src/log_parser.rs` — remove (replaced by `context-api::log_parser`)
- `tools/log-viewer/src/mcp_server.rs` — update imports to use `viewer_api` re-exports
- `tools/log-viewer/src/handlers.rs` — update imports
- `tools/context-cli/src/repl.rs` — add log REPL commands
- `tools/context-cli/src/main.rs` — add log subcommands, `--trace` flag
- `tools/context-cli/src/output.rs` — add log output formatting
- `tools/context-mcp/src/server.rs` — add `trace` flag to `ExecuteInput`, log commands to `help`/`workflow`

**New:**
- `crates/context-api/src/log_parser.rs` — log file parser (moved from `log-viewer`, adapted)
- `crates/context-api/src/jq.rs` — thin JQ wrapper around `jaq-*` (like `viewer-api/src/query.rs`)
- `crates/context-api/src/commands/logs.rs` — log command implementations
- `crates/context-api/src/tracing_capture.rs` — per-command trace capture infrastructure

---

## Analysis

### Architecture: Per-Command Trace Capture

The core challenge is: `context-api::execute()` is a synchronous function that calls into engine crates. The engine crates emit `tracing` events via the standard `tracing::info!` macro, which dispatches to the **thread-local default dispatcher** (falling back to the global default). We need to capture those events *per-call* into a file, without interfering with the binary's own tracing setup.

**Solution: Scoped tracing dispatcher via `tracing::dispatcher::with_default()`**

This sets a dispatcher for the duration of a closure, replacing the thread-local default. The engine's `GraphOpEvent::emit()` calls `tracing::info!(...)`, which will be captured by our scoped dispatcher. When the closure returns, the previous dispatcher is restored.

This is the exact same mechanism used by `TestTracing::init_internal()`, which calls `tracing::dispatcher::set_default()` and holds the `DefaultGuard` for the test's lifetime.

```text
┌──────────────────────────────────────────────────────────────────┐
│  context-cli / context-mcp                                       │
│                                                                   │
│  Global subscriber (stderr diagnostics)                           │
│  │                                                                │
│  │  ┌────────────────────────────────────────────┐                │
│  │  │ with_default(capture_dispatch, || {         │                │
│  │  │   // Thread-local dispatch → JSON log file  │                │
│  │  │   execute(manager, cmd)                     │                │
│  │  │   // GraphOpEvent::emit() captured here     │ → writes to   │
│  │  │   // tracing::info! captured here           │   .context-   │
│  │  │ })                                          │   engine/ws/  │
│  │  │ // Previous dispatch restored               │   logs/*.log  │
│  │  └────────────────────────────────────────────┘                │
│  │                                                                │
│  │  // Global subscriber active again                             │
│  │                                                                │
│  │  execute(manager, ListLogs { workspace })    ← reads logs      │
│  │  execute(manager, QueryLog { workspace, jq })← JQ filter       │
│  │  execute(manager, AnalyzeLog { workspace })  ← analysis        │
│  └────────────────────────────────────────────────────────────────│
└──────────────────────────────────────────────────────────────────┘
```

**Important behavioral note:** `with_default` *replaces* (does not layer on top of) the thread-local default. This means the global/CLI stderr subscriber is suppressed during the captured call. This is acceptable and even desirable — the captured events go to the log file, not to stderr. After the closure, the CLI's normal stderr output resumes.

### Thread-Local State Interaction

The insert visualization module uses thread-locals (`INSERT_STEP`, `INSERT_PATH_ID`, `INSERT_VIZ_PATH`) for step counting and path accumulation. These are **orthogonal** to the tracing dispatcher — they store visualization state, not subscriber state. The scoped dispatcher only changes *where* events go, not *how* they're emitted. No interaction issues.

### Log File Location & Naming

Workspace logs live inside the workspace's persistence directory:

```text
.context-engine/
  <workspace-name>/
    graph.bin           (existing — persisted graph)
    logs/
      20260314T005530123_insert_sequence.log
      20260314T005531456_search_sequence.log
      20260314T005532789_insert_sequence.log
      ...
```

Naming convention: `<YYYYMMDD>T<HHMMSS><millis>_<command_name>.log`

Millisecond precision avoids filename collisions for rapid automated sequences. Logs are workspace-scoped and automatically cleaned up with `delete_workspace`.

### Log Access Commands

New `Command` variants that integrate into the existing dispatch system:

| Command | Parameters | Result | Description |
|---------|-----------|--------|-------------|
| `ListLogs` | `workspace`, `pattern?`, `limit?` | `LogList` | List log files for a workspace |
| `GetLog` | `workspace`, `filename`, `filter?`, `limit?`, `offset?` | `LogEntries` | Read a log file with optional JQ filter and pagination |
| `QueryLog` | `workspace`, `filename`, `query` (JQ), `limit?` | `LogQueryResult` | Run a JQ query against a log file |
| `AnalyzeLog` | `workspace`, `filename` | `LogAnalysis` | Statistics: entry counts by level/event type, error summary, span summary |
| `SearchLogs` | `workspace`, `query` (JQ), `limit_per_file?` | `LogSearchResult` | Search across all log files in a workspace |
| `DeleteLog` | `workspace`, `filename` | `Ok` | Delete a specific log file |
| `DeleteLogs` | `workspace`, `older_than_days?` | `LogDeleteResult` | Delete logs, optionally only those older than N days |

### Tracing Toggle

Not every command needs tracing. Adding atoms doesn't produce interesting events. The capture is:

1. **Opt-in at the adapter level** — The CLI and MCP server decide whether to enable capture per-call.
2. **Configurable** — A per-session or per-call toggle.
3. **Cheap when off** — Zero overhead when tracing capture is disabled (just a boolean check).

Proposed approach:
- `context-api` provides `execute_traced()` alongside `execute()` — adapters choose which to call
- CLI: `--trace` flag on subcommands, `trace on/off` REPL command, `CONTEXT_TRACE=1` env var
- MCP: `trace: true` optional field in `ExecuteInput`, `trace` field in `ExecuteOutput`

### MCP Response Integration

When tracing is enabled for an MCP `execute` call, the response includes a trace summary:

```json
{
  "success": true,
  "result": { ... },
  "trace": {
    "log_file": "20260314T005530123_insert_sequence.log",
    "entry_count": 47,
    "event_summary": {
      "search": 12,
      "insert": 35
    },
    "duration_ms": 23
  }
}
```

This lets agents see at a glance what happened without having to separately query the log file. For deep inspection, they call `GetLog` or `QueryLog`.

---

## Execution Steps

### Step 1: Expose Tracing Capture Infrastructure from `context-trace`

**Files:**
- `crates/context-trace/src/logging/tracing_utils/writers.rs`
- `crates/context-trace/src/logging/tracing_utils/debug_to_json.rs`
- `crates/context-trace/src/logging/tracing_utils/mod.rs`
- `crates/context-trace/src/logging/tracing_utils/special_fields.rs`
- `crates/context-trace/src/logging/mod.rs`

Currently, `PrettyJsonWriter`, `FlushingWriter`, and `SignatureStore` are `pub(super)` — visible only within the `tracing_utils` module. External crates like `context-api` cannot use them to build their own scoped dispatchers. We need to open up a public API for constructing capture dispatchers.

**Option A (minimal visibility change):** Change `pub(super)` to `pub` on the writer types and re-export through the module chain:

```rust
// crates/context-trace/src/logging/tracing_utils/writers.rs
// Change:
//   pub(super) struct FlushingWriter { ... }
//   pub(super) struct PrettyJsonWriter<W> { ... }
// To:
pub struct FlushingWriter { ... }
pub struct PrettyJsonWriter<W> { ... }
```

```rust
// crates/context-trace/src/logging/tracing_utils/mod.rs
// Add:
pub use writers::{FlushingWriter, PrettyJsonWriter};
pub use debug_to_json::{SignatureStore, new_signature_store};
pub use special_fields::SpecialFieldExtractor;
```

```rust
// crates/context-trace/src/logging/mod.rs
// Add:
pub use tracing_utils::{
    FlushingWriter,
    PrettyJsonWriter,
    SignatureStore,
    new_signature_store,
    SpecialFieldExtractor,
    SpanFieldFormatter,
};
```

**Option B (higher-level API — recommended):** In addition to exposing the raw types, add a builder function that constructs a complete capture `Dispatch`:

```rust
// crates/context-trace/src/logging/tracing_utils/capture.rs (NEW)

use std::path::Path;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tracing::Dispatch;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt};

use super::writers::{FlushingWriter, PrettyJsonWriter};
use super::debug_to_json::{self, SignatureStore};
use super::special_fields::SpecialFieldExtractor;

/// Handle returned from `build_capture_dispatch`.
/// Holds the dispatch and metadata; drop to finalize.
pub struct CaptureDispatch {
    /// The dispatch to use with `tracing::dispatcher::with_default`.
    pub dispatch: Dispatch,
    /// Shared event counter — read after the dispatch scope ends.
    pub event_count: Arc<AtomicUsize>,
    /// Signature store (optional, for fn_sig collection).
    pub signatures: SignatureStore,
}

/// Build a `tracing::Dispatch` that captures events to a JSON log file.
///
/// The returned dispatch writes pretty-printed JSON to `log_file_path`
/// in the same format as `TestTracing`, compatible with `LogParser`.
///
/// # Usage
///
/// ```ignore
/// let capture = build_capture_dispatch(&log_path, "TRACE")?;
/// let result = tracing::dispatcher::with_default(&capture.dispatch, || {
///     do_something_that_emits_tracing_events()
/// });
/// let count = capture.event_count.load(Ordering::Relaxed);
/// ```
pub fn build_capture_dispatch(
    log_file_path: &Path,
    level_filter: &str,
) -> Result<CaptureDispatch, Box<dyn std::error::Error + Send + Sync>> {
    let file = fs::File::create(log_file_path)?;
    let flushing = FlushingWriter::new(file);
    let signatures = debug_to_json::new_signature_store();
    let writer = PrettyJsonWriter::new(flushing, signatures.clone());

    let filter = EnvFilter::try_new(level_filter)
        .unwrap_or_else(|_| EnvFilter::new("TRACE"));

    let event_count = Arc::new(AtomicUsize::new(0));
    let counter = event_count.clone();

    // Build a counting wrapper layer
    // (The event count can be derived from PrettyJsonWriter's write count,
    //  or from a simple layer that increments on each event.)

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(move || writer.clone())
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_ansi(false)
        .json()
        .with_filter(filter);

    let registry = tracing_subscriber::registry();
    let dispatch = Dispatch::new(
        registry
            .with(SpecialFieldExtractor)
            .with(file_layer),
    );

    Ok(CaptureDispatch {
        dispatch,
        event_count,
        signatures,
    })
}
```

This gives `context-api` a single function to call — `build_capture_dispatch()` — without needing to understand the subscriber layer internals.

**Both options should be implemented:** Option A provides the raw building blocks for anyone who needs custom subscriber setups, and Option B provides the convenient high-level API that `context-api` will actually use.

**Validation:**
- `cargo check -p context-trace`
- Existing `TestTracing` tests still pass (no behavior change, only visibility)
- New unit test: call `build_capture_dispatch`, emit a `tracing::info!`, verify the file is written

---

### Step 2: Log Parser and JQ in `context-api`

**Files:**
- `crates/context-api/Cargo.toml` — add `jaq-core`, `jaq-std`, `jaq-interpret`, `jaq-syn`
- `crates/context-api/src/log_parser.rs` (new)
- `crates/context-api/src/jq.rs` (new)

Move the log parsing logic from `tools/log-viewer/src/log_parser.rs` into `crates/context-api/src/log_parser.rs`. This is the canonical home because:
- Log types are domain types that belong in the API crate
- The log parser is needed by `context-api`'s log commands
- Downstream consumers (`viewer-api`, `log-viewer`) re-import from here

The move is largely mechanical. Key changes during the move:
- Remove any `ts-rs` derives (those get added back in Phase 5 with `#[cfg_attr(feature = "ts-gen", derive(TS))]`)
- Add `schemars::JsonSchema` derives for MCP schema generation
- Keep the `LogEntry` struct and `LogParser` struct with their full field set
- Keep the streaming `serde_json::Deserializer` approach for parsing

Create `crates/context-api/src/jq.rs` with the JQ wrapper:

```rust
// Thin wrapper around jaq-* crates, modeled on viewer-api/src/query.rs

pub struct JqFilter { ... }

impl JqFilter {
    pub fn compile(query: &str) -> Result<Self, JqError> { ... }
    pub fn run(&self, input: &serde_json::Value) -> Vec<Result<serde_json::Value, String>> { ... }
    pub fn matches(&self, input: &serde_json::Value) -> bool { ... }
}

pub fn filter_values(values: &[serde_json::Value], query: &str) -> Result<Vec<serde_json::Value>, JqError> { ... }
```

**Validation:** `cargo check -p context-api`, unit tests for `LogParser::parse()` and `JqFilter::compile()`.

---

### Step 3: Log-Related API Types

**File:** `crates/context-api/src/types.rs` (modify)

Add types for log command results:

```rust
/// Information about a log file.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogFileInfo {
    pub filename: String,
    pub size: u64,
    pub modified: String,
    pub command: String,
}

/// A parsed log entry (mirrors LogEntry but with JsonSchema).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogEntryInfo {
    pub entry_number: usize,
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    pub message: String,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_name: Option<String>,
    pub depth: usize,
    #[schemars(with = "serde_json::Value")]
    pub fields: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_line: Option<usize>,
}

/// Analysis summary of a log file.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogAnalysis {
    pub total_entries: usize,
    pub by_level: HashMap<String, usize>,
    pub by_event_type: HashMap<String, usize>,
    pub spans: Vec<SpanSummary>,
    pub errors: Vec<LogEntryInfo>,
}

/// Summary of a tracing span.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpanSummary {
    pub name: String,
    pub count: usize,
    pub has_errors: bool,
}

/// Brief trace summary for inclusion in execute responses.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceSummary {
    pub log_file: String,
    pub entry_count: usize,
    pub event_summary: HashMap<String, usize>,
    pub duration_ms: u64,
}

/// Result of deleting logs.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogDeleteResult {
    pub deleted_count: usize,
    pub freed_bytes: u64,
}

/// Per-file results from SearchLogs.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogFileSearchResult {
    pub filename: String,
    pub matches: usize,
    pub entries: Vec<LogEntryInfo>,
}
```

**Validation:** `cargo check -p context-api`, serde round-trip tests for each type.

---

### Step 4: Per-Command Tracing Capture Module

**File:** `crates/context-api/src/tracing_capture.rs` (new)

This module uses the `build_capture_dispatch()` API from Step 1 to wrap `execute()` calls:

```rust
use std::path::PathBuf;
use std::time::Instant;
use context_trace::logging::build_capture_dispatch;
use crate::types::TraceSummary;

/// Configuration for per-command tracing capture.
pub struct CaptureConfig {
    /// Whether capture is enabled.
    pub enabled: bool,
    /// The log directory to write to.
    pub log_dir: PathBuf,
    /// Minimum tracing level to capture (default: "TRACE").
    pub level: String,
}

/// Result of a traced execution.
pub struct CaptureResult<T> {
    /// The inner result from the closure.
    pub result: T,
    /// Path to the log file (if capture was enabled).
    pub log_file: Option<PathBuf>,
    /// Summary of what was captured (if capture was enabled).
    pub summary: Option<TraceSummary>,
}

/// Generate a log filename from a command name.
fn log_filename(command_name: &str) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    // Format as YYYYMMDDTHHMMSS<millis> using manual arithmetic (avoids chrono dependency)
    // For simplicity, use epoch-based compact timestamp
    format!("{}{}_{}.log", secs, millis, command_name)
}

/// Wrap a closure with per-command tracing capture.
///
/// Creates a scoped tracing dispatcher that writes JSON events to a
/// log file in `config.log_dir` for the duration of the closure.
/// Uses the same format as TestTracing, compatible with LogParser.
pub fn capture_traced<F, T>(
    config: &CaptureConfig,
    command_name: &str,
    f: F,
) -> CaptureResult<T>
where
    F: FnOnce() -> T,
{
    if !config.enabled {
        return CaptureResult {
            result: f(),
            log_file: None,
            summary: None,
        };
    }

    let filename = log_filename(command_name);
    let log_path = config.log_dir.join(&filename);

    let capture = match build_capture_dispatch(&log_path, &config.level) {
        Ok(c) => c,
        Err(_) => {
            // If we can't create the capture, run without it
            return CaptureResult {
                result: f(),
                log_file: None,
                summary: None,
            };
        },
    };

    let start = Instant::now();
    let result = tracing::dispatcher::with_default(&capture.dispatch, f);
    let duration = start.elapsed();

    let entry_count = capture.event_count
        .load(std::sync::atomic::Ordering::Relaxed);

    let summary = TraceSummary {
        log_file: filename,
        entry_count,
        event_summary: HashMap::new(), // TODO: parse from captured events
        duration_ms: duration.as_millis() as u64,
    };

    CaptureResult {
        result,
        log_file: Some(log_path),
        summary: Some(summary),
    }
}
```

**Validation:** Unit test that calls `capture_traced` with a closure that emits `tracing::info!` events, verifies the log file is created and parseable.

---

### Step 5: Log Directory Management in Workspace

**Files:** `crates/context-api/src/workspace/`

Add log directory creation/management to the workspace system:

```rust
impl WorkspaceManager {
    /// Get the log directory path for a workspace.
    /// Creates the directory if it doesn't exist.
    pub fn log_dir(&self, workspace_name: &str) -> Result<PathBuf, ApiError> {
        let ws_dir = self.workspace_dir(workspace_name)?;
        let dir = ws_dir.join("logs");
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
```

**Validation:** `cargo check -p context-api`

---

### Step 6: Log Commands in the Command Enum

**Files:** `crates/context-api/src/commands/mod.rs`, `crates/context-api/src/commands/logs.rs` (new)

Add log command variants to `Command` and `CommandResult`:

```rust
// In Command enum — new section:
// -- Logs -------------------------------------------------------------------
ListLogs {
    workspace: String,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default = "default_log_limit")]
    limit: usize,
},
GetLog {
    workspace: String,
    filename: String,
    #[serde(default)]
    filter: Option<String>,
    #[serde(default = "default_log_limit")]
    limit: usize,
    #[serde(default)]
    offset: usize,
},
QueryLog {
    workspace: String,
    filename: String,
    query: String,
    #[serde(default = "default_query_limit")]
    limit: usize,
},
AnalyzeLog {
    workspace: String,
    filename: String,
},
SearchLogs {
    workspace: String,
    query: String,
    #[serde(default = "default_search_limit_per_file")]
    limit_per_file: usize,
},
DeleteLog {
    workspace: String,
    filename: String,
},
DeleteLogs {
    workspace: String,
    #[serde(default)]
    older_than_days: Option<u32>,
},
```

```rust
// In CommandResult enum — new section:
LogList {
    logs: Vec<LogFileInfo>,
},
LogEntries {
    filename: String,
    total: usize,
    offset: usize,
    limit: usize,
    returned: usize,
    entries: Vec<LogEntryInfo>,
},
LogQueryResult {
    query: String,
    matches: usize,
    entries: Vec<LogEntryInfo>,
},
LogAnalysis(LogAnalysis),
LogSearchResult {
    query: String,
    files_with_matches: usize,
    results: Vec<LogFileSearchResult>,
},
LogDeleteResult(LogDeleteResult),
```

Add dispatch arms in `execute()`:

```rust
Command::ListLogs { workspace, pattern, limit } => {
    let log_dir = manager.log_dir(&workspace)?;
    let logs = logs::list_logs(&log_dir, pattern.as_deref(), limit)?;
    Ok(CommandResult::LogList { logs })
},
// ... etc for all log commands
```

The `logs.rs` module implements the actual logic using `LogParser` and `JqFilter` from the same crate:

```rust
// crates/context-api/src/commands/logs.rs

use crate::log_parser::LogParser;
use crate::jq::JqFilter;
use crate::types::*;

pub fn list_logs(log_dir: &Path, pattern: Option<&str>, limit: usize)
    -> Result<Vec<LogFileInfo>, ApiError> { ... }

pub fn get_log(log_dir: &Path, filename: &str, filter: Option<&str>, limit: usize, offset: usize)
    -> Result<(Vec<LogEntryInfo>, usize), ApiError> { ... }

pub fn query_log(log_dir: &Path, filename: &str, query: &str, limit: usize)
    -> Result<(Vec<LogEntryInfo>, usize), ApiError> { ... }

pub fn analyze_log(log_dir: &Path, filename: &str)
    -> Result<LogAnalysis, ApiError> { ... }

pub fn search_logs(log_dir: &Path, query: &str, limit_per_file: usize)
    -> Result<(Vec<LogFileSearchResult>, usize), ApiError> { ... }

pub fn delete_log(log_dir: &Path, filename: &str) -> Result<(), ApiError> { ... }

pub fn delete_logs(log_dir: &Path, older_than_days: Option<u32>)
    -> Result<LogDeleteResult, ApiError> { ... }
```

Add `execute_traced()` and `Command::command_name()` to the module:

```rust
/// Execute a command with optional tracing capture.
pub fn execute_traced(
    manager: &mut WorkspaceManager,
    cmd: Command,
    capture: Option<&CaptureConfig>,
) -> Result<(CommandResult, Option<TraceSummary>), ApiError> { ... }

impl Command {
    /// Returns the snake_case name of this command variant.
    pub fn command_name(&self) -> &'static str {
        match self {
            Command::CreateWorkspace { .. } => "create_workspace",
            Command::InsertSequence { .. } => "insert_sequence",
            Command::SearchSequence { .. } => "search_sequence",
            // ... all variants
        }
    }
}
```

**Validation:** `cargo check --workspace`, full unit test suite for all log commands.

---

### Step 7: Update `viewer-api` to Depend on `context-api`

**Files:**
- `tools/viewer-api/Cargo.toml` — add `context-api = { path = "../../crates/context-api" }`
- `tools/viewer-api/src/lib.rs` — re-export log types and parser

```rust
// tools/viewer-api/src/lib.rs — add:
pub use context_api::log_parser;
pub use context_api::jq;
pub use context_api::types::{
    LogFileInfo, LogEntryInfo, LogAnalysis, SpanSummary,
    TraceSummary, LogDeleteResult, LogFileSearchResult,
};
```

This means `viewer-api` now exposes both:
- Its own `query::JqFilter` (existing, unchanged)
- The canonical `LogParser` and log types from `context-api`

Both `JqFilter` implementations (in `viewer-api/src/query.rs` and `context-api/src/jq.rs`) wrap the same `jaq-*` crates. Long-term, `viewer-api::query` could be deprecated in favor of `context-api::jq`, but for this phase we keep both to minimize churn.

**Validation:** `cargo check -p viewer-api`

---

### Step 8: Migrate `log-viewer` to Use Shared Types

**Files:**
- `tools/log-viewer/src/log_parser.rs` — gut the file, replace with re-export: `pub use viewer_api::log_parser::*;`
- `tools/log-viewer/src/mcp_server.rs` — update imports
- `tools/log-viewer/src/handlers.rs` — update imports

The `LogEntry` type in `log-viewer` is richer than the API's `LogEntryInfo` (it has `panic_file`, `assertion_diff`, `backtrace`, `raw` fields). Two approaches:

**Option A:** Make `context-api`'s `LogEntry` the full superset (include all fields, with `Option` for the niche ones). Recommended — it means one type everywhere.

**Option B:** Keep `log-viewer`'s extended `LogEntry` as a wrapper/extension of `LogEntryInfo`. Use `From<LogEntry> for LogEntryInfo` for the API boundary.

**Recommended: Option A** — merge the full `LogEntry` into `context-api::log_parser`, including optional panic/backtrace fields. The `LogEntryInfo` API type is then a simplified view with `From<LogEntry>`.

**Validation:** `cargo test -p log-viewer` — all existing tests pass with the new import paths.

---

### Step 9: CLI Integration — Subcommand Mode

**Files:** `tools/context-cli/src/main.rs`, `tools/context-cli/Cargo.toml`

Add `--trace` flag and log subcommands:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,

    /// Enable per-command tracing capture (writes .log files to workspace)
    #[arg(long, global = true)]
    trace: bool,
}

// Add to CliCommand enum:
/// List trace log files for a workspace
ListLogs { workspace: String, #[arg(long)] pattern: Option<String> },
/// Read a trace log file
GetLog { workspace: String, filename: String, #[arg(long)] filter: Option<String>, #[arg(long, default_value = "100")] limit: usize },
/// Query a trace log with a JQ expression
QueryLog { workspace: String, filename: String, query: String },
/// Analyze a trace log file
AnalyzeLog { workspace: String, filename: String },
/// Search across all trace logs
SearchLogs { workspace: String, query: String },
/// Delete a trace log file
DeleteLog { workspace: String, filename: String },
```

When `--trace` is set, wrap the `execute()` call:

```rust
if cli.trace {
    let log_dir = manager.log_dir(&workspace_name)?;
    let config = CaptureConfig { enabled: true, log_dir, level: "TRACE".into() };
    let (result, trace) = execute_traced(&mut manager, cmd, Some(&config))?;
    output::print_command_result(&result);
    if let Some(summary) = trace {
        eprintln!("📝 Trace: {} ({} events, {}ms)",
            summary.log_file, summary.entry_count, summary.duration_ms);
    }
} else {
    let result = execute(&mut manager, cmd)?;
    output::print_command_result(&result);
}
```

**Validation:** Manual test: `context-cli --trace insert-sequence demo "hello"` → verify `.log` file created.

---

### Step 10: CLI Integration — REPL Mode

**File:** `tools/context-cli/src/repl.rs`

Add REPL commands for tracing and log access:

```text
REPL Commands (new):

  trace on|off         Toggle per-command tracing for subsequent commands
  trace status         Show current tracing state

  logs                 List log files for current workspace
  logs <pattern>       List log files matching pattern
  log <filename>       Read a log file (paginated)
  log <filename> <jq>  Read a log file with JQ filter
  query <filename> <jq>  Query log file with JQ expression
  analyze <filename>   Analyze a log file (statistics)
  search-logs <jq>     Search across all logs
  delete-log <file>    Delete a specific log file
  clean-logs           Delete all logs
  clean-logs <days>    Delete logs older than N days
```

Add `tracing_enabled: bool` field to the REPL state. When true, wrap graph-mutating and search/read commands with `execute_traced()`.

Update `print_help()` to include the new commands.

**Validation:** Manual test in REPL: `trace on` → `insert hello` → `logs` → `analyze <latest>` → `query <latest> select(.level == "INFO")`.

---

### Step 11: MCP Integration — Trace Flag in Execute

**File:** `tools/context-mcp/src/server.rs`

Add optional tracing to `ExecuteInput` and `ExecuteOutput`:

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteInput {
    #[serde(flatten)]
    pub command: Command,

    /// Enable tracing for this command execution.
    /// When true, engine internal events (search transitions,
    /// insert split/join phases, etc.) are captured to a log file.
    /// The response will include a `trace` field with the log filename.
    #[serde(default)]
    pub trace: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteOutput {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CommandResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Trace summary (present when trace: true was requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<TraceSummary>,
}
```

In the `execute_command` handler, check `input.trace` and call `execute_traced()` or `execute()` accordingly. Extract the workspace name from the command to determine the log directory.

**Validation:** MCP test: execute with `trace: true` → verify response includes `trace` field.

---

### Step 12: MCP Integration — Log Commands in Help & Workflow

**File:** `tools/context-mcp/src/server.rs`

Add to `all_commands()` help registry — 7 new entries for the log commands, under a new "logs" category:

```rust
CategoryInfo {
    name: "logs",
    display: "Trace Logs",
    description: "Access and query per-command tracing logs. Enable tracing via trace:true in execute.",
},
```

Add help entries for: `list_logs`, `get_log`, `query_log`, `analyze_log`, `search_logs`, `delete_log`, `delete_logs`.

Add a new `trace_and_debug` workflow template:

```rust
"trace_and_debug" => Some(WorkflowTemplate {
    name: "trace_and_debug",
    title: "Trace & Debug Workflow",
    description: "Execute a command with tracing enabled, then analyze the trace log.",
    steps: vec![
        // 1. Execute with tracing
        // 2. List logs to find the file
        // 3. Analyze the log for overview
        // 4. Query for graph_op events
        // 5. Query for errors/warnings
    ],
    tips: vec![
        "The trace flag only affects the command it's attached to.",
        "Log files persist across sessions in the workspace directory.",
        "JQ queries use the jaq engine — most standard jq syntax works.",
        "Use analyze_log first for a quick overview before deep-querying.",
        "graph_op fields contain rich structured data about algorithm steps.",
    ],
}),
```

Update `test_help_all_commands_covered` to include the new command names.

**Validation:** `cargo test -p context-mcp` — all tests pass.

---

### Step 13: Output Formatting in CLI

**File:** `tools/context-cli/src/output.rs`

Add formatters for log command results:

```rust
fn print_log_list(logs: &[LogFileInfo]) { ... }
fn print_log_entries(entries: &[LogEntryInfo], total: usize, offset: usize) { ... }
fn print_log_analysis(analysis: &LogAnalysis) { ... }
fn print_log_query_result(query: &str, matches: usize, entries: &[LogEntryInfo]) { ... }
fn print_log_search_result(query: &str, results: &[LogFileSearchResult]) { ... }
fn print_log_delete_result(result: &LogDeleteResult) { ... }
```

Match on the new `CommandResult` variants in `print_command_result()`.

**Validation:** Manual check of output formatting.

---

### Step 14: Integration Tests

**Files:** `crates/context-api/src/commands/logs.rs`, `tools/context-mcp/src/server.rs`

Key test scenarios:

**In `context-api`:**
- `test_list_logs_empty` — new workspace has no logs
- `test_traced_insert_creates_log` — `execute_traced` with capture creates a parseable log file
- `test_list_logs_after_trace` — log appears in `ListLogs` result
- `test_query_log_with_jq` — JQ filter works on captured log
- `test_analyze_log` — statistics are correct
- `test_get_log_with_pagination` — offset/limit work
- `test_delete_log` — file is removed
- `test_delete_logs_older_than` — age-based cleanup
- `test_search_logs_across_files` — multi-file JQ search

**In `context-mcp`:**
- `test_execute_with_trace` — `trace: true` produces trace summary in response
- `test_execute_without_trace` — no trace field when `trace: false`
- `test_log_commands_in_help` — `list_logs`, `query_log` etc. appear in help
- `test_trace_and_debug_workflow` — workflow template exists and has valid commands

**Validation:** `cargo test -p context-api`, `cargo test -p context-mcp`, `cargo test -p log-viewer`.

---

### Step 15: Documentation

**Files:**
- `tools/context-mcp/README.md` — add log tools section, trace flag documentation
- `tools/context-cli/README.md` — add `--trace` flag, REPL log commands
- `crates/context-api/README.md` — add log parser and JQ module documentation

---

### Step 16: Final Verification

- [ ] `cargo check --workspace` — no errors
- [ ] `cargo test -p context-api` — all tests pass including new log tests
- [ ] `cargo test -p context-mcp` — all tests pass including trace and help tests
- [ ] `cargo test -p log-viewer` — still passes after migration to shared types
- [ ] `cargo test -p viewer-api` — still passes with new `context-api` dependency
- [ ] `cargo build -p context-cli` — builds with new log subcommands
- [ ] `cargo build -p context-mcp` — builds with trace support
- [ ] Manual CLI test: `context-cli --trace insert-sequence demo "hello"` → log file created
- [ ] Manual REPL test: `trace on` → `insert hello` → `logs` → `analyze <file>` → `query <file> select(.level == "INFO")`
- [ ] Manual MCP test: execute with `trace: true` → trace summary in response → `list_logs` shows file → `query_log` with JQ works

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `PrettyJsonWriter` visibility change breaks internal assumptions | Low | Low | It's a simple wrapper struct. Making it `pub` doesn't change behavior, only visibility. Existing `TestTracing` code continues to work identically. |
| `tracing::dispatcher::with_default` suppresses CLI stderr output during capture | Medium | Low | This is intentional — captured events go to the log file. The CLI's normal stderr output resumes after the closure. Document this behavior. |
| JQ compilation is slow for complex queries | Low | Low | Each query is compiled once. For repeated queries, callers could cache the `JqFilter`. |
| Large log files from complex operations | Medium | Medium | Pagination via `offset`/`limit`. `analyze_log` streams entries without loading all into memory. Add `max_log_entries` config. |
| `jaq-*` crates duplicated between `context-api` and `viewer-api` | Medium | Low | Cargo deduplicates same-version dependencies. Both use `jaq-* = "1"`. Long-term, `viewer-api::query` could re-export from `context-api::jq`. |
| `viewer-api → context-api` dependency adds domain coupling to a generic crate | Low | Medium | `viewer-api` was already implicitly domain-specific (it's called "viewer-api" and lives in a domain-specific workspace). The coupling is minor — just re-exporting log types. |
| Thread-local insert visualization state | Low | Low | Orthogonal to tracing dispatcher. Thread-locals store visualization state (step count, path ID), not subscriber state. No interaction. |
| Timestamp collisions in log filenames | Low | Low | Millisecond precision in filename. For sub-millisecond collisions (extremely unlikely), add a fallback with incrementing suffix. |
| `LogEntry` migration breaks `log-viewer` | Medium | Medium | Use `From` conversions between the full `LogEntry` and the simplified `LogEntryInfo`. Migrate in two steps: first add the types to `context-api`, then switch `log-viewer` imports. |

---

## Notes

### Relationship to `log-viewer`

This phase doesn't replace `log-viewer`. The log-viewer remains the primary tool for querying *test* logs in `target/test-logs/` and for the HTTP/frontend-based log viewing experience. Phase 3.1 adds *workspace-scoped* logs that are part of the graph workspace lifecycle.

After this phase, `log-viewer` and `context-api` share the same `LogParser` and `LogEntry` types. Long-term, `log-viewer` could gain a "connect to workspace" mode that points at workspace logs instead of test logs. The HTTP handlers in `log-viewer` would continue to provide the richer debugging UI (source file lookups, assertion diffs, backtraces, session management).

### Relationship to Phase 4 (HTTP Adapter)

The log commands added here become automatically available through Phase 4's `POST /api/execute`. The Phase 4 plan already defines `context-http` depending on both `context-api` and `viewer-api`. REST convenience endpoints can be added in Phase 4:

```text
GET  /api/workspaces/:name/logs                    → ListLogs
GET  /api/workspaces/:name/logs/:filename          → GetLog
POST /api/workspaces/:name/logs/:filename/query    → QueryLog
GET  /api/workspaces/:name/logs/:filename/analyze  → AnalyzeLog
```

Phase 4's `POST /api/execute` also inherits the `trace` flag — if the HTTP adapter supports it in the request body, the response can include `TraceSummary`.

### Relationship to Phase 5 (TypeScript Types)

All new types in `context-api/src/types.rs` should be annotated with:

```rust
#[cfg_attr(feature = "ts-gen", derive(TS))]
#[cfg_attr(feature = "ts-gen", ts(
    export,
    export_to = "../../../../packages/context-types/src/generated/"
))]
```

This follows Phase 5's hybrid approach. The affected types:
- `LogFileInfo`
- `LogEntryInfo`
- `LogAnalysis`
- `SpanSummary`
- `TraceSummary`
- `LogDeleteResult`
- `LogFileSearchResult`

These will be exported to `@context-engine/types` and consumed by any future frontend that wants to display workspace logs (separate from the `log-viewer` frontend which has its own types today).

### Why `context-api` Owns the Log Parser (Not `viewer-api`)

The original draft proposed `context-api → viewer-api` for the JQ and log parser dependency. This was wrong for several reasons:

1. **Domain ownership:** Log parsing is a domain concern of the context-engine API. The workspace's log files are part of the workspace lifecycle. The API that creates, queries, and deletes them should own the parser.

2. **Dependency direction:** `crates/` should not depend on `tools/`. `context-api` is a library crate in `crates/`; `viewer-api` is a tool support crate in `tools/`. Having the library depend on a tool's support crate inverts the natural layering.

3. **`viewer-api` is infrastructure, not domain:** `viewer-api` provides HTTP server boilerplate, CORS, dev proxy, session management, and source file resolution. None of these are needed by `context-api`. Adding the dependency would pull in `axum`, `hyper`, `tower-http`, etc. into the core API crate.

4. **Phase 4 alignment:** The Phase 4 HTTP adapter already depends on both `context-api` and `viewer-api`. With log types in `context-api`, the HTTP adapter gets them directly. `viewer-api` re-exports them as a convenience for `log-viewer`.

### `jaq-*` Crates in `context-api`

Adding `jaq-core`, `jaq-std`, `jaq-interpret`, `jaq-syn` as direct dependencies of `context-api` increases its dependency footprint. These crates are pure Rust, well-maintained, and compile reasonably fast. The total addition is ~4 crates with no native dependencies.

If the dependency footprint becomes a concern, the JQ support can be feature-gated:

```toml
[features]
default = ["jq"]
jq = ["jaq-core", "jaq-std", "jaq-interpret", "jaq-syn"]
```

With `jq` disabled, the `QueryLog` and `SearchLogs` commands would return an error ("JQ support not compiled in"). The `ListLogs`, `GetLog`, `AnalyzeLog`, `DeleteLog`, `DeleteLogs` commands would still work (they don't need JQ).

### Future Enhancements

- **Live tracing** — WebSocket/SSE streaming of events during execution (aligns with Phase 4's HTTP adapter)
- **Log rotation** — Automatic cleanup of old logs (keep last N or last N days)
- **Log compression** — gzip old log files to save space
- **Trace correlation** — Link log entries to specific graph vertices for visual debugging
- **Diff mode** — Compare traces from two runs of the same command
- **Selective capture** — Filter by module/level at capture time (e.g., only `context_search` events)
- **Nested dispatcher layering** — Instead of replacing the thread-local dispatcher, layer the capture dispatcher on top (requires `tracing-subscriber` `reload` layer or a custom dispatcher that fans out to multiple subscribers). This would allow both stderr and file capture simultaneously.

### Deviations from Plan

*(To be filled during implementation)*

### Lessons Learned

*(To be filled during implementation)*

---

## Appendix A: Example Log Output

A traced `insert_sequence` call produces a log file like this (excerpt from actual test logs in `target/test-logs/`):

```json
{
  "timestamp": "2026-03-14T00:55:30.028572Z",
  "level": "INFO",
  "fields": {
    "message": "Split phase starting",
    "graph_op": "{\"step\":0,\"op_type\":\"Insert\",\"transition\":{\"SplitStart\":{\"node\":{\"index\":5}}},\"description\":\"Split phase starting\",\"path_id\":\"insert/context-insert/1710374130028303400\",\"path_graph\":{...},\"graph_mutation\":null}",
    "step": 0,
    "op_type": "Insert"
  },
  "target": "context_insert::visualization",
  "span": "insert",
  "spans": ["insert"],
  "filename": "crates/context-insert/src/visualization.rs",
  "line_number": 103
}
```

An agent can then query this with JQ:

```text
select(.fields.graph_op != null)                    → all graph operations
select(.fields.op_type == "Insert")                 → insert operations only
select(.fields.step >= 3 and .fields.step <= 5)     → specific step range
select(.level == "ERROR" or .level == "WARN")       → errors and warnings
```

---

## Appendix B: Dependency Graph After Phase 3.1

```text
crates/context-api
  ├── context-trace       (graph types, scoped capture via build_capture_dispatch)
  ├── context-search      (search commands)
  ├── context-insert      (insert commands)
  ├── context-read        (read commands)
  ├── jaq-core/std/interpret/syn  (JQ engine, direct dep)
  └── serde_json          (streaming log parser)

tools/viewer-api
  ├── context-api  ←NEW   (re-exports LogParser, log types)
  ├── jaq-*                (JqFilter — existing, may deprecate in favor of context-api::jq)
  ├── axum, tokio, rmcp
  └── ...

tools/log-viewer
  └── viewer-api           (gets LogParser + JqFilter + log types transitively)

tools/context-cli
  └── context-api          (gets everything: commands, capture, log parser)

tools/context-mcp
  ├── context-api
  └── rmcp

tools/context-http (Phase 4, future)
  ├── context-api
  └── viewer-api           (HTTP infrastructure + log types)
```

No circular dependencies. `crates/` never depends on `tools/`. Domain types flow downward from `context-api`.

---

## Appendix C: Step Execution Order Summary

| Step | What | Crate(s) | Depends On |
|------|------|----------|------------|
| 1 | Expose tracing capture infra from `context-trace` | `context-trace` | — |
| 2 | Log parser + JQ wrapper in `context-api` | `context-api` | — |
| 3 | Log-related API types | `context-api` | — |
| 4 | Per-command tracing capture module | `context-api` | Step 1 |
| 5 | Log directory management in workspace | `context-api` | — |
| 6 | Log commands in Command enum + dispatch | `context-api` | Steps 2, 3, 4, 5 |
| 7 | Update `viewer-api` to depend on `context-api` | `viewer-api` | Steps 2, 3 |
| 8 | Migrate `log-viewer` to shared types | `log-viewer` | Step 7 |
| 9 | CLI subcommand mode integration | `context-cli` | Step 6 |
| 10 | CLI REPL mode integration | `context-cli` | Step 6 |
| 11 | MCP trace flag in execute | `context-mcp` | Steps 4, 6 |
| 12 | MCP help & workflow for log commands | `context-mcp` | Step 6 |
| 13 | CLI output formatting | `context-cli` | Step 6 |
| 14 | Integration tests | All | Steps 1–13 |
| 15 | Documentation | All | Steps 1–13 |
| 16 | Final verification | — | All |

Steps 1, 2, 3, 5 can be done in parallel. Step 6 is the integration point. Steps 7–8 and 9–13 can proceed in parallel after Step 6.