use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod analyzer;
mod function_collector;
mod tracing_collector;

use analyzer::analyze_file;

#[derive(Parser, Debug)]
#[command(name = "tracing-analyzer")]
#[command(about = "Analyze debug/trace/info statement coverage in Rust source files")]
struct Args {
    /// Path to the crate or directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format: text, json, or csv
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Sort by: name, count, density
    #[arg(short, long, default_value = "density")]
    sort: String,

    /// Show only functions with zero tracing statements
    #[arg(long)]
    zero_only: bool,

    /// Minimum function line count to include
    #[arg(long, default_value = "3")]
    min_lines: usize,
}

fn main() {
    let args = Args::parse();

    let source_files = collect_source_files(&args.path);
    println!("Found {} source files to analyze", source_files.len());

    let mut all_functions = Vec::new();

    for file_path in &source_files {
        match analyze_file(file_path) {
            Ok(functions) => {
                all_functions.extend(functions);
            }
            Err(e) => {
                eprintln!("Error analyzing {:?}: {}", file_path, e);
            }
        }
    }

    // Filter by minimum lines
    all_functions.retain(|f| f.line_count() >= args.min_lines);

    // Filter by zero-only if requested
    if args.zero_only {
        all_functions.retain(|f| f.tracing_count == 0);
    }

    // Sort
    match args.sort.as_str() {
        "name" => all_functions.sort_by(|a, b| a.full_path().cmp(&b.full_path())),
        "count" => all_functions.sort_by(|a, b| b.tracing_count.cmp(&a.tracing_count)),
        "density" | _ => all_functions.sort_by(|a, b| {
            b.density()
                .partial_cmp(&a.density())
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
    }

    // Output
    match args.format.as_str() {
        "json" => output_json(&all_functions),
        "csv" => output_csv(&all_functions),
        "text" | _ => output_text(&all_functions),
    }

    // Summary statistics
    print_summary(&all_functions);
}

fn collect_source_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Skip target, hidden directories, and deps
            !name.starts_with('.') && name != "target" && name != "deps"
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.to_path_buf());
        }
    }

    files
}

fn output_text(functions: &[analyzer::FunctionInfo]) {
    println!("\n{:-<100}", "");
    println!(
        "{:<60} {:>8} {:>8} {:>8} {:>10}",
        "Function", "Start", "End", "Count", "Density"
    );
    println!("{:-<100}", "");

    for func in functions {
        println!(
            "{:<60} {:>8} {:>8} {:>8} {:>10.2}",
            truncate(&func.full_path(), 60),
            func.start_line,
            func.end_line,
            func.tracing_count,
            func.density()
        );
    }
}

fn output_json(functions: &[analyzer::FunctionInfo]) {
    println!("{}", serde_json::to_string_pretty(functions).unwrap());
}

fn output_csv(functions: &[analyzer::FunctionInfo]) {
    println!("file,module_path,name,start_line,end_line,tracing_count,density");
    for func in functions {
        println!(
            "{},{},{},{},{},{},{:.4}",
            func.file.display(),
            func.module_path,
            func.name,
            func.start_line,
            func.end_line,
            func.tracing_count,
            func.density()
        );
    }
}

fn print_summary(functions: &[analyzer::FunctionInfo]) {
    let total_functions = functions.len();
    let total_tracing: usize = functions.iter().map(|f| f.tracing_count).sum();
    let zero_count = functions.iter().filter(|f| f.tracing_count == 0).count();
    let total_lines: usize = functions.iter().map(|f| f.line_count()).sum();

    let avg_density = if total_lines > 0 {
        (total_tracing as f64) / (total_lines as f64) * 100.0
    } else {
        0.0
    };

    println!("\n{:=<60}", "");
    println!("SUMMARY");
    println!("{:=<60}", "");
    println!("Total functions analyzed: {}", total_functions);
    println!("Total tracing statements: {}", total_tracing);
    println!("Total function lines:     {}", total_lines);
    println!(
        "Functions with 0 traces:  {} ({:.1}%)",
        zero_count,
        if total_functions > 0 {
            (zero_count as f64) / (total_functions as f64) * 100.0
        } else {
            0.0
        }
    );
    println!("Average density:          {:.2}%", avg_density);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}
