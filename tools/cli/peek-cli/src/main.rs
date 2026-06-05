//! peek — token-bounded file inspection utility
//!
//! Reads a targeted line window from a file rather than pulling the whole file.
//! Bounded reads are the default; full-file reads require an explicit opt-in flag.
//!
//! ## Usage
//!
//! ```text
//! # Read lines 42–80 (bounded, recommended)
//! peek path/to/file.rs --start 42 --end 80
//!
//! # Read 20 lines from line 100 (window shorthand)
//! peek path/to/file.rs --start 100 --window 20
//!
//! # Read the first N lines (head-style)
//! peek path/to/file.rs --head 30
//!
//! # Read the last N lines (tail-style)
//! peek path/to/file.rs --tail 30
//!
//! # Find a pattern and show a window around the match
//! peek path/to/file.rs --grep "fn my_function" --window 15
//!
//! # Count total lines (useful for planning bounded reads)
//! peek path/to/file.rs --count
//!
//! # Escape hatch: full file (add --all to acknowledge the cost)
//! peek path/to/file.rs --all
//! ```

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use serde_json::{Value, json};

const REPO_MAP_FILE: &str = "repo_map.toon";

/// peek — token-bounded file inspection.
///
/// Reads a targeted slice of a file. Bounded reads are the default interaction
/// pattern; use `--all` only when the entire file is genuinely needed.
#[derive(Debug, Parser)]
#[command(name = "peek", version, about)]
struct Args {
    /// Path to the file to inspect.
    file: PathBuf,

    /// Generate the repository structural map for the target root.
    ///
    /// This emits a compact tree-shaped workspace map suitable for
    /// root-level `repo_map.toon` regeneration.
    #[arg(long)]
    repo_map: bool,

    /// Write generated tree output to a file instead of stdout.
    ///
    /// Currently supported with `--repo-map`.
    #[arg(long)]
    output: Option<PathBuf>,

    /// First line to include (1-based, inclusive).
    #[arg(long, short = 's')]
    start: Option<usize>,

    /// Last line to include (1-based, inclusive). Requires --start.
    #[arg(long, short = 'e')]
    end: Option<usize>,

    /// Number of lines to show after --start (alternative to --end).
    /// When used without --start, anchors at line 1.
    #[arg(long, short = 'w')]
    window: Option<usize>,

    /// Show the first N lines (like `head -n N`).
    #[arg(long)]
    head: Option<usize>,

    /// Show the last N lines (like `tail -n N`).
    #[arg(long)]
    tail: Option<usize>,

    /// Search for a pattern and print matching line numbers (no content).
    /// Combine with --window to show context around the first match.
    #[arg(long, short = 'g')]
    grep: Option<String>,

    /// Print total line count only (useful for planning bounded reads before
    /// choosing --start/--end coordinates).
    #[arg(long, short = 'c')]
    count: bool,

    /// Escape hatch: read the entire file.
    ///
    /// Prefer bounded reads whenever possible. Only use this flag when the
    /// complete file content is genuinely required and no targeted slice will
    /// suffice. The flag name is intentional — it makes the token cost visible
    /// in command history.
    #[arg(long)]
    all: bool,

    /// Skeletonize: strip function/method bodies and return only structural
    /// signatures, type definitions, trait bounds, and doc comments.
    ///
    /// Supports Rust and Python files (detected by extension).
    /// Use this to map the architecture of a file before deciding which
    /// function bodies to read in detail.
    #[arg(long, short = 'k')]
    skeleton: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.repo_map {
        if args.skeleton
            || args.start.is_some()
            || args.end.is_some()
            || args.window.is_some()
            || args.head.is_some()
            || args.tail.is_some()
            || args.grep.is_some()
            || args.count
            || args.all
        {
            bail!("--repo-map cannot be combined with file inspection flags");
        }

        let map = generate_repo_map(&args.file)?;
        write_generated_output(&map, args.output.as_deref())?;
        return Ok(());
    }

    // --skeleton: emit only structural signatures.
    if args.skeleton {
        if args.file.is_dir() {
            let tree = skeletonize_directory(&args.file)?;
            print!("{tree}");
            return Ok(());
        }

        let file = File::open(&args.file)
            .with_context(|| format!("cannot open '{}'", args.file.display()))?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .collect::<std::io::Result<_>>()
            .with_context(|| format!("error reading '{}'", args.file.display()))?;
        return skeletonize(&args.file, &lines);
    }

