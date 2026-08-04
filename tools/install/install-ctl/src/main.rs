mod cli;
mod commands;
mod config;
mod logging;
mod paths;
mod process;
mod registry;
mod selection;
mod shell;

use std::{
    path::Path,
    time::Duration,
};

use clap::{Parser, Subcommand};
use cli::ViewerCmd;
use config::Config;
use registry::{Artifact, ArtifactKind, load_registry};
use selection::resolve_selection;
use sysinfo::Pid;

#[derive(Parser)]
#[command(
    name = "install-ctl",
    about = "Install workspace tool binaries and extensions, and manage the viewer lifecycle"
)]
struct Cli {
    /// Print supported artifacts grouped by category and exit.
    #[arg(long)]
    list: bool,

    /// Print the planned actions without performing them.
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Install one or more artifacts by id, category, or "all".
    Install {
        #[arg(required = true)]
        selection: Vec<String>,
    },
    /// Start a viewer server (alias for `viewer start`; kept top-level so
    /// existing `install-ctl start <viewer>` invocations, e.g. from
    /// .vscode/tasks.json, keep working verbatim).
    Start {
        server: String,
        #[arg(long, alias = "fg", short = 'f')]
        foreground: bool,
        #[arg(last = true)]
        extra: Vec<String>,
    },
    /// Build + install the frontend linked to a viewer server (alias for
    /// `viewer prepare`; kept top-level for the same reason as `start`).
    Prepare { server: String },
    /// Viewer lifecycle surface (list/status/build/install/start/stop/restart/task/prepare/static-dir),
    /// nested under `viewer` so it does not collide with the top-level
    /// artifact-registry `--list`/`install`.
    Viewer {
        #[command(subcommand)]
        command: ViewerCmd,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        match load_registry() {
            Ok(reg) => print_list(&reg.artifacts),
            Err(e) => fail(&e),
        }
        return;
    }

    match cli.command {
        Some(Command::Install { selection }) => {
            let reg = match load_registry() {
                Ok(reg) => reg,
                Err(e) => fail(&e),
            };
            let selected = match resolve_selection(&reg.artifacts, &selection) {
                Ok(s) => s,
                Err(e) => fail(&e),
            };
            if selected.is_empty() {
                fail("selection matched no artifacts");
            }
            if cli.dry_run {
                print_plan(&selected);
            } else {
                run_install(&selected);
            }
        }
        Some(Command::Start { server, foreground, extra }) => run_viewer(|cfg, root| {
            commands::cmd_start(cfg, root, &server, foreground, extra)
        }),
        Some(Command::Prepare { server }) => {
            run_viewer(|cfg, root| commands::cmd_prepare(cfg, root, &server))
        }
        Some(Command::Viewer { command }) => {
            run_viewer(|cfg, root| dispatch_viewer(cfg, root, command))
        }
        None => {
            eprintln!("error: no command given; try --list or `install <selection>`");
            std::process::exit(1);
        }
    }
}

/// Load `viewer-ctl.toml` from the same repo root the artifact registry
/// resolves, then run `action` against it, failing the process on error.
fn run_viewer<F>(action: F)
where
    F: FnOnce(&Config, &Path) -> Result<(), String>,
{
    let root = match registry::resolve_repo_root() {
        Ok(root) => root,
        Err(e) => fail(&e),
    };
    let cfg = match Config::load(&root) {
        Ok(c) => c,
        Err(e) => fail(&e),
    };
    if let Err(e) = action(&cfg, &root) {
        fail(&e);
    }
}

fn dispatch_viewer(cfg: &Config, root: &Path, command: ViewerCmd) -> Result<(), String> {
    match command {
        ViewerCmd::List => commands::cmd_list(cfg),
        ViewerCmd::Status { name } => commands::cmd_status(cfg, name.as_deref()),
        ViewerCmd::Build { name, kind } => commands::cmd_build(cfg, root, &name, kind),
        ViewerCmd::Install { name, kind } => commands::cmd_install(cfg, root, &name, kind),
        ViewerCmd::Start { server, foreground, extra } => {
            commands::cmd_start(cfg, root, &server, foreground, extra)
        }
        ViewerCmd::Stop { server } => commands::cmd_stop(cfg, &server),
        ViewerCmd::Restart { server, foreground, extra } => {
            restart_server(cfg, root, &server, foreground, extra)
        }
        ViewerCmd::Task { name } => commands::cmd_task(cfg, root, &name),
        ViewerCmd::Prepare { server } => commands::cmd_prepare(cfg, root, &server),
        ViewerCmd::StaticDir { server } => commands::cmd_static_dir(cfg, &server),
    }
}

