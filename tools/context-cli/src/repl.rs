//! Interactive REPL for the context-engine CLI.
//!
//! Provides a readline-based interactive loop with:
//! - A prompt that shows the currently active workspace (if any)
//! - Tab-friendly command parsing (space-separated tokens)
//! - Help text listing all available commands
//! - History support via `rustyline`

use std::collections::HashSet;

use rustyline::{
    DefaultEditor,
    error::ReadlineError,
};

use context_api::{
    commands::{
        Command,
        execute,
    },
    types::TokenRef,
    workspace::manager::WorkspaceManager,
};

use crate::output;

/// Run the interactive REPL loop.
///
/// The REPL maintains a "current workspace" that is used as the default
/// target for graph commands (atom, pattern, vertex, etc.). Workspace
/// lifecycle commands (create, open, close, save, list, delete) can be
/// used to manage the current workspace.
pub fn run(manager: &mut WorkspaceManager) {
    println!("Context Engine REPL (type 'help' for commands, 'quit' to exit)");

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Error initializing readline: {e}");
            return;
        },
    };

    let mut current_workspace: Option<String> = None;

    loop {
        let prompt = match &current_workspace {
            Some(name) => format!("({name})> "),
            None => "> ".to_string(),
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(line);

                match line {
                    "quit" | "exit" => break,
                    "help" | "?" => print_help(),
                    _ => {
                        execute_repl_line(
                            manager,
                            &mut current_workspace,
                            line,
                        );
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            },
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            },
        }
    }

    println!("Goodbye.");
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