    // Validation: reject conflicting combinations early.
    if args.all
        && (args.start.is_some()
            || args.end.is_some()
            || args.window.is_some()
            || args.head.is_some()
            || args.tail.is_some()
            || args.grep.is_some()
            || args.count)
    {
        bail!("--all cannot be combined with bounded inspection flags");
    }

    if args.end.is_some() && args.start.is_none() {
        bail!("--end requires --start");
    }

    if args.end.is_some() && args.window.is_some() {
        bail!("--end and --window are mutually exclusive");
    }

    let file = File::open(&args.file)
        .with_context(|| format!("cannot open '{}'", args.file.display()))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<std::io::Result<_>>()
        .with_context(|| format!("error reading '{}'", args.file.display()))?;

    let total = lines.len();

    // --count: just print the line count.
    if args.count {
        println!("{total}");
        return Ok(());
    }

    // --grep: find matching lines (and optionally show a window around the first match).
    if let Some(ref pattern) = args.grep {
        let matches: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.contains(pattern.as_str()))
            .map(|(i, _)| i + 1) // 1-based
            .collect();

        if matches.is_empty() {
            eprintln!("peek: no match for {:?} in {}", pattern, args.file.display());
            return Ok(());
        }

        if let Some(w) = args.window {
            // Show a window around the first match.
            let first = matches[0];
            let start = first.saturating_sub(w / 2);
            let end = (first + w / 2).min(total);
            print_window(&lines, start.max(1), end, total)?;
        } else {
            // Just print matching line numbers.
            for m in &matches {
                println!("{m}");
            }
        }
        return Ok(());
    }

    // --all: full file (explicit escape hatch).
    if args.all {
        for (i, line) in lines.iter().enumerate() {
            println!("{:>6} {line}", i + 1);
        }
        return Ok(());
    }

    // --head N
    if let Some(n) = args.head {
        let end = n.min(total);
        print_window(&lines, 1, end, total)?;
        return Ok(());
    }

    // --tail N
    if let Some(n) = args.tail {
        let start = total.saturating_sub(n) + 1;
        print_window(&lines, start, total, total)?;
        return Ok(());
    }

    // --start [--end | --window]
    if let Some(start) = args.start {
        if start == 0 {
            bail!("--start is 1-based; use --start 1 for the first line");
        }
        if start > total {
            bail!("--start {start} exceeds file length ({total} lines)");
        }

        let end = if let Some(e) = args.end {
            if e < start {
                bail!("--end ({e}) must be >= --start ({start})");
            }
            e.min(total)
        } else if let Some(w) = args.window {
            (start + w - 1).min(total)
        } else {
            // Default window: 40 lines from start.
            const DEFAULT_WINDOW: usize = 40;
            (start + DEFAULT_WINDOW - 1).min(total)
        };

        print_window(&lines, start, end, total)?;
        return Ok(());
    }

    // No mode selected — require the caller to choose a bounded mode.
    bail!(
        "peek requires a bounded read mode.\n\
         \n\
         Common patterns:\n\
         \n\
         # Targeted window (recommended)\n\
         peek {path} --start 42 --end 80\n\
         \n\
         # Window from start\n\
         peek {path} --start 100 --window 20\n\
         \n\
         # Head / tail\n\
         peek {path} --head 30\n\
         peek {path} --tail 30\n\
         \n\
         # Find a function, then show context\n\
         peek {path} --grep \"fn my_fn\" --window 15\n\
         \n\
         # Count lines before choosing coordinates\n\
         peek {path} --count\n\
         \n\
         # Architecture map (signatures only, no bodies)\n\
         peek {path} --skeleton\n\
         \n\
         # Full file (token-expensive — explicit opt-in)\n\
         peek {path} --all",
        path = args.file.display(),
    );
}

fn print_window(lines: &[String], start: usize, end: usize, total: usize) -> Result<()> {
    let start = start.max(1);
    let end = end.min(total);
    for i in start..=end {
        println!("{:>6} {}", i, lines[i - 1]);
    }
    Ok(())
}

/// Skeletonize a source file: emit only structural lines (signatures, type
/// definitions, trait impls, doc comments, use/mod/struct/enum/const/static
/// declarations) while collapsing function/method bodies to `{ ... }`.
///
/// Supports:
/// - Rust (`.rs`) — collapses brace-delimited bodies.
/// - Python (`.py`) — collapses indented bodies after `def`/`class` lines.
/// - Other file types — falls back to a heuristic blank-line collapsing pass.
fn skeletonize(path: &std::path::Path, lines: &[String]) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "rs" => skeletonize_rust(lines),
        "py" => skeletonize_python(lines),
        _ => skeletonize_generic(lines),
    }
}

