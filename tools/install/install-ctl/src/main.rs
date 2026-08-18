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

use clap::{
    Parser,
    Subcommand,
};
use cli::ViewerCmd;
use config::Config;
use registry::{
    Artifact,
    ArtifactKind,
    load_registry,
};
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
        /// Skip passing --force to `cargo install` for rust-binary artifacts.
        #[arg(long)]
        no_force: bool,
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
        Some(Command::Install {
            selection,
            no_force,
        }) => {
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
            let force = !no_force;
            if cli.dry_run {
                print_plan(&selected, force);
            } else {
                run_install(&selected, force);
            }
        },
        Some(Command::Start {
            server,
            foreground,
            extra,
        }) => run_viewer(|cfg, root| {
            commands::cmd_start(cfg, root, &server, foreground, extra)
        }),
        Some(Command::Prepare { server }) =>
            run_viewer(|cfg, root| commands::cmd_prepare(cfg, root, &server)),
        Some(Command::Viewer { command }) =>
            run_viewer(|cfg, root| dispatch_viewer(cfg, root, command)),
        None => {
            eprintln!(
                "error: no command given; try --list or `install <selection>`"
            );
            std::process::exit(1);
        },
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

fn dispatch_viewer(
    cfg: &Config,
    root: &Path,
    command: ViewerCmd,
) -> Result<(), String> {
    match command {
        ViewerCmd::List => commands::cmd_list(cfg),
        ViewerCmd::Status { name } =>
            commands::cmd_status(cfg, name.as_deref()),
        ViewerCmd::Build { name, kind } =>
            commands::cmd_build(cfg, root, &name, kind),
        ViewerCmd::Install { name, kind } =>
            commands::cmd_install(cfg, root, &name, kind),
        ViewerCmd::Start {
            server,
            foreground,
            extra,
        } => commands::cmd_start(cfg, root, &server, foreground, extra),
        ViewerCmd::Stop { server } => commands::cmd_stop(cfg, &server),
        ViewerCmd::Restart {
            server,
            foreground,
            extra,
        } => restart_server(cfg, root, &server, foreground, extra),
        ViewerCmd::Task { name } => commands::cmd_task(cfg, root, &name),
        ViewerCmd::Prepare { server } =>
            commands::cmd_prepare(cfg, root, &server),
        ViewerCmd::StaticDir { server } =>
            commands::cmd_static_dir(cfg, &server),
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
    let mut categories: Vec<&str> =
        artifacts.iter().map(|a| a.category.as_str()).collect();
    categories.sort();
    categories.dedup();

    for category in categories {
        println!("{category}:");
        for artifact in artifacts.iter().filter(|a| a.category == category) {
            println!("  {}", artifact.id);
        }
    }
}

fn print_plan(
    selected: &[Artifact],
    force: bool,
) {
    let repo_root = match registry::resolve_repo_root() {
        Ok(root) => root,
        Err(e) => fail(&e),
    };
    let (rust_artifacts, ext_artifacts) = split_by_kind(selected);

    if !rust_artifacts.is_empty() {
        let plan = match RustBuildPlan::new(&rust_artifacts, &repo_root) {
            Ok(p) => p,
            Err(e) => fail(&e),
        };
        println!("==> {}", plan.bins.join(", "));
        println!("    {}", plan.build_command_string());
        let force_note =
            if force { "always" } else { "only if the built binary is newer" };
        println!(
            "    copy target/install-tools/release/<bin> -> {} ({})",
            paths::disp(&cargo_bin_dir()),
            force_note
        );
    }

    for artifact in ext_artifacts {
        let script = artifact.npm_script.as_deref().unwrap_or("install:vsix");
        println!("==> {}", artifact.id);
        println!(
            "    (cd \"{}\" && npm ci && npm run {})",
            artifact.path, script
        );
    }
}

/// Split a selection into rust-binary and vscode-extension artifacts,
/// preserving relative order within each group.
fn split_by_kind(selected: &[Artifact]) -> (Vec<&Artifact>, Vec<&Artifact>) {
    let mut rust_artifacts = Vec::new();
    let mut ext_artifacts = Vec::new();
    for artifact in selected {
        match artifact.kind {
            ArtifactKind::RustBinary => rust_artifacts.push(artifact),
            ArtifactKind::VscodeExtension => ext_artifacts.push(artifact),
        }
    }
    (rust_artifacts, ext_artifacts)
}

/// Everything needed to build every selected rust-binary artifact in a
/// single `cargo build` call against the root workspace manifest.
///
/// Building through `cargo install --path <crate>` once per artifact (the
/// old approach) ignores the enclosing workspace: it re-resolves and
/// re-fetches each crate's dependency graph from scratch on every call,
/// bypassing the workspace `Cargo.lock` and `[patch]` table, and unifies
/// features only within that one crate — so siblings built with different
/// `--features` (e.g. `ticket` vs `ticket-mcp`) evict each other's cache
/// entries and rebuild shared dependencies repeatedly. Building every
/// requested binary together, against the workspace manifest, resolves
/// features once and reuses the workspace's own lockfile/patches.
struct RustBuildPlan {
    manifest_path: String,
    packages: Vec<String>,
    bins: Vec<String>,
    feature_flags: Vec<String>,
}

impl RustBuildPlan {
    fn new(
        artifacts: &[&Artifact],
        repo_root: &Path,
    ) -> Result<Self, String> {
        let manifest_path = repo_root
            .join("Cargo.toml")
            .to_string_lossy()
            .to_string();
        let mut packages: Vec<String> = Vec::new();
        let mut bins: Vec<String> = Vec::new();
        let mut feature_flags: Vec<String> = Vec::new();

        for artifact in artifacts {
            let pkg = package_name_for(repo_root, &artifact.path)?;
            if !packages.contains(&pkg) {
                packages.push(pkg.clone());
            }
            let bin = artifact.bin.clone().unwrap_or_else(|| artifact.id.clone());
            if !bins.contains(&bin) {
                bins.push(bin);
            }
            for feature in &artifact.features {
                let flag = format!("{pkg}/{feature}");
                if !feature_flags.contains(&flag) {
                    feature_flags.push(flag);
                }
            }
        }

        Ok(Self {
            manifest_path,
            packages,
            bins,
            feature_flags,
        })
    }

    fn build_args(&self) -> Vec<String> {
        let mut args = vec![
            "build".to_string(),
            "--manifest-path".to_string(),
            self.manifest_path.clone(),
            "--release".to_string(),
        ];
        for pkg in &self.packages {
            args.push("-p".to_string());
            args.push(pkg.clone());
        }
        for bin in &self.bins {
            args.push("--bin".to_string());
            args.push(bin.clone());
        }
        if !self.feature_flags.is_empty() {
            args.push("--features".to_string());
            args.push(self.feature_flags.join(","));
        }
        args
    }

    fn build_command_string(&self) -> String {
        std::iter::once("cargo".to_string())
            .chain(self.build_args())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Read `[package].name` out of `<repo_root>/<artifact_path>/Cargo.toml`.
fn package_name_for(
    repo_root: &Path,
    artifact_path: &str,
) -> Result<String, String> {
    let manifest_path = repo_root.join(artifact_path).join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("failed to read {}: {e}", manifest_path.display()))?;
    let value: toml::Value = text
        .parse()
        .map_err(|e| format!("failed to parse {}: {e}", manifest_path.display()))?;
    value
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(str::to_string)
        .ok_or_else(|| {
            format!("no [package].name in {}", manifest_path.display())
        })
}

/// Resolve where `cargo install` would place binaries, so plain builds can
/// be copied to the same location: `$CARGO_INSTALL_ROOT/bin`, else
/// `$CARGO_HOME/bin`, else `~/.cargo/bin`.
fn cargo_bin_dir() -> std::path::PathBuf {
    if let Ok(root) = std::env::var("CARGO_INSTALL_ROOT") {
        return std::path::PathBuf::from(root).join("bin");
    }
    if let Ok(home) = std::env::var("CARGO_HOME") {
        return std::path::PathBuf::from(home).join("bin");
    }
    dirs::home_dir()
        .map(|h| h.join(".cargo").join("bin"))
        .unwrap_or_else(|| std::path::PathBuf::from(".cargo/bin"))
}

fn run_install(
    selected: &[Artifact],
    force: bool,
) {
    let repo_root = match registry::resolve_repo_root() {
        Ok(root) => root,
        Err(e) => fail(&e),
    };

    // Isolate build artifacts from the dev target dir so a running debug
    // binary's file lock doesn't block the build itself.
    let target_dir = repo_root.join("target/install-tools");
    unsafe {
        std::env::set_var("CARGO_TARGET_DIR", &target_dir);
    }

    let (rust_artifacts, ext_artifacts) = split_by_kind(selected);

    if !rust_artifacts.is_empty() {
        install_rust_binaries(&rust_artifacts, &repo_root, &target_dir, force);
    }

    for artifact in ext_artifacts {
        install_vscode_extension(artifact, &repo_root);
    }
}

fn install_rust_binaries(
    artifacts: &[&Artifact],
    repo_root: &Path,
    target_dir: &Path,
    force: bool,
) {
    let plan = match RustBuildPlan::new(artifacts, repo_root) {
        Ok(p) => p,
        Err(e) => fail(&e),
    };

    println!("==> {}", plan.bins.join(", "));

    // Stop every requested binary up front: a locked exe on Windows blocks
    // both the link step and the copy that follows it.
    let mut stopped_by_bin: Vec<(String, Vec<Pid>)> = Vec::new();
    for bin in &plan.bins {
        let running = process::pids_by_image_name(bin);
        let mut stopped: Vec<Pid> = Vec::new();
        for pid in running {
            process::print_process_info(pid, bin);
            if process::kill_process(pid, bin) {
                stopped.push(pid);
            } else {
                eprintln!(
                    "warning: [{bin}] failed to stop PID {} before install",
                    pid.as_u32()
                );
            }
        }
        stopped_by_bin.push((bin.clone(), stopped));
    }

    let build_args = plan.build_args();
    let arg_refs: Vec<&str> = build_args.iter().map(String::as_str).collect();
    let label = plan.bins.join(", ");
    if let Err(e) = shell::run_cmd_args("cargo", &arg_refs, repo_root, &label) {
        fail(&e);
    }

    let bin_dir = cargo_bin_dir();
    if let Err(e) = std::fs::create_dir_all(&bin_dir) {
        fail(&format!("failed to create {}: {e}", bin_dir.display()));
    }
    let exe_suffix = std::env::consts::EXE_SUFFIX;
    let profile_dir = target_dir.join("release");

    for (bin, stopped) in stopped_by_bin {
        let built = profile_dir.join(format!("{bin}{exe_suffix}"));
        let installed = bin_dir.join(format!("{bin}{exe_suffix}"));
        match copy_binary(&built, &installed, force) {
            Ok(true) => println!(
                "    installed {bin} -> {}",
                paths::disp(&installed)
            ),
            Ok(false) => println!("    {bin} already up to date"),
            Err(e) => fail(&e),
        }
        if stopped.is_empty() {
            println!("    no running instance of {bin} was found");
        } else {
            let pids = stopped
                .iter()
                .map(|p| p.as_u32().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("    stopped PID(s) for {bin}: {pids}");
        }
    }
}

/// Copy `src` to `dst` when it changed, or when `force` requires an
/// unconditional overwrite. Returns `Ok(true)` when `dst` was stale (so
/// callers report "installed") vs `Ok(false)` when it was already
/// up to date (nothing new was compiled) — computed the same way
/// regardless of `force`, so the "up to date" message stays accurate even
/// when `force` still performs the (cheap) copy.
fn copy_binary(
    src: &Path,
    dst: &Path,
    force: bool,
) -> Result<bool, String> {
    let src_mtime = std::fs::metadata(src)
        .and_then(|m| m.modified())
        .map_err(|e| format!("failed to stat {}: {e}", src.display()))?;
    let up_to_date = match std::fs::metadata(dst).and_then(|m| m.modified()) {
        Ok(dst_mtime) => dst_mtime >= src_mtime,
        Err(_) => false,
    };

    if up_to_date && !force {
        return Ok(false);
    }

    std::fs::copy(src, dst).map_err(|e| {
        format!("failed to copy {} to {}: {e}", src.display(), dst.display())
    })?;
    Ok(!up_to_date)
}

fn install_vscode_extension(
    artifact: &Artifact,
    repo_root: &Path,
) {
    println!("==> {}", artifact.id);
    let script = artifact.npm_script.as_deref().unwrap_or("install:vsix");
    let ext_dir = repo_root.join(&artifact.path);

    if !ext_dir.join("node_modules").is_dir()
        && let Err(e) =
            shell::run_cmd_args("npm", &["ci"], &ext_dir, &artifact.id)
    {
        fail(&e);
    }

    if let Err(e) =
        shell::run_cmd_args("npm", &["run", script], &ext_dir, &artifact.id)
    {
        fail(&e);
    }
}

fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}