/// Parse and execute a single REPL line.
fn execute_repl_line(
    manager: &mut WorkspaceManager,
    current_ws: &mut Option<String>,
    line: &str,
) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        // -- Workspace lifecycle --------------------------------------------
        "create" => {
            if let Some(name) = parts.get(1) {
                match execute_and_print(
                    manager,
                    Command::CreateWorkspace {
                        name: name.to_string(),
                    },
                ) {
                    Ok(_) => {
                        // Automatically set as current workspace
                        *current_ws = Some(name.to_string());
                        println!("(workspace '{}' is now active)", name);
                    },
                    Err(()) => {}, // error already printed
                }
            } else {
                eprintln!("Usage: create <name>");
            }
        },

        "open" =>
            if let Some(name) = parts.get(1) {
                match execute_and_print(
                    manager,
                    Command::OpenWorkspace {
                        name: name.to_string(),
                    },
                ) {
                    Ok(_) => {
                        *current_ws = Some(name.to_string());
                        println!("(workspace '{}' is now active)", name);
                    },
                    Err(()) => {},
                }
            } else {
                eprintln!("Usage: open <name>");
            },

        "close" => {
            let name = parts
                .get(1)
                .map(|s| s.to_string())
                .or_else(|| current_ws.clone());

            if let Some(name) = name {
                match execute_and_print(
                    manager,
                    Command::CloseWorkspace { name: name.clone() },
                ) {
                    Ok(_) =>
                        if current_ws.as_deref() == Some(&name) {
                            *current_ws = None;
                        },
                    Err(()) => {},
                }
            } else {
                eprintln!(
                    "Usage: close [<name>]  (defaults to current workspace)"
                );
            }
        },

        "save" => {
            let name = parts
                .get(1)
                .map(|s| s.to_string())
                .or_else(|| current_ws.clone());

            if let Some(name) = name {
                execute_and_print(manager, Command::SaveWorkspace { name })
                    .ok();
            } else {
                eprintln!(
                    "Usage: save [<name>]  (defaults to current workspace)"
                );
            }
        },

        "list" => {
            execute_and_print(manager, Command::ListWorkspaces).ok();
        },

        "delete" =>
            if let Some(name) = parts.get(1) {
                match execute_and_print(
                    manager,
                    Command::DeleteWorkspace {
                        name: name.to_string(),
                    },
                ) {
                    Ok(_) =>
                        if current_ws.as_deref() == Some(*name) {
                            *current_ws = None;
                        },
                    Err(()) => {},
                }
            } else {
                eprintln!("Usage: delete <name>");
            },

        // -- Graph commands (require current workspace) ---------------------
        "atom" => {
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(chars_str) = parts.get(1) {
                    if chars_str.len() == 1 {
                        // Single atom
                        let ch = chars_str.chars().next().unwrap();
                        execute_and_print(
                            manager,
                            Command::AddAtom { workspace: ws, ch },
                        )
                        .ok();
                    } else {
                        // Multiple atoms
                        let char_vec: Vec<char> = chars_str.chars().collect();
                        execute_and_print(
                            manager,
                            Command::AddAtoms {
                                workspace: ws,
                                chars: char_vec,
                            },
                        )
                        .ok();
                    }
                } else {
                    eprintln!(
                        "Usage: atom <chars>  (e.g. 'atom a' or 'atom abcde')"
                    );
                }
            }
        },

        "pattern" =>
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(atoms_str) = parts.get(1) {
                    let atom_chars: Vec<char> = atoms_str.chars().collect();
                    execute_and_print(
                        manager,
                        Command::AddSimplePattern {
                            workspace: ws,
                            atoms: atom_chars,
                        },
                    )
                    .ok();
                } else {
                    eprintln!(
                        "Usage: pattern <chars>  (e.g. 'pattern abc' creates pattern from atoms a, b, c)"
                    );
                }
            },

        "vertex" =>
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(index_str) = parts.get(1) {
                    match index_str.parse::<usize>() {
                        Ok(index) => {
                            execute_and_print(
                                manager,
                                Command::GetVertex {
                                    workspace: ws,
                                    index,
                                },
                            )
                            .ok();
                        },
                        Err(_) => {
                            eprintln!(
                                "Error: '{}' is not a valid index",
                                index_str
                            );
                        },
                    }
                } else {
                    eprintln!("Usage: vertex <index>");
                }
            },

        "vertices" =>
            if let Some(ws) = require_workspace(current_ws) {
                execute_and_print(
                    manager,
                    Command::ListVertices { workspace: ws },
                )
                .ok();
            },

        "atoms" =>
            if let Some(ws) = require_workspace(current_ws) {
                execute_and_print(
                    manager,
                    Command::ListAtoms { workspace: ws },
                )
                .ok();
            },

        // -- Search commands (Phase 2) --------------------------------------
        "search" =>
            if let Some(ws) = require_workspace(current_ws) {
                if parts.len() < 2 {
                    eprintln!(
                        "Usage: search <text>  or  search <ref1> <ref2> ..."
                    );
                } else if parts.len() == 2 && parts[1].parse::<usize>().is_err()
                {
                    // Single non-numeric argument → search as text sequence
                    let text = parts[1].to_string();
                    execute_and_print(
                        manager,
                        Command::SearchSequence {
                            workspace: ws,
                            text,
                        },
                    )
                    .ok();
                } else {
                    // Multiple arguments or single numeric → search as pattern with TokenRefs
                    let query: Vec<TokenRef> =
                        parts[1..].iter().map(|s| parse_token_ref(s)).collect();
                    execute_and_print(
                        manager,
                        Command::SearchPattern {
                            workspace: ws,
                            query,
                        },
                    )
                    .ok();
                }
            },

        // -- Insert commands (Phase 2) --------------------------------------
        "insert" =>
            if let Some(ws) = require_workspace(current_ws) {
                if parts.len() < 2 {
                    eprintln!(
                        "Usage: insert <text>  (inserts the text as a sequence)"
                    );
                } else {
                    // Join all remaining parts as the text to insert
                    let text = parts[1..].join(" ");
                    execute_and_print(
                        manager,
                        Command::InsertSequence {
                            workspace: ws,
                            text,
                        },
                    )
                    .ok();
                }
            },

        "insert-match" =>
            if let Some(ws) = require_workspace(current_ws) {
                if parts.len() < 3 {
                    eprintln!(
                        "Usage: insert-match <ref1> <ref2> ...  (at least 2 token refs)"
                    );
                } else {
                    let query: Vec<TokenRef> =
                        parts[1..].iter().map(|s| parse_token_ref(s)).collect();
                    execute_and_print(
                        manager,
                        Command::InsertFirstMatch {
                            workspace: ws,
                            query,
                        },
                    )
                    .ok();
                }
            },

        "insert-bulk" =>
            if let Some(ws) = require_workspace(current_ws) {
                if parts.len() < 2 {
                    eprintln!(
                        "Usage: insert-bulk <text1> <text2> ...  (each arg is a sequence)"
                    );
                } else {
                    let texts: HashSet<String> =
                        parts[1..].iter().map(|s| s.to_string()).collect();
                    execute_and_print(
                        manager,
                        Command::InsertSequences {
                            workspace: ws,
                            texts,
                        },
                    )
                    .ok();
                }
            },

        // -- Read commands (Phase 2) ----------------------------------------
        "read" =>
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(index_str) = parts.get(1) {
                    match index_str.parse::<usize>() {
                        Ok(index) => {
                            execute_and_print(
                                manager,
                                Command::ReadPattern {
                                    workspace: ws,
                                    index,
                                },
                            )
                            .ok();
                        },
                        Err(_) => {
                            eprintln!(
                                "Error: '{}' is not a valid index",
                                index_str
                            );
                        },
                    }
                } else {
                    eprintln!("Usage: read <index>");
                }
            },

        "text" =>
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(index_str) = parts.get(1) {
                    match index_str.parse::<usize>() {
                        Ok(index) => {
                            execute_and_print(
                                manager,
                                Command::ReadAsText {
                                    workspace: ws,
                                    index,
                                },
                            )
                            .ok();
                        },
                        Err(_) => {
                            eprintln!(
                                "Error: '{}' is not a valid index",
                                index_str
                            );
                        },
                    }
                } else {
                    eprintln!("Usage: text <index>");
                }
            },

        // -- Validation (Phase 2) -------------------------------------------
        "validate" =>
            if let Some(ws) = require_workspace(current_ws) {
                execute_and_print(
                    manager,
                    Command::ValidateGraph { workspace: ws },
                )
                .ok();
            },

        // -- Show commands --------------------------------------------------
        "show" =>
            if let Some(ws) = require_workspace(current_ws) {
                if let Some(arg) = parts.get(1) {
                    match arg.parse::<usize>() {
                        Ok(index) => {
                            execute_and_print(
                                manager,
                                Command::ShowVertex {
                                    workspace: ws,
                                    index,
                                },
                            )
                            .ok();
                        },
                        Err(_) => {
                            eprintln!(
                                "Error: '{}' is not a valid vertex index",
                                arg
                            );
                        },
                    }
                } else {
                    execute_and_print(
                        manager,
                        Command::ShowGraph { workspace: ws },
                    )
                    .ok();
                }
            },

        // -- Debug / introspection ------------------------------------------
        "snapshot" =>
            if let Some(ws) = require_workspace(current_ws) {
                execute_and_print(
                    manager,
                    Command::GetSnapshot { workspace: ws },
                )
                .ok();
            },

        "stats" =>
            if let Some(ws) = require_workspace(current_ws) {
                execute_and_print(
                    manager,
                    Command::GetStatistics { workspace: ws },
                )
                .ok();
            },

        // -- Informational --------------------------------------------------
        "ws" | "workspace" => match current_ws {
            Some(name) => println!("Current workspace: {name}"),
            None => println!(
                "No workspace is currently active. Use 'create' or 'open'."
            ),
        },

        "use" | "switch" =>
            if let Some(name) = parts.get(1) {
                if manager.is_open(name) {
                    *current_ws = Some(name.to_string());
                    println!("Switched to workspace '{}'", name);
                } else {
                    eprintln!(
                        "Workspace '{}' is not open. Use 'open {}' first.",
                        name, name
                    );
                }
            } else {
                eprintln!(
                    "Usage: use <name>  (switch to an already-open workspace)"
                );
            },

        _ => {
            eprintln!(
                "Unknown command: '{}'. Type 'help' for available commands.",
                parts[0]
            );
        },
    }
}