/// Rust skeletonization: track brace depth and collapse bodies.
fn skeletonize_rust(lines: &[String]) -> Result<()> {
    // State: are we inside a collapsed body?
    let mut depth: i64 = 0;
    let mut collapse_start_depth: Option<i64> = None;

    for (i, line) in lines.iter().enumerate() {
        let lineno = i + 1;
        let trimmed = line.trim();

        // Always emit structural lines regardless of depth:
        // doc comments, use, mod, struct, enum, type, trait, impl, const, static,
        // pub fn / fn / async fn, attribute macros (#[...]), and blank lines.
        let is_structural = trimmed.is_empty()
            || trimmed.starts_with("///")
            || trimmed.starts_with("//!")
            || trimmed.starts_with("#[")
            || trimmed.starts_with("#!")
            || trimmed.starts_with("use ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("mod ")
            || trimmed.starts_with("pub mod ")
            || trimmed.starts_with("pub struct")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("pub enum")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("pub type")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("pub trait")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("pub const")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("pub static")
            || trimmed.starts_with("static ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("pub impl")
            || trimmed.starts_with("pub fn")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("async fn")
            || trimmed.starts_with("pub async fn")
            || trimmed.starts_with("extern ");

        // Count braces on this line.
        let open: i64 = line.chars().filter(|&c| c == '{').count() as i64;
        let close: i64 = line.chars().filter(|&c| c == '}').count() as i64;

        if let Some(cd) = collapse_start_depth {
            // We're inside a collapsed body — track depth.
            depth += open - close;
            if depth <= cd {
                // Body closed.
                collapse_start_depth = None;
            }
            // Don't emit body lines.
        } else if is_structural || depth == 0 {
            // Emit this line.
            // If it opens a body (ends with `{`), start collapsing the interior.
            if open > close && (trimmed.ends_with('{') || trimmed.contains("->")) {
                // This line itself is structural; next lines are the body.
                println!("{lineno:>6} {line}");
                if trimmed.ends_with('{') {
                    collapse_start_depth = Some(depth + open - close - 1);
                    depth += open - close;
                    // Replace subsequent body with "    // ..."
                    println!("       // ...");
                } else {
                    depth += open - close;
                }
            } else {
                println!("{lineno:>6} {line}");
                depth += open - close;
            }
        } else {
            depth += open - close;
        }
    }
    Ok(())
}

/// Python skeletonization: collapse indented bodies after `def`/`class`.
fn skeletonize_python(lines: &[String]) -> Result<()> {
    let mut collapse_indent: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        let lineno = i + 1;

        // Compute this line's indent.
        let indent = line.len() - line.trim_start().len();
        let trimmed = line.trim();

        if trimmed.is_empty() {
            println!("{lineno:>6} {line}");
            continue;
        }

        if let Some(col_indent) = collapse_indent {
            if indent > col_indent {
                // Inside collapsed body — skip.
                continue;
            } else {
                // Body ended.
                collapse_indent = None;
            }
        }

        // Check if this starts a body.
        let starts_body = trimmed.starts_with("def ")
            || trimmed.starts_with("async def ")
            || trimmed.starts_with("class ");

        println!("{lineno:>6} {line}");

        if starts_body && trimmed.ends_with(':') {
            // Next lines at deeper indent are the body.
            collapse_indent = Some(indent);
            println!("       # ...");
        }
    }
    Ok(())
}

/// Generic skeletonization: emit non-blank lines that look like declarations,
/// collapse runs of indented non-empty lines to `// ...`.
fn skeletonize_generic(lines: &[String]) -> Result<()> {
    let mut in_body = false;
    for (i, line) in lines.iter().enumerate() {
        let lineno = i + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            in_body = false;
            println!("{lineno:>6} {line}");
        } else if line.starts_with(' ') || line.starts_with('\t') {
            if !in_body {
                println!("       // ...");
                in_body = true;
            }
        } else {
            in_body = false;
            println!("{lineno:>6} {line}");
        }
    }
    Ok(())
}

#[derive(Debug, Default)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
    note: Option<String>,
    is_file: bool,
}

