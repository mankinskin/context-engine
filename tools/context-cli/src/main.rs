//! `context-cli` — Command-line interface for context-engine hypergraph workspaces.
//!
//! Provides both a subcommand-based interface and an interactive REPL for
//! managing workspaces and manipulating hypergraphs through the `context-api`
//! crate.

mod output;
mod repl;

use std::collections::HashSet;

use clap::{
    Parser,
    Subcommand,
};
use context_api::{
    commands::{
        Command,
        execute,
        execute_traced,
        export_import::ExportFormat,
    },
    tracing_capture::CaptureConfig,
    types::TokenRef,
    workspace::manager::WorkspaceManager,
};

/// Context Engine hypergraph workspace CLI.
#[derive(Parser)]
#[command(
    name = "context-cli",
    about = "CLI for context-engine hypergraph workspaces",
    version
)]
struct Cli {
    /// Enable per-command tracing capture (writes .log files to workspace)
    #[arg(long, global = true)]
    trace: bool,

    /// Subcommand to execute. If omitted, starts the interactive REPL.
    #[command(subcommand)]
    command: Option<CliCommand>,
}

/// Available CLI subcommands.
#[derive(Subcommand)]
enum CliCommand {
    /// Create a new workspace.
    Create {
        /// Name of the workspace to create.
        name: String,
    },

    /// Open an existing workspace.
    Open {
        /// Name of the workspace to open.
        name: String,
    },

    /// Close an open workspace (does NOT auto-save).
    Close {
        /// Name of the workspace to close.
        name: String,
    },

    /// Save an open workspace to disk.
    Save {
        /// Name of the workspace to save.
        name: String,
    },

    /// List all workspaces (both open and on-disk).
    List,

    /// Delete a workspace from disk.
    Delete {
        /// Name of the workspace to delete.
        name: String,
    },

    /// Add a single-character atom to a workspace.
    AddAtom {
        /// Name of the open workspace.
        workspace: String,
        /// Single character to add as an atom.
        ch: char,
    },

    /// Add multiple atoms (one per character in the string).
    AddAtoms {
        /// Name of the open workspace.
        workspace: String,
        /// String of characters — each character becomes an atom (e.g. "abcde").
        chars: String,
    },

    /// Add a simple pattern from atom characters.
    AddPattern {
        /// Name of the open workspace.
        workspace: String,
        /// String of characters — each character must be an existing atom
        /// (e.g. "abc" creates a pattern from atoms 'a', 'b', 'c').
        atoms: String,
    },

    /// Get detailed information about a vertex by index.
    GetVertex {
        /// Name of the open workspace.
        workspace: String,
        /// Vertex index (numeric).
        index: usize,
    },

    /// List all vertices in a workspace.
    ListVertices {
        /// Name of the open workspace.
        workspace: String,
    },

    /// List all atoms in a workspace.
    ListAtoms {
        /// Name of the open workspace.
        workspace: String,
    },

    /// Search for a text sequence in the graph.
    SearchSequence {
        /// Name of the open workspace.
        workspace: String,
        /// The text to search for (must be at least 2 characters).
        text: String,
    },

    /// Search for a pattern by token references (indices or labels).
    SearchPattern {
        /// Name of the open workspace.
        workspace: String,
        /// Token references: numbers are indices, strings are labels.
        /// Must provide at least 2.
        query: Vec<String>,
    },

    /// Insert a text sequence into the graph (auto-creates atoms).
    InsertSequence {
        /// Name of the open workspace.
        workspace: String,
        /// The text to insert (must be at least 2 characters).
        text: String,
    },

    /// Insert a pattern by token references (indices or labels).
    InsertFirstMatch {
        /// Name of the open workspace.
        workspace: String,
        /// Token references: numbers are indices, strings are labels.
        /// Must provide at least 2.
        query: Vec<String>,
    },

    /// Insert multiple text sequences (bulk insert).
    InsertSequences {
        /// Name of the open workspace.
        workspace: String,
        /// Text sequences to insert (each must be at least 2 characters).
        texts: Vec<String>,
    },

