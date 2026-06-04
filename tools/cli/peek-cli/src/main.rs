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
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use clap::Parser;

/// peek — token-bounded file inspection.
///
/// Reads a targeted slice of a file. Bounded reads are the default interaction
/// pattern; use `--all` only when the entire file is genuinely needed.
#[derive(Debug, Parser)]
#[command(name = "peek", version, about)]
struct Args {
    /// Path to the file to inspect.
    file: PathBuf,

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

    // --skeleton: emit only structural signatures.
    if args.skeleton {
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
    let mut pending_open = false; // we emitted "{ ... }" but haven't closed yet

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
                pending_open = false;
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
                    pending_open = true;
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
