/// Data types that mirror the TypeScript types generated from Rust via ts-rs,
/// plus the frontend-only types from the log-viewer.
///
/// These must stay in sync with `context-api`'s serialized output. The
/// canonical source of truth is the generated TypeScript in
/// `tools/viewer/log-viewer/frontend/src/types/generated/`.
use serde::{Deserialize, Serialize};

// ── Log files ────────────────────────────────────────────────────────────────

/// Mirrors `context_api::types::LogFileInfo` serialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogFileInfo {
    pub name: String,
    pub size: u64,
    #[serde(default)]
    pub has_graph_snapshot: bool,
    #[serde(default)]
    pub has_search_ops: bool,
    #[serde(default)]
    pub has_insert_ops: bool,
    #[serde(default)]
    pub has_search_paths: bool,
}

// Frontend alias used throughout the codebase
pub type LogFile = LogFileInfo;

// ── Log entries ───────────────────────────────────────────────────────────────

/// Mirrors `context_api::types::AssertionDiff` serialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssertionDiff {
    pub left: String,
    pub right: String,
}

/// Mirrors `context_api::log_parser::LogEntry` serialization (the *full* parser
/// type, not the simplified `LogEntryInfo`).  Field names match the generated
/// TypeScript type in `types/generated/LogEntry.ts`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    /// Entry index (1-based).
    pub line_number: usize,
    pub level: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_name: Option<String>,
    pub depth: usize,
    /// Additional structured fields from the tracing event.
    #[serde(default)]
    pub fields: serde_json::Value,
    /// Source file from tracing macro (field name is `file` in the JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic_line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_diff: Option<AssertionDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<String>,
    /// Raw log line summary.
    pub raw: String,
}

/// Mirrors `context_api::types::LogContentResponse` serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContentResponse {
    pub name: String,
    pub entries: Vec<LogEntry>,
    pub total_lines: usize,
}

// ── Hypergraph snapshot ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotNode {
    pub index: u32,
    pub label: String,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotEdge {
    pub from: u32,
    pub to: u32,
    pub pattern_idx: u32,
    pub sub_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphSnapshot {
    pub nodes: Vec<SnapshotNode>,
    pub edges: Vec<SnapshotEdge>,
}

pub type HypergraphSnapshot = GraphSnapshot;

// ── View tabs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ViewTab {
    #[default]
    Logs,
    Hypergraph,
    Debug,
    Settings,
}

impl ViewTab {
    pub fn label(&self) -> &'static str {
        match self {
            ViewTab::Logs => "Logs",
            ViewTab::Hypergraph => "Hypergraph",
            ViewTab::Debug => "Debug",
            ViewTab::Settings => "Settings",
        }
    }
}