    /// Read a vertex as a decomposition tree.
    ReadPattern {
        /// Name of the open workspace.
        workspace: String,
        /// Vertex index to read.
        index: usize,
    },

    /// Read a vertex as concatenated leaf text.
    ReadAsText {
        /// Name of the open workspace.
        workspace: String,
        /// Vertex index to read.
        index: usize,
    },

    /// Validate graph integrity.
    Validate {
        /// Name of the open workspace.
        workspace: String,
    },

    /// Print the graph snapshot as JSON.
    Snapshot {
        /// Name of the open workspace.
        workspace: String,
    },

    /// Print graph statistics.
    Stats {
        /// Name of the open workspace.
        workspace: String,
    },

    /// Show the entire graph visualization for a workspace.
    Show {
        /// Name of the open workspace.
        workspace: String,
    },

    /// Show a single vertex with its children and parents.
    ShowVertex {
        /// Name of the open workspace.
        workspace: String,
        /// Vertex index to show.
        index: usize,
    },

    /// Start the interactive REPL.
    Repl,

    /// List trace log files for a workspace.
    ListLogs {
        /// Name of the workspace.
        workspace: String,
        /// Optional filename pattern to filter.
        #[arg(long)]
        pattern: Option<String>,
    },

    /// Read a trace log file.
    GetLog {
        /// Name of the workspace.
        workspace: String,
        /// Log filename.
        filename: String,
        /// Optional level/message filter.
        #[arg(long)]
        filter: Option<String>,
        /// Max entries to return.
        #[arg(long, default_value = "100")]
        limit: usize,
    },

    /// Query a trace log with a JQ expression.
    QueryLog {
        /// Name of the workspace.
        workspace: String,
        /// Log filename.
        filename: String,
        /// JQ query expression.
        query: String,
    },

    /// Analyze a trace log file.
    AnalyzeLog {
        /// Name of the workspace.
        workspace: String,
        /// Log filename.
        filename: String,
    },

    /// Search across all trace logs.
    SearchLogs {
        /// Name of the workspace.
        workspace: String,
        /// Search query (JQ expression).
        query: String,
    },

    /// Delete a trace log file.
    DeleteLog {
        /// Name of the workspace.
        workspace: String,
        /// Log filename to delete.
        filename: String,
    },

    /// Delete trace log files, optionally only those older than N days.
    DeleteLogs {
        /// Name of the workspace.
        workspace: String,
        /// Only delete logs older than this many days (omit to delete all).
        #[arg(long)]
        older_than_days: Option<u32>,
    },

    /// Export a workspace to JSON or bincode format.
    ExportWorkspace {
        /// Name of the workspace to export.
        workspace: String,
        /// Export format: "json" or "bincode".
        #[arg(long, default_value = "json")]
        format: String,
        /// Optional output file path. If omitted, data is written to stdout.
        #[arg(long)]
        path: Option<String>,
    },

    /// Import a workspace from a previously exported file.
    ImportWorkspace {
        /// Name to assign to the imported workspace.
        name: String,
        /// Path to the exported file.
        path: String,
        /// Overwrite if a workspace with the same name already exists.
        #[arg(long)]
        overwrite: bool,
    },
}

/// Parse a string as a `TokenRef`.
///
/// If it parses as a `usize`, it becomes `TokenRef::Index`.
/// Otherwise, it becomes `TokenRef::Label`.
fn parse_token_ref(s: &str) -> TokenRef {
    match s.parse::<usize>() {
        Ok(n) => TokenRef::Index(n),
        Err(_) => TokenRef::Label(s.to_string()),
    }
}

/// Parse a slice of strings as `TokenRef` values.
fn parse_token_refs(strings: &[String]) -> Vec<TokenRef> {
    strings.iter().map(|s| parse_token_ref(s)).collect()
}