fn restart_server(
    cfg: &Config,
    root: &Path,
    server: &str,
    foreground: bool,
    extra: Vec<String>,
) -> Result<(), String> {
    commands::cmd_stop(cfg, server)?;
    std::thread::sleep(Duration::from_millis(500));
    commands::cmd_start(cfg, root, server, foreground, extra)
}

fn print_list(artifacts: &[Artifact]) {
    let mut categories: Vec<&str> = artifacts.iter().map(|a| a.category.as_str()).collect();
    categories.sort();
    categories.dedup();

    for category in categories {
        println!("{category}:");
        for artifact in artifacts.iter().filter(|a| a.category == category) {
            println!("  {}", artifact.id);
        }
    }
}

fn print_plan(selected: &[Artifact]) {
    for artifact in selected {
        match artifact.kind {
            ArtifactKind::RustBinary => {
                let bin = artifact.bin.as_deref().unwrap_or(&artifact.id);
                println!("==> {}", artifact.id);
                println!(
                    "    cargo install --path \"{}\" --bin {} --force",
                    artifact.path, bin
                );
            }
            ArtifactKind::VscodeExtension => {
                let script = artifact.npm_script.as_deref().unwrap_or("install:vsix");
                println!("==> {}", artifact.id);
                println!("    (cd \"{}\" && npm ci && npm run {})", artifact.path, script);
            }
        }
    }
}

fn run_install(selected: &[Artifact]) {
    let repo_root = match registry::resolve_repo_root() {
        Ok(root) => root,
        Err(e) => fail(&e),
    };

    // Match install-tools.sh: isolate build artifacts from the dev target dir
    // so a running debug binary's file lock doesn't block the build itself.
    unsafe {
        std::env::set_var("CARGO_TARGET_DIR", repo_root.join("target/install-tools"));
    }

    for artifact in selected {
        match artifact.kind {
            ArtifactKind::RustBinary => install_rust_binary(artifact, &repo_root),
            ArtifactKind::VscodeExtension => install_vscode_extension(artifact, &repo_root),
        }
    }
}

fn install_rust_binary(artifact: &Artifact, repo_root: &Path) {
    let bin = artifact.bin.as_deref().unwrap_or(&artifact.id);
    println!("==> {}", artifact.id);

    let running = process::pids_by_image_name(bin);
    let mut stopped: Vec<Pid> = Vec::new();
    for pid in running {
        process::print_process_info(pid, &artifact.id);
        if process::kill_process(pid, &artifact.id) {
            stopped.push(pid);
        } else {
            eprintln!(
                "warning: [{}] failed to stop PID {} before install",
                artifact.id,
                pid.as_u32()
            );
        }
    }

    let full_path = repo_root.join(&artifact.path);
    let full_path = full_path.to_string_lossy().to_string();
    let args = ["install", "--path", full_path.as_str(), "--bin", bin, "--force"];
    if let Err(e) = shell::run_cmd_args("cargo", &args, repo_root, &artifact.id) {
        fail(&e);
    }

    if stopped.is_empty() {
        println!("    no running instance of {bin} was found");
    } else {
        let pids = stopped
            .iter()
            .map(|p| p.as_u32().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("    stopped PID(s): {pids}");
    }
}

fn install_vscode_extension(artifact: &Artifact, repo_root: &Path) {
    println!("==> {}", artifact.id);
    let script = artifact.npm_script.as_deref().unwrap_or("install:vsix");
    let ext_dir = repo_root.join(&artifact.path);

    if !ext_dir.join("node_modules").is_dir()
        && let Err(e) = shell::run_cmd_args("npm", &["ci"], &ext_dir, &artifact.id)
    {
        fail(&e);
    }

    if let Err(e) = shell::run_cmd_args("npm", &["run", script], &ext_dir, &artifact.id) {
        fail(&e);
    }
}

fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}