fn write_generated_output(text: &str, output: Option<&Path>) -> Result<()> {
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("cannot create '{}'", parent.display()))?;
        }
        fs::write(path, text).with_context(|| format!("cannot write '{}'", path.display()))?;
    } else {
        print!("{text}");
    }
    Ok(())
}

fn generate_repo_map(root: &Path) -> Result<String> {
    let root = root
        .canonicalize()
        .with_context(|| format!("cannot resolve '{}'", root.display()))?;
    let cargo_toml = root.join("Cargo.toml");
    let cargo_text = fs::read_to_string(&cargo_toml)
        .with_context(|| format!("cannot read '{}'", cargo_toml.display()))?;

    let members = parse_workspace_members(&cargo_text);
    let mut crate_tree = TreeNode::default();
    for member in members {
        let crate_name = read_crate_name(&root.join(&member))?;
        insert_tree_path(
            &mut crate_tree,
            &member,
            crate_name.map(|name| format!("crate={name}")),
            false,
        );
    }

    let top_dirs = collect_top_level_dirs(&root)?;
    let agent_guidance = collect_existing_paths(
        &root,
        &[
            "AGENTS.md",
            ".agents/instructions/token-efficiency.instructions.md",
            "CHEAT_SHEET.md",
        ],
    );
    let agent_files = collect_agent_files(&root)?;
    let hooks = collect_hook_files(&root)?;

    let repo_map = json!({
        "format": "repo_map_toon_v1",
        "description": "Compact workspace structural map for low-token orientation",
        "usage": "Read this before opening source files for structural orientation",
        "refresh_command": format!(
            "cargo run -p peek-cli -- . --repo-map --output {REPO_MAP_FILE}"
        ),
        "workspace": {
            "root": display_path(&root),
        },
        "top_level_dirs": top_dirs,
        "crates": tree_to_value("crates", &crate_tree, "crates"),
        "agent_guidance": tree_to_value(
            "agent-guidance",
            &tree_from_paths(&agent_guidance),
            "agent-guidance",
        ),
        "agent_files": tree_to_value(
            "agent-files",
            &tree_from_paths(&agent_files),
            "agent-files",
        ),
        "hooks": tree_to_value("hooks", &tree_from_paths(&hooks), "hooks"),
        "key_tools": [
            {
                "path": "target/debug/ticket.exe",
                "description": "ticket-cli (state machine, board, deps)",
            },
            {
                "path": "target/debug/spec.exe",
                "description": "spec-cli",
            },
            {
                "path": "target/debug/peek",
                "description": "bounded inspection + repo-map generation",
            },
            {
                "path": "rtk <cmd>",
                "description": "token-optimized CLI proxy (auto-compress output)",
            }
        ],
        "bounded_inspection_pattern": [
            "peek <file> --count  # 1. learn size",
            "peek <file> --grep <pattern>  # 2. locate target line",
            "peek <file> --start N --end M  # 3. bounded read",
            format!(
                "peek . --repo-map --output {REPO_MAP_FILE}  # refresh structural map"
            ),
        ],
    });

    toon_format::encode_default(&repo_map)
        .context("failed to encode repo map as TOON")
}

fn skeletonize_directory(root: &Path) -> Result<String> {
    let root = root
        .canonicalize()
        .with_context(|| format!("cannot resolve '{}'", root.display()))?;
    let mut tree = TreeNode::default();
    collect_directory_tree(&root, &root, &mut tree)?;

    let root_name = root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| root.display().to_string());

    let mut lines = vec![format!("{root_name}/")];
    lines.extend(render_tree_lines(&tree, 1));
    Ok(lines.join("\n") + "\n")
}

fn collect_directory_tree(root: &Path, current: &Path, tree: &mut TreeNode) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(current)
        .with_context(|| format!("cannot read '{}'", current.display()))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("cannot read '{}'", current.display()))?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let metadata = entry
            .metadata()
            .with_context(|| format!("cannot stat '{}'", path.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative = path
            .strip_prefix(root)
            .with_context(|| format!("cannot relativize '{}'", path.display()))?;

        if should_skip_directory_entry(relative, &name, metadata.is_dir()) {
            continue;
        }

        if metadata.is_dir() {
            insert_tree_path(tree, &to_slash_path(relative), None, false);
            collect_directory_tree(root, &path, tree)?;
        } else if should_include_structural_file(relative) {
            insert_tree_path(tree, &to_slash_path(relative), None, true);
        }
    }

    Ok(())
}