/// Extract the workspace name from a CLI command, if the command is
/// workspace-scoped.
fn workspace_name_from_cli_cmd(cmd: &CliCommand) -> Option<&str> {
    match cmd {
        CliCommand::Create { name }
        | CliCommand::Open { name }
        | CliCommand::Close { name }
        | CliCommand::Save { name }
        | CliCommand::Delete { name } => Some(name.as_str()),
        CliCommand::AddAtom { workspace, .. }
        | CliCommand::AddAtoms { workspace, .. }
        | CliCommand::AddPattern { workspace, .. }
        | CliCommand::GetVertex { workspace, .. }
        | CliCommand::ListVertices { workspace, .. }
        | CliCommand::ListAtoms { workspace, .. }
        | CliCommand::SearchSequence { workspace, .. }
        | CliCommand::SearchPattern { workspace, .. }
        | CliCommand::InsertSequence { workspace, .. }
        | CliCommand::InsertFirstMatch { workspace, .. }
        | CliCommand::InsertSequences { workspace, .. }
        | CliCommand::ReadPattern { workspace, .. }
        | CliCommand::ReadAsText { workspace, .. }
        | CliCommand::Validate { workspace, .. }
        | CliCommand::Snapshot { workspace, .. }
        | CliCommand::Stats { workspace, .. }
        | CliCommand::Show { workspace, .. }
        | CliCommand::ShowVertex { workspace, .. }
        | CliCommand::ListLogs { workspace, .. }
        | CliCommand::GetLog { workspace, .. }
        | CliCommand::QueryLog { workspace, .. }
        | CliCommand::AnalyzeLog { workspace, .. }
        | CliCommand::SearchLogs { workspace, .. }
        | CliCommand::DeleteLog { workspace, .. }
        | CliCommand::DeleteLogs { workspace, .. }
        | CliCommand::ExportWorkspace { workspace, .. } =>
            Some(workspace.as_str()),
        CliCommand::ImportWorkspace { name, .. } => Some(name.as_str()),
        CliCommand::List | CliCommand::Repl => None,
    }
}

fn main() {
    // Initialize tracing (respects RUST_LOG env var).
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let mut manager = match WorkspaceManager::current_dir() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: failed to initialize workspace manager: {e}");
            std::process::exit(1);
        },
    };

    match cli.command {
        Some(cmd) => execute_subcommand(&mut manager, cmd, cli.trace),
        None => {
            // No subcommand → start REPL
            repl::run(&mut manager);
        },
    }
}

