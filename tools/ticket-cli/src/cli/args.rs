use std::path::PathBuf;

use clap::{Args, Subcommand, ValueEnum};
use uuid::Uuid;

// ── arg structs ────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub struct CreateArgs {
    #[arg(long)]
    pub id: Option<Uuid>,
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long = "field")]
    pub fields: Vec<String>,
    /// Copy the contents of this file into the ticket as description.md.
    #[arg(long = "body-file")]
    pub body_file: Option<PathBuf>,
    /// Place the ticket in this scan root (defaults to first registered root).
    #[arg(long = "root")]
    pub target_root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct IdArgs {
    #[arg(long)]
    pub id: Uuid,
}

#[derive(Debug, Args)]
pub struct StatusArgs {
    /// Optional prefix filter — only include tickets whose title starts with this string.
    /// E.g. "[bootstrap]" to scope the view to the bootstrap track.
    #[arg(long)]
    pub filter: Option<String>,
    /// Include blocked tickets in the output (default: omitted for brevity).
    #[arg(long, default_value_t = false)]
    pub show_blocked: bool,
}

#[derive(Debug, Args)]
pub struct ReadyOverviewArgs {
    /// Optional prefix filter — only include tickets whose title starts with this string.
    #[arg(long)]
    pub filter: Option<String>,
    /// Optional scope label included in the JSON response.
    #[arg(long)]
    pub scope: Option<String>,
}

#[derive(Debug, Args)]
pub struct WatchArgs {
    /// Debounce time in milliseconds before triggering reconcile after an event.
    #[arg(long, default_value = "200")]
    pub debounce_ms: u64,
}

#[derive(Debug, Args)]
pub struct ServeCliArgs {
    /// TCP port to bind to.
    #[arg(long, default_value = "8080")]
    pub port: u16,
    /// Host address to bind to.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    /// Serve a specific named workspace only (default: all registered).
    #[arg(long)]
    pub workspace: Option<String>,
}

#[derive(Debug, Args)]
pub struct CloseArgs {
    #[arg(long)]
    pub id: Uuid,
    /// Target state to fast-forward to (default: done).
    #[arg(long = "to-state", default_value = "done")]
    pub to_state: String,
}

#[derive(Debug, Args)]
pub struct AttachArgs {
    #[arg(long)]
    pub id: Uuid,
    /// Path to the file to attach.
    pub path: PathBuf,
    /// Optional name for the asset (defaults to source filename).
    #[arg(long = "as")]
    pub asset_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct LinkArgs {
    /// UUID of the source ticket.
    #[arg(long)]
    pub from: Uuid,
    /// UUID of the target ticket.
    #[arg(long)]
    pub to: Uuid,
    /// Edge kind (e.g. depends_on, blocks, linked).
    #[arg(long)]
    pub kind: String,
    /// Human-readable reason for this edge (optional, stored in response only).
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct UnlinkArgs {
    /// UUID of the source ticket.
    #[arg(long)]
    pub from: Uuid,
    /// UUID of the target ticket.
    #[arg(long)]
    pub to: Uuid,
    /// Edge kind (e.g. depends_on, blocks, linked).
    #[arg(long)]
    pub kind: String,
    /// Human-readable reason for this removal (optional, stored in response only).
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "from-state")]
    pub from_state: Option<String>,
    #[arg(long = "to-state")]
    pub to_state: Option<String>,
    #[arg(long = "field")]
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReproOutcome {
    Reproduced,
    NotReproduced,
    Intermittent,
    Fixed,
}

impl ReproOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reproduced => "reproduced",
            Self::NotReproduced => "not_reproduced",
            Self::Intermittent => "intermittent",
            Self::Fixed => "fixed",
        }
    }
}

#[derive(Debug, Args)]
pub struct ReproArgs {
    /// Ticket UUID.
    #[arg(long)]
    pub id: Uuid,
    /// Reproduction outcome.
    #[arg(long, value_enum, default_value_t = ReproOutcome::Reproduced)]
    pub outcome: ReproOutcome,
    /// Commit SHA where reproduction was attempted (defaults to git HEAD if available).
    #[arg(long)]
    pub commit: Option<String>,
    /// Optional reproduction command used.
    #[arg(long)]
    pub command: Option<String>,
    /// Optional short note.
    #[arg(long)]
    pub note: Option<String>,
    /// Optional RFC3339 timestamp (defaults to now/UTC).
    #[arg(long)]
    pub timestamp: Option<String>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    #[arg(long)]
    pub limit: Option<usize>,
    /// Include latest reproduction metadata in each list item.
    #[arg(long, default_value_t = false)]
    pub with_repro: bool,
    /// Include soft-deleted tickets in the listing.
    #[arg(long, default_value_t = false)]
    pub include_deleted: bool,
    /// Filter by field values (key=value). Can be repeated.
    #[arg(long = "where")]
    pub where_clauses: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    #[arg(long = "reindex")]
    pub reindex: bool,
}

#[derive(Debug, Args)]
pub struct ClaimArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "agent")]
    pub agent_id: String,
    #[arg(long = "ttl-secs", default_value_t = 300)]
    pub ttl_secs: u64,
    #[arg(long = "intent")]
    pub work_intent: Option<String>,
}

#[derive(Debug, Args)]
pub struct UnclaimArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct TextArgs {
    pub expression: String,
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct AddRootArgs {
    pub path: PathBuf,
    #[arg(long, default_value = "default")]
    pub label: String,
}

#[derive(Debug, Args)]
pub struct HistoryArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long)]
    pub from: String,
    #[arg(long)]
    pub to: String,
}

#[derive(Debug, Args)]
pub struct RevertArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "to")]
    pub to_sha: String,
}

#[derive(Debug, Args)]
pub struct FinalizeMergeArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "merge-commit")]
    pub merge_commit: String,
}

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    #[command(subcommand)]
    pub command: WorkspaceSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkspaceSubCommand {
    /// List all registered workspaces.
    List,
    /// Register a new named workspace.
    New(WorkspaceNewArgs),
    /// Set the active workspace by name.
    Use(WorkspaceUseArgs),
    /// Show the currently active workspace and how it was resolved.
    Current,
    /// Unregister a workspace (data on disk is not removed).
    Remove(WorkspaceRemoveArgs),
}

#[derive(Debug, Args)]
pub struct WorkspaceNewArgs {
    /// Name for the new workspace.
    pub name: String,
    /// Index root path (defaults to ~/.ticket-<name>/).
    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct WorkspaceUseArgs {
    /// Name of the workspace to activate.
    pub name: String,
    /// Write a .ticket-workspace file in the current directory instead of
    /// updating the global active pointer.
    #[arg(long)]
    pub local: bool,
}

#[derive(Debug, Args)]
pub struct WorkspaceRemoveArgs {
    /// Name of the workspace to unregister.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ExecArgs {
    /// Execute multiple commands from stdin, one JSON object per line, as a
    /// single transaction. Rolls back all on first failure.
    #[arg(long)]
    pub batch: bool,
}

#[derive(Debug, Args)]
pub struct BatchArgs {
    /// Optional NDJSON file path (one JSON object per line). If omitted, read stdin.
    #[arg(long)]
    pub file: Option<PathBuf>,
}