fn should_skip_directory_entry(relative: &Path, name: &str, is_dir: bool) -> bool {
    let noisy_names = [
        "target",
        "node_modules",
        "dist",
        "build",
        ".git",
        ".jj",
        ".idea",
        ".vscode",
        "__pycache__",
    ];
    if noisy_names.contains(&name) {
        return true;
    }

    if name.starts_with('.')
        && !matches!(
            name,
            ".agents" | ".githooks" | ".github" | ".rule"
        )
    {
        return true;
    }

    if !is_dir {
        let path = to_slash_path(relative);
        return path.ends_with(".toon") && path != REPO_MAP_FILE;
    }

    false
}

fn should_include_structural_file(relative: &Path) -> bool {
    let path = to_slash_path(relative);
    let Some(name) = relative.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    matches!(
        name,
        "Cargo.toml"
            | "README.md"
            | "HIGH_LEVEL_GUIDE.md"
            | "AGENTS.md"
            | "CHEAT_SHEET.md"
            | "Makefile.toml"
            | "rust-toolchain.toml"
            | "rustfmt.toml"
            | "viewer-ctl.toml"
            | "repo_map.toon"
    ) || path.ends_with(".instructions.md")
        || path.ends_with(".prompt.md")
        || path.ends_with(".SKILL.md")
        || path.ends_with(".agent.md")
        || path.ends_with(".sh")
        || path.ends_with(".yaml")
        || path.ends_with(".yml")
        || path.ends_with("hooks.json")
}

fn collect_top_level_dirs(root: &Path) -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(root)
        .with_context(|| format!("cannot read '{}'", root.display()))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("cannot read '{}'", root.display()))?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let metadata = entry
            .metadata()
            .with_context(|| format!("cannot stat '{}'", entry.path().display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if metadata.is_dir() && !name.starts_with('.') && name != "target" {
            dirs.push(name);
        }
    }

    Ok(dirs)
}

fn collect_existing_paths(root: &Path, paths: &[&str]) -> Vec<String> {
    paths.iter()
        .filter_map(|path| {
            let full = root.join(path);
            full.exists().then(|| path.replace('\\', "/"))
        })
        .collect()
}

fn collect_agent_files(root: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for subdir in ["instructions", "prompts", "skills"] {
        let dir = root.join(".agents").join(subdir);
        if !dir.exists() {
            continue;
        }

        let mut entries: Vec<_> = fs::read_dir(&dir)
            .with_context(|| format!("cannot read '{}'", dir.display()))?
            .collect::<std::io::Result<Vec<_>>>()
            .with_context(|| format!("cannot read '{}'", dir.display()))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry
                .metadata()
                .with_context(|| format!("cannot stat '{}'", entry.path().display()))?
                .is_file()
            {
                let path = entry.path();
                let relative = path
                    .strip_prefix(root)
                    .with_context(|| format!("cannot relativize '{}'", path.display()))?;
                files.push(to_slash_path(relative));
            }
        }
    }
    Ok(files)
}

fn collect_hook_files(root: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for dir in [root.join(".githooks"), root.join(".github").join("hooks")] {
        if !dir.exists() {
            continue;
        }

        let mut entries: Vec<_> = fs::read_dir(&dir)
            .with_context(|| format!("cannot read '{}'", dir.display()))?
            .collect::<std::io::Result<Vec<_>>>()
            .with_context(|| format!("cannot read '{}'", dir.display()))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry
                .metadata()
                .with_context(|| format!("cannot stat '{}'", entry.path().display()))?
                .is_file()
            {
                let path = entry.path();
                let relative = path
                    .strip_prefix(root)
                    .with_context(|| format!("cannot relativize '{}'", path.display()))?;
                files.push(to_slash_path(relative));
            }
        }
    }
    Ok(files)
}

fn parse_workspace_members(cargo_toml: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut inside_members = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if !inside_members {
            if let Some(index) = trimmed.find("members") {
                let remainder = &trimmed[index..];
                if remainder.starts_with("members") && remainder.contains('[') {
                    inside_members = true;
                    members.extend(extract_quoted_strings(remainder));
                    if remainder.contains(']') {
                        inside_members = false;
                    }
                }
            }
        } else {
            members.extend(extract_quoted_strings(trimmed));
            if trimmed.contains(']') {
                inside_members = false;
            }
        }
    }

    members
}