/// Map a CLI subcommand to a `Command` enum value and execute it.
fn execute_subcommand(
    manager: &mut WorkspaceManager,
    cmd: CliCommand,
    trace: bool,
) {
    // Extract workspace name before we move `cmd` into the match.
    let ws_name_owned: Option<String> = if trace {
        workspace_name_from_cli_cmd(&cmd).map(String::from)
    } else {
        None
    };

    let api_cmd = match cmd {
        CliCommand::Create { name } => Command::CreateWorkspace { name },
        CliCommand::Open { name } => Command::OpenWorkspace { name },
        CliCommand::Close { name } => Command::CloseWorkspace { name },
        CliCommand::Save { name } => Command::SaveWorkspace { name },
        CliCommand::List => Command::ListWorkspaces,
        CliCommand::Delete { name } => Command::DeleteWorkspace { name },
        CliCommand::AddAtom { workspace, ch } =>
            Command::AddAtom { workspace, ch },
        CliCommand::AddAtoms { workspace, chars } => Command::AddAtoms {
            workspace,
            chars: chars.chars().collect(),
        },
        CliCommand::AddPattern { workspace, atoms } => {
            let atom_chars: Vec<char> = atoms.chars().collect();
            Command::AddSimplePattern {
                workspace,
                atoms: atom_chars,
            }
        },
        CliCommand::GetVertex { workspace, index } =>
            Command::GetVertex { workspace, index },
        CliCommand::ListVertices { workspace } =>
            Command::ListVertices { workspace },
        CliCommand::ListAtoms { workspace } => Command::ListAtoms { workspace },
        CliCommand::SearchSequence { workspace, text } =>
            Command::SearchSequence { workspace, text },
        CliCommand::SearchPattern { workspace, query } =>
            Command::SearchPattern {
                workspace,
                query: parse_token_refs(&query),
            },
        CliCommand::InsertSequence { workspace, text } =>
            Command::InsertSequence { workspace, text },
        CliCommand::InsertFirstMatch { workspace, query } =>
            Command::InsertFirstMatch {
                workspace,
                query: parse_token_refs(&query),
            },
        CliCommand::InsertSequences { workspace, texts } => {
            let text_set: HashSet<String> = texts.into_iter().collect();
            Command::InsertSequences {
                workspace,
                texts: text_set,
            }
        },
        CliCommand::ReadPattern { workspace, index } =>
            Command::ReadPattern { workspace, index },
        CliCommand::ReadAsText { workspace, index } =>
            Command::ReadAsText { workspace, index },
        CliCommand::Validate { workspace } =>
            Command::ValidateGraph { workspace },
        CliCommand::Snapshot { workspace } =>
            Command::GetSnapshot { workspace },
        CliCommand::Stats { workspace } => Command::GetStatistics { workspace },
        CliCommand::Show { workspace } => Command::ShowGraph { workspace },
        CliCommand::ShowVertex { workspace, index } =>
            Command::ShowVertex { workspace, index },
        CliCommand::Repl => {
            repl::run(manager);
            return;
        },
        CliCommand::ListLogs { workspace, pattern } => Command::ListLogs {
            workspace,
            pattern,
            limit: 100,
        },
        CliCommand::GetLog {
            workspace,
            filename,
            filter,
            limit,
        } => Command::GetLog {
            workspace,
            filename,
            filter,
            limit,
            offset: 0,
        },
        CliCommand::QueryLog {
            workspace,
            filename,
            query,
        } => Command::QueryLog {
            workspace,
            filename,
            query,
            limit: 100,
        },
        CliCommand::AnalyzeLog {
            workspace,
            filename,
        } => Command::AnalyzeLog {
            workspace,
            filename,
        },
        CliCommand::SearchLogs { workspace, query } => Command::SearchLogs {
            workspace,
            query,
            limit_per_file: 10,
        },
        CliCommand::DeleteLog {
            workspace,
            filename,
        } => Command::DeleteLog {
            workspace,
            filename,
        },
        CliCommand::DeleteLogs {
            workspace,
            older_than_days,
        } => Command::DeleteLogs {
            workspace,
            older_than_days,
        },
        CliCommand::ExportWorkspace {
            workspace,
            format,
            path,
        } => {
            let export_format = match format.to_lowercase().as_str() {
                "bincode" => ExportFormat::Bincode,
                _ => ExportFormat::Json,
            };
            Command::ExportWorkspace {
                workspace,
                format: export_format,
                path,
            }
        },
        CliCommand::ImportWorkspace {
            name,
            path,
            overwrite,
        } => Command::ImportWorkspace {
            name,
            path,
            overwrite,
        },
    };

    // Traced execution path
    if trace {
        if let Some(ws_name) = ws_name_owned {
            match manager.log_dir(&ws_name) {
                Ok(log_dir) => {
                    let config = CaptureConfig {
                        enabled: true,
                        log_dir,
                        level: "TRACE".to_string(),
                    };
                    match execute_traced(manager, api_cmd, Some(&config)) {
                        Ok((result, trace_summary)) => {
                            output::print_command_result(&result);
                            if let Some(summary) = trace_summary {
                                eprintln!(
                                    "\u{1f4dd} Trace: {} ({} events, {}ms)",
                                    summary.log_file,
                                    summary.entry_count,
                                    summary.duration_ms,
                                );
                            }
                        },
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        },
                    }
                    return;
                },
                Err(e) => {
                    eprintln!("Warning: could not enable tracing: {e}");
                    // Fall through to normal execution
                },
            }
        } else {
            eprintln!("Warning: --trace requires a workspace-scoped command");
        }
    }

    // Normal (non-traced) execution
    match execute(manager, api_cmd) {
        Ok(result) => output::print_command_result(&result),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        },
    }
}
