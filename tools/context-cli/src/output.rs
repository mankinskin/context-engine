//! Human-friendly output formatting for CLI command results.
//!
//! Each `CommandResult` variant has a dedicated formatting function that
//! prints readable, structured output to stdout. The `print_command_result`
//! function dispatches to the appropriate formatter.

use context_api::{
    commands::CommandResult,
    types::{
        AtomInfo,
        GraphStatistics,
        InsertResult,
        PatternInfo,
        PatternReadResult,
        ReadNode,
        SearchResult,
        TokenInfo,
        ValidationReport,
        VertexInfo,
        WorkspaceInfo,
    },
};

/// Dispatch a `CommandResult` to the appropriate human-friendly printer.
pub fn print_command_result(result: &CommandResult) {
    match result {
        CommandResult::WorkspaceInfo(info) => print_workspace_info(info),
        CommandResult::WorkspaceInfoList { workspaces } => {
            print_workspace_info_list(workspaces);
        },
        CommandResult::AtomInfo(info) => print_atom_info(info),
        CommandResult::AtomInfoList { atoms } => print_atom_info_list(atoms),
        CommandResult::OptionalAtomInfo { atom } => match atom {
            Some(info) => print_atom_info(info),
            None => println!("Atom not found."),
        },
        CommandResult::PatternInfo(info) => print_pattern_info(info),
        CommandResult::OptionalVertexInfo { vertex } => match vertex {
            Some(info) => print_vertex_info(info),
            None => println!("Vertex not found."),
        },
        CommandResult::TokenInfoList { tokens } =>
            print_token_info_list(tokens),
        CommandResult::SearchResult(result) => print_search_result(result),
        CommandResult::InsertResult(result) => print_insert_result(result),
        CommandResult::InsertResultList { results } => {
            print_insert_result_list(results);
        },
        CommandResult::ReadResult(result) => print_read_result(result),
        CommandResult::Text { text } => println!("{text}"),
        CommandResult::Snapshot(snapshot) => {
            // Print snapshot as pretty JSON
            match serde_json::to_string_pretty(snapshot) {
                Ok(json) => println!("{json}"),
                Err(e) => eprintln!("Error serializing snapshot: {e}"),
            }
        },
        CommandResult::Statistics(stats) => print_statistics(stats),
        CommandResult::ValidationReport(report) => {
            print_validation_report(report);
        },
        CommandResult::Ok => println!("Ok."),
    }
}

/// Print workspace summary information.
pub fn print_workspace_info(info: &WorkspaceInfo) {
    println!("Workspace: {}", info.name);
    println!(
        "  Vertices: {}, Atoms: {}, Patterns: {}",
        info.vertex_count, info.atom_count, info.pattern_count
    );
    println!("  Created:  {}", info.created_at);
    println!("  Modified: {}", info.modified_at);
}

/// Print a list of workspace summaries.
pub fn print_workspace_info_list(workspaces: &[WorkspaceInfo]) {
    if workspaces.is_empty() {
        println!("No workspaces found.");
        return;
    }

    println!("Workspaces ({}):", workspaces.len());
    // Calculate column widths for alignment
    let max_name_len = workspaces
        .iter()
        .map(|w| w.name.len())
        .max()
        .unwrap_or(4)
        .max(4);

    println!(
        "  {:<width$}  {:>8}  {:>6}  {:>8}  {}",
        "Name",
        "Vertices",
        "Atoms",
        "Patterns",
        "Modified",
        width = max_name_len
    );
    println!(
        "  {:<width$}  {:>8}  {:>6}  {:>8}  {}",
        "----",
        "--------",
        "------",
        "--------",
        "--------",
        width = max_name_len
    );

    for info in workspaces {
        println!(
            "  {:<width$}  {:>8}  {:>6}  {:>8}  {}",
            info.name,
            info.vertex_count,
            info.atom_count,
            info.pattern_count,
            info.modified_at,
            width = max_name_len
        );
    }
}

/// Print information about a single atom.
pub fn print_atom_info(info: &AtomInfo) {
    println!("Atom '{}' (index: {})", info.ch, info.index);
}

/// Print a list of atoms.
pub fn print_atom_info_list(atoms: &[AtomInfo]) {
    if atoms.is_empty() {
        println!("No atoms.");
        return;
    }

    println!("Atoms ({}):", atoms.len());
    for info in atoms {
        println!("  '{}' -> {}", info.ch, info.index);
    }
}

