/// Data types that mirror the TypeScript types generated from Rust via ts-rs,
/// plus the frontend-only types from the log-viewer.
use serde::{Deserialize, Serialize};

// ── Log files ────────────────────────────────────────────────────────────────

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub timestamp: Option<String>,
    pub target: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub event_type: Option<String>,
    pub depth: Option<u32>,
    #[serde(default)]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContentResponse {
    pub entries: Vec<LogEntry>,
}

// ── Hypergraph snapshot ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotNode {
    pub index: u32,
    pub label: String,
    pub width: u32,
    pub is_atom: bool,
    #[serde(default)]
    pub child_indices: Vec<u32>,
    #[serde(default)]
    pub parent_indices: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotEdge {
    pub from: u32,
    pub to: u32,
    pub pattern_idx: u32,
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