/// Execute a command and print the result. Returns `Ok(())` on success,
/// `Err(())` if the command returned an error (which is printed to stderr).
fn execute_and_print(
    manager: &mut WorkspaceManager,
    cmd: Command,
) -> Result<(), ()> {
    match execute(manager, cmd) {
        Ok(result) => {
            output::print_command_result(&result);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {e}");
            Err(())
        },
    }
}

/// Ensure a current workspace is set, printing an error if not.
/// Returns the workspace name if available.
fn require_workspace(current_ws: &Option<String>) -> Option<String> {
    match current_ws {
        Some(name) => Some(name.clone()),
        None => {
            eprintln!(
                "No workspace is active. Use 'create <name>' or 'open <name>' first."
            );
            None
        },
    }
}

/// Print the help text listing all available REPL commands.
fn print_help() {
    println!();
    println!("Workspace commands:");
    println!(
        "  create <name>        Create a new workspace and set it as active"
    );
    println!(
        "  open <name>          Open a workspace from disk and set it as active"
    );
    println!("  close [<name>]       Close the current (or named) workspace");
    println!(
        "  save [<name>]        Save the current (or named) workspace to disk"
    );
    println!("  list                 List all workspaces (open and on-disk)");
    println!("  delete <name>        Delete a workspace from disk");
    println!(
        "  use <name>           Switch active workspace (must already be open)"
    );
    println!("  ws                   Show the currently active workspace");
    println!();
    println!("Graph commands (require an active workspace):");
    println!("  atom <chars>         Add atom(s): 'atom a' or 'atom abcde'");
    println!(
        "  pattern <chars>      Add a simple pattern from atoms: 'pattern abc'"
    );
    println!("  vertex <index>       Show detailed vertex information");
    println!("  vertices             List all vertices");
    println!("  atoms                List all atoms");
    println!();
    println!("Search commands:");
    println!(
        "  search <text>        Search for a text sequence (e.g. 'search hello')"
    );
    println!(
        "  search <r1> <r2> ... Search by token refs (numbers=indices, strings=labels)"
    );
    println!();
    println!("Insert commands:");
    println!(
        "  insert <text>        Insert a text sequence (e.g. 'insert hello world')"
    );
    println!(
        "  insert-match <refs>  Insert by token refs (numbers=indices, strings=labels)"
    );
    println!(
        "  insert-bulk <t1> ... Bulk insert multiple sequences (each arg is one)"
    );
    println!();
    println!("Read commands:");
    println!("  read <index>         Read a vertex as a decomposition tree");
    println!("  text <index>         Read a vertex as concatenated leaf text");
    println!();
    println!("Debug commands:");
    println!("  validate             Validate graph integrity");
    println!("  snapshot             Print graph snapshot as JSON");
    println!("  stats                Print graph statistics");
    println!(
        "  show                 Show the entire graph (all vertices with children & parents)"
    );
    println!(
        "  show <index>         Show a single vertex with its children and parents"
    );
    println!();
    println!("General:");
    println!("  help / ?             Show this help");
    println!("  quit / exit          Exit the REPL");
    println!();
}