/// Print information about a newly created pattern.
pub fn print_pattern_info(info: &PatternInfo) {
    println!(
        "Pattern \"{}\" (index: {}, width: {})",
        info.label, info.index, info.width
    );

    if !info.children.is_empty() {
        let children_str: Vec<String> = info
            .children
            .iter()
            .map(|c| format!("\"{}\"({})", c.label, c.index))
            .collect();
        println!("  Children: {}", children_str.join(" -> "));
    }
}

/// Print detailed vertex information.
pub fn print_vertex_info(info: &VertexInfo) {
    let kind = if info.is_atom { "atom" } else { "pattern" };
    println!(
        "Vertex {} \"{}\" (width: {}, {})",
        info.index, info.label, info.width, kind
    );

    if !info.children.is_empty() {
        for (i, pattern) in info.children.iter().enumerate() {
            let children_str: Vec<String> = pattern
                .iter()
                .map(|c| format!("\"{}\"({})", c.label, c.index))
                .collect();
            println!("  Pattern {}: {}", i, children_str.join(", "));
        }
    }

    println!("  Parents: {}", info.parent_count);
}

/// Print a list of vertices (lightweight token info).
pub fn print_token_info_list(tokens: &[TokenInfo]) {
    if tokens.is_empty() {
        println!("No vertices.");
        return;
    }

    println!("Vertices ({}):", tokens.len());
    for info in tokens {
        println!(
            "  [{:>4}] \"{}\"{:>width$}(width: {})",
            info.index,
            info.label,
            "",
            info.width,
            width = 8usize.saturating_sub(info.label.len())
        );
    }
}

/// Print a search result.
pub fn print_search_result(result: &SearchResult) {
    if result.complete {
        if let Some(token) = &result.token {
            println!(
                "Found: \"{}\" (index: {}, width: {})",
                token.label, token.index, token.width
            );
        } else {
            println!("Found (complete match).");
        }
    } else if let Some(partial) = &result.partial {
        println!("Partial match ({:?}):", partial.kind);
        if let Some(root) = &partial.root_token {
            println!(
                "  Root: \"{}\" (index: {}, width: {})",
                root.label, root.index, root.width
            );
        }
        if result.query_exhausted {
            println!("  Query fully consumed.");
        } else {
            println!("  Query not fully consumed.");
        }
    } else {
        println!("Not found.");
    }
}

/// Print an insert result.
pub fn print_insert_result(result: &InsertResult) {
    if result.already_existed {
        println!(
            "= Existing: \"{}\" (index: {}, width: {})",
            result.token.label, result.token.index, result.token.width
        );
    } else {
        println!(
            "+ Inserted: \"{}\" (index: {}, width: {})",
            result.token.label, result.token.index, result.token.width
        );
    }
}

/// Print a list of insert results.
pub fn print_insert_result_list(results: &[InsertResult]) {
    if results.is_empty() {
        println!("No sequences inserted.");
        return;
    }

    println!("Inserted {} sequence(s):", results.len());
    for result in results {
        let status = if result.already_existed { "=" } else { "+" };
        println!(
            "  {} \"{}\" (index: {}, width: {})",
            status, result.token.label, result.token.index, result.token.width
        );
    }
}

/// Print a read (decomposition) result.
pub fn print_read_result(result: &PatternReadResult) {
    println!(
        "Root: \"{}\" (index: {}, width: {})",
        result.root.label, result.root.index, result.root.width
    );
    println!("Text: \"{}\"", result.text);
    println!("Tree:");
    print_read_tree(&result.tree, 1);
}

/// Recursively print a decomposition tree node with indentation.
fn print_read_tree(
    node: &ReadNode,
    depth: usize,
) {
    let indent = "  ".repeat(depth);
    if node.children.is_empty() {
        println!("{indent}'{}' [{}]", node.token.label, node.token.index);
    } else {
        println!(
            "{indent}\"{}\" [{}] (width: {})",
            node.token.label, node.token.index, node.token.width
        );
        for child in &node.children {
            print_read_tree(child, depth + 1);
        }
    }
}

/// Print aggregate graph statistics.
pub fn print_statistics(stats: &GraphStatistics) {
    println!("Graph Statistics:");
    println!("  Vertices:  {}", stats.vertex_count);
    println!("  Atoms:     {}", stats.atom_count);
    println!("  Patterns:  {}", stats.pattern_count);
    println!("  Edges:     {}", stats.edge_count);
    println!("  Max width: {}", stats.max_width);
}

/// Print a validation report.
pub fn print_validation_report(report: &ValidationReport) {
    if report.valid {
        println!("Graph is valid ({} vertices checked).", report.vertex_count);
    } else {
        println!(
            "Graph has {} issue(s) ({} vertices checked):",
            report.issues.len(),
            report.vertex_count
        );
        for issue in &report.issues {
            println!("  - {issue}");
        }
    }
}