fn extract_quoted_strings(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            let mut value = String::new();
            for next in chars.by_ref() {
                if next == '"' {
                    break;
                }
                value.push(next);
            }
            if !value.is_empty() {
                out.push(value);
            }
        }
    }
    out
}

fn read_crate_name(member_dir: &Path) -> Result<Option<String>> {
    let cargo_toml = member_dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Ok(None);
    }

    let text = fs::read_to_string(&cargo_toml)
        .with_context(|| format!("cannot read '{}'", cargo_toml.display()))?;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name = ") {
            let value = rest.trim().trim_matches('"');
            if !value.is_empty() {
                return Ok(Some(value.to_string()));
            }
        }
    }
    Ok(None)
}

fn insert_tree_path(tree: &mut TreeNode, path: &str, note: Option<String>, is_file: bool) {
    let mut current = tree;
    let mut segments = path.split('/').peekable();
    while let Some(segment) = segments.next() {
        current = current.children.entry(segment.to_string()).or_default();
        if segments.peek().is_none() {
            current.note = note.clone();
            current.is_file = is_file;
        }
    }
}

fn tree_from_paths(paths: &[String]) -> TreeNode {
    let mut tree = TreeNode::default();
    for path in paths {
        insert_tree_path(&mut tree, path, None, true);
    }
    tree
}

fn render_tree_lines(tree: &TreeNode, indent: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for (name, child) in &tree.children {
        let mut line = format!("{}{}", "  ".repeat(indent), name);
        if !child.is_file {
            line.push('/');
        }
        if let Some(note) = &child.note {
            line.push_str(&format!("  {note}"));
        }
        lines.push(line);
        lines.extend(render_tree_lines(child, indent + 1));
    }
    lines
}

fn to_slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .replace('\\', "/")
}

fn tree_to_value(
    name: &str,
    tree: &TreeNode,
    path: &str,
) -> Value {
    json!({
        "name": name,
        "kind": "dir",
        "path": path,
        "children": tree_children_to_values(&tree.children, path),
    })
}

fn tree_children_to_values(
    children: &BTreeMap<String, TreeNode>,
    parent_path: &str,
) -> Vec<Value> {
    children
        .iter()
        .map(|(name, child)| {
            let path = if parent_path.is_empty() {
                name.clone()
            } else {
                format!("{parent_path}/{name}")
            };

            let mut value = json!({
                "name": name,
                "kind": if child.is_file { "file" } else { "dir" },
                "path": path,
            });

            if let Some(note) = &child.note {
                value["note"] = json!(note);
            }

            if !child.children.is_empty() {
                value["children"] = json!(tree_children_to_values(&child.children, &path));
            }

            value
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        REPO_MAP_FILE,
        TreeNode,
        insert_tree_path,
        parse_workspace_members,
        render_tree_lines,
        tree_from_paths,
    };

    #[test]
    fn parses_workspace_members_from_multiline_array() {
        let cargo = r#"
            [workspace]
            members = [
              "tools/cli/peek-cli",
              "context-stack/context-read",
            ]
        "#;

        assert_eq!(
            parse_workspace_members(cargo),
            vec![
                "tools/cli/peek-cli".to_string(),
                "context-stack/context-read".to_string()
            ]
        );
    }

    #[test]
    fn renders_nested_tree_without_repeating_prefixes() {
        let tree = tree_from_paths(&[
            "tools/cli/peek-cli".to_string(),
            "tools/mcp/compact-terminal-mcp".to_string(),
        ]);
        let lines = render_tree_lines(&tree, 1);

        assert!(lines.contains(&"  tools/".to_string()));
        assert!(lines.contains(&"    cli/".to_string()));
        assert!(lines.contains(&"      peek-cli".to_string()));
        assert!(lines.contains(&"    mcp/".to_string()));
    }

    #[test]
    fn attaches_notes_to_leaf_nodes() {
        let mut tree = TreeNode::default();
        insert_tree_path(&mut tree, "tools/cli/peek-cli", Some("crate=peek-cli".to_string()), false);

        let lines = render_tree_lines(&tree, 1);
        assert!(lines.contains(&"      peek-cli/  crate=peek-cli".to_string()));
    }

    #[test]
    fn repo_map_file_constant_uses_repo_root_target() {
        assert_eq!(REPO_MAP_FILE, "repo_map.toon");
    }
}
