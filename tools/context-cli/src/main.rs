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
    },
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

    /// Start the interactive REPL.
    Repl,
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
        Some(cmd) => execute_subcommand(&mut manager, cmd),
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
) {
    let api_cmd = match cmd {
        CliCommand::Create { name } => Command::CreateWorkspace { name },
        CliCommand::Open { name } => Command::OpenWorkspace { name },
        CliCommand::Close { name } => Command::CloseWorkspace { name },
        CliCommand::Save { name } => Command::SaveWorkspace { name },
        CliCommand::List => Command::ListWorkspaces,
        CliCommand::Delete { name } => Command::DeleteWorkspace { name },
        CliCommand::AddAtom { workspace, ch } =>
            Command::AddAtom { workspace, ch },
        CliCommand::AddAtoms { workspace, chars } => {
            let char_set: HashSet<char> = chars.chars().collect();
            Command::AddAtoms {
                workspace,
                chars: char_set,
            }
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
        CliCommand::Repl => {
            repl::run(manager);
            return;
        },
    };

    match execute(manager, api_cmd) {
        Ok(result) => output::print_command_result(&result),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        },
    }
}
