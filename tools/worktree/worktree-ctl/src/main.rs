use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
    process::Command as ProcessCommand,
};

use clap::{
    Parser,
    Subcommand,
};
use git2::{
    BranchType,
    Oid,
    Repository,
};
use session_worktree_provision::{
    ReclaimEligibility,
    ReclaimRejectionReason,
    SessionStoreActivity,
    WorktreeGit,
    evaluate_reclaim_candidate,
    policy::ProvisionPolicy,
};

const WORKTREE_PATH_OUTPUT_PREFIX: &str = "WORKTREE_PATH=";
const FINISH_READY_TO_MERGE_MARKER: &str = "ready-to-merge";
const DIRTY_MAIN_UNCOMMITTED_CHANGES_MESSAGE: &str = "uncommitted changes";
const PRESERVE_MAIN_CHANGES_HINT: &str = "preserve-main-changes";
#[cfg(test)]
const WORKTREE_PATH_TEMPLATE: &str = ".worktrees/<full-session-uuid>/<slug>";
#[cfg(test)]
const BRANCH_TEMPLATE: &str = "agent/<full-session-uuid>/<slug>";

#[derive(Debug, Parser)]
#[command(
    name = "worktree-ctl",
    about = "Manage local Git worktree lifecycles"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
enum Command {
    New {
        session_uuid: String,
        slug: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        preserve_main_changes: bool,
    },
    Bootstrap {
        session_uuid: String,
        slug: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        preserve_main_changes: bool,
    },
    List {
        #[arg(long)]
        dry_run: bool,
    },
    Rebase {
        name: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        auto_commit: bool,
    },
    Merge {
        name: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        auto_commit: bool,
    },
    Sync {
        name: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        auto_commit: bool,
    },
    Remove {
        name: String,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        dry_run: bool,
    },
    Rename {
        source_name: String,
        target_name: String,
        #[arg(long)]
        dry_run: bool,
    },
    Finish {
        name: String,
        #[arg(long)]
        dry_run: bool,
    },
    Doctor {
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = dispatch(cli.command) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn dispatch(command: Command) -> Result<(), String> {
    match command {
        Command::New {
            session_uuid,
            slug,
            dry_run,
            preserve_main_changes,
        } => handle_new(&session_uuid, &slug, dry_run, preserve_main_changes),
        Command::Bootstrap {
            session_uuid,
            slug,
            dry_run,
            preserve_main_changes,
        } => handle_bootstrap(
            &session_uuid,
            &slug,
            dry_run,
            preserve_main_changes,
        ),
        Command::List { dry_run } => handle_list(dry_run),
        Command::Rebase {
            name,
            dry_run,
            auto_commit,
        } => handle_rebase(&name, dry_run, auto_commit),
        Command::Merge {
            name,
            dry_run,
            auto_commit,
        } => handle_merge(&name, dry_run, auto_commit),
        Command::Sync {
            name,
            dry_run,
            auto_commit,
        } => handle_sync(&name, dry_run, auto_commit),
        Command::Remove {
            name,
            force,
            dry_run,
        } => handle_remove(&name, force, dry_run),
        Command::Rename {
            source_name,
            target_name,
            dry_run,
        } => handle_rename(&source_name, &target_name, dry_run),
        Command::Finish { name, dry_run } => handle_finish(&name, dry_run),
        Command::Doctor { dry_run } => handle_doctor(dry_run),
    }
}

#[derive(Default)]
struct LifecyclePlan {
    actions: Vec<String>,
}

impl LifecyclePlan {
    fn add(
        &mut self,
        action: impl Into<String>,
    ) {
        self.actions.push(action.into());
    }

    fn emit(&self) {
        for action in &self.actions {
            println!("[dry-run] {action}");
        }
    }
}

fn handle_new(
    session_uuid: &str,
    slug: &str,
    dry_run: bool,
    preserve_main_changes: bool,
) -> Result<(), String> {
    validate_full_session_uuid(session_uuid)?;
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(&main_checkout).map_err(|error| error.to_string())?;
    let relative_path = Path::new(session_uuid).join(slug);
    let branch = format!("agent/{session_uuid}/{slug}");
    let worktree_path =
        git.main_checkout().join(".worktrees").join(&relative_path);
    let worktrees = git.list_worktrees().map_err(|error| error.to_string())?;

    if let Some(worktree) = worktrees
        .iter()
        .find(|worktree| worktree.path == worktree_path)
    {
        println!("{WORKTREE_PATH_OUTPUT_PREFIX}{}", worktree.path.display());
        return Ok(());
    }

    let nested_slugs =
        nested_slug_directories(git.main_checkout(), session_uuid)?;
    if !nested_slugs.is_empty() {
        return Err(format!(
            "ambiguous session worktree for {session_uuid}: nested slug directories already exist: {}; exactly one active slug is allowed",
            nested_slugs.join(", ")
        ));
    }

    let dirty_paths = git
        .dirty_paths(git.main_checkout())
        .map_err(|error| error.to_string())?
        .into_iter()
        .filter(|path| !path.path.starts_with(".worktrees"))
        .collect::<Vec<_>>();
    if !dirty_paths.is_empty() && !preserve_main_changes {
        let paths = dirty_paths
            .iter()
            .map(|path| path.path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "{DIRTY_MAIN_UNCOMMITTED_CHANGES_MESSAGE} in main checkout: {paths}; pass --{PRESERVE_MAIN_CHANGES_HINT} to stash them"
        ));
    }

    let mut plan = LifecyclePlan::default();
    if !dirty_paths.is_empty() {
        let paths = dirty_paths
            .iter()
            .map(|path| path.path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        plan.add(format!(
            "stash main-checkout changes ({paths}) with {PRESERVE_MAIN_CHANGES_HINT}"
        ));
    }
    plan.add(format!(
        "create {} from local main on branch {branch}",
        worktree_path.display()
    ));
    for submodule in git.submodule_paths().map_err(|error| error.to_string())? {
        plan.add(format!(
            "populate {} from its recorded local gitlink",
            worktree_path.join(submodule).display()
        ));
    }

    if dry_run {
        plan.emit();
        return Ok(());
    }

    if !dirty_paths.is_empty() {
        let paths = dirty_paths
            .iter()
            .map(|path| path.path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("preserving main-checkout changes: {paths}");
        git.stash_push(PRESERVE_MAIN_CHANGES_HINT)
            .map_err(|error| error.to_string())?;
    }
    let worktree = git
        .create_worktree_at(&relative_path, &branch, "main")
        .map_err(|error| error.to_string())?;
    println!("{WORKTREE_PATH_OUTPUT_PREFIX}{}", worktree.path.display());
    Ok(())
}

fn handle_bootstrap(
    session_uuid: &str,
    slug: &str,
    dry_run: bool,
    preserve_main_changes: bool,
) -> Result<(), String> {
    handle_new(session_uuid, slug, dry_run, preserve_main_changes)?;

    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(main_checkout).map_err(|error| error.to_string())?;
    let worktree_path = git
        .main_checkout()
        .join(".worktrees")
        .join(session_uuid)
        .join(slug);

    if dry_run {
        println!(
            "[dry-run] initialize repository stores and Copilot surfaces in {} with init.sh",
            worktree_path.display()
        );
        return Ok(());
    }

    let init_script = worktree_path.join("init.sh");
    if !init_script.is_file() {
        return Err(format!(
            "worktree initializer is missing at {}; repair the worktree and rerun bootstrap",
            init_script.display()
        ));
    }

    let status = ProcessCommand::new("bash")
        .arg("init.sh")
        .current_dir(&worktree_path)
        .status()
        .map_err(|error| {
            format!(
                "could not run {}: {error}",
                init_script.display()
            )
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "worktree initializer failed in {}; repair the worktree and rerun bootstrap",
            worktree_path.display()
        ))
    }
}

fn handle_list(_dry_run: bool) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(&main_checkout).map_err(|error| error.to_string())?;
    let activity = SessionStoreActivity::with_default_staleness(
        git.main_checkout().join(".session"),
    );
    let policy = ProvisionPolicy::default();
    let registered = git.list_worktrees().map_err(|error| error.to_string())?;

    for worktree in &registered {
        let submodules = submodule_status(&git, &worktree.path)?;
        let lifecycle = lifecycle_status(&git, &activity, worktree, &policy)?;
        println!(
            "path={} branch={} submodules={} lifecycle={}",
            worktree.path.display(),
            worktree.branch.as_deref().unwrap_or("detached"),
            submodules,
            lifecycle
        );
    }

    let worktree_root = git.main_checkout().join(".worktrees");
    if worktree_root.is_dir() {
        for entry in std::fs::read_dir(&worktree_root)
            .map_err(|error| error.to_string())?
        {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            if path.is_dir()
                && !registered.iter().any(|worktree| {
                    worktree.path == path || worktree.path.starts_with(&path)
                })
            {
                println!(
                    "path={} lifecycle=unregistered-debris",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn handle_rebase(
    name: &str,
    dry_run: bool,
    auto_commit: bool,
) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(main_checkout).map_err(|error| error.to_string())?;
    let worktree = find_worktree(&git, name)?;
    let branch = worktree.branch.as_deref().ok_or_else(|| {
        format!("worktree {name} is detached and cannot be rebased")
    })?;
    let mut plan = LifecyclePlan::default();

    for submodule in git.submodule_paths().map_err(|error| error.to_string())? {
        let nested_worktree = worktree.path.join(&submodule);
        if !repository_has_branch(&nested_worktree, branch)? {
            plan.add(format!(
                "skip {} because branch {branch} does not exist",
                nested_worktree.display()
            ));
            if !dry_run {
                println!(
                    "skip {submodule} because branch {branch} does not exist"
                );
            }
            continue;
        }
        plan.add(format!(
            "checkout {branch} and rebase {} onto its local main",
            nested_worktree.display()
        ));
        let stashed = guard_dirty_tree(
            &git,
            &nested_worktree,
            &format!("submodule {submodule}"),
            auto_commit,
            dry_run,
            &mut plan,
        )?;
        if dry_run {
            continue;
        }
        let rebase_result =
            checkout_and_rebase(&nested_worktree, branch).map_err(|error| {
                format!(
                    "submodule {submodule} branch {branch} could not rebase onto local main: {error}; resolve the conflict in {} and continue or abort the rebase",
                    nested_worktree.display()
                )
            });
        let restore_result = restore_dirty_tree(&nested_worktree, stashed);
        combine_results(rebase_result, restore_result)?;
        commit_rebased_gitlink(&worktree.path, &submodule)?;
    }

    plan.add(format!(
        "rebase {} onto local main",
        worktree.path.display()
    ));
    let stashed = guard_dirty_tree(
        &git,
        &worktree.path,
        "worktree",
        auto_commit,
        dry_run,
        &mut plan,
    )?;
    if dry_run {
        plan.emit();
        return Ok(());
    }
    let rebase_result = rebase_onto_local_main(&worktree.path);
    let restore_result = restore_dirty_tree(&worktree.path, stashed);
    combine_results(rebase_result, restore_result)
}

fn handle_merge(
    name: &str,
    dry_run: bool,
    auto_commit: bool,
) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(&main_checkout).map_err(|error| error.to_string())?;
    let worktree = find_worktree(&git, name)?;
    let branch = worktree.branch.as_deref().ok_or_else(|| {
        format!("worktree {name} is detached and cannot be merged")
    })?;
    let mut plan = LifecyclePlan::default();

    let preflight = verify_gitlink_containment(git.main_checkout())?;
    let (fixable, blocking) =
        partition_gitlink_statuses(git.main_checkout(), preflight)?;
    reject_gitlink_violations(&blocking)?;
    for status in &fixable {
        plan.add(format!(
            "auto-fix gitlink: fast-forward submodule {} local main to recorded commit {} (only one possible resolution)",
            status.submodule_path, status.recorded_sha
        ));
    }
    if !dry_run {
        for status in &fixable {
            let submodule_path =
                git.main_checkout().join(&status.submodule_path);
            run_git(&submodule_path, ["checkout", "main"])?;
            run_git(&submodule_path, [
                "merge",
                "--ff-only",
                &status.recorded_sha.to_string(),
            ])?;
            println!(
                "auto-fixed gitlink: {} local main fast-forwarded to {}",
                status.submodule_path, status.recorded_sha
            );
        }
    }

    for submodule in git.submodule_paths().map_err(|error| error.to_string())? {
        let nested_worktree = worktree.path.join(&submodule);
        if !repository_has_branch(&nested_worktree, branch)? {
            plan.add(format!(
                "skip {} because branch {branch} does not exist",
                submodule
            ));
            continue;
        }
        reject_unmerged_submodule_branch(
            &git.main_checkout().join(&submodule),
            branch,
            &submodule,
        )?;
        let main_submodule = git.main_checkout().join(&submodule);
        plan.add(format!(
            "fast-forward {} local main from nested branch {branch}",
            main_submodule.display()
        ));
        let stashed = guard_dirty_tree(
            &git,
            &main_submodule,
            &format!("submodule {submodule}"),
            auto_commit,
            dry_run,
            &mut plan,
        )?;
        if dry_run {
            continue;
        }
        let merge_result = merge_ff_only(&main_submodule, branch);
        let restore_result = restore_dirty_tree(&main_submodule, stashed);
        combine_results(merge_result, restore_result)?;
    }
    plan.add(format!(
        "fast-forward superproject local main from {branch}"
    ));
    let stashed = guard_dirty_tree(
        &git,
        git.main_checkout(),
        "superproject",
        auto_commit,
        dry_run,
        &mut plan,
    )?;
    if dry_run {
        plan.emit();
        return Ok(());
    }

    let merge_result = merge_ff_only(git.main_checkout(), branch);
    let restore_result = restore_dirty_tree(git.main_checkout(), stashed);
    combine_results(merge_result, restore_result)?;
    let postflight = verify_gitlink_containment(git.main_checkout())?;
    reject_gitlink_violations(&postflight)
}

// Rebase then merge behind one command; a rebase conflict stops before any
// merge so the user resolves it and reruns sync to finish and merge to main.
fn handle_sync(
    name: &str,
    dry_run: bool,
    auto_commit: bool,
) -> Result<(), String> {
    handle_rebase(name, dry_run, auto_commit)?;
    handle_merge(name, dry_run, auto_commit)
}

const AUTOSTASH_MESSAGE: &str = "worktree-ctl autostash";
const AUTO_COMMIT_MESSAGE: &str = "worktree-ctl auto-commit before sync";

// Guards a mutating rebase/merge step against an unclean working tree.
// Default: stash (including untracked files) and let the caller restore it
// with restore_dirty_tree once the mutation is done. With auto_commit: commit
// the dirty state instead, so it rides along with the rebase/merge and there
// is nothing left to restore. Returns whether a stash was created.
fn guard_dirty_tree(
    git: &WorktreeGit,
    path: &Path,
    label: &str,
    auto_commit: bool,
    dry_run: bool,
    plan: &mut LifecyclePlan,
) -> Result<bool, String> {
    if !git.is_dirty(path).map_err(|error| error.to_string())? {
        return Ok(false);
    }
    if auto_commit {
        plan.add(format!(
            "auto-commit uncommitted changes in {label} before mutating"
        ));
        if dry_run {
            return Ok(false);
        }
        run_git(path, ["add", "-A"])?;
        run_git(path, ["commit", "-m", AUTO_COMMIT_MESSAGE])?;
        println!("auto-committed uncommitted changes in {label}");
        return Ok(false);
    }
    plan.add(format!(
        "stash uncommitted changes in {label} before mutating (restored afterward)"
    ));
    if dry_run {
        return Ok(false);
    }
    run_git(path, [
        "stash",
        "push",
        "--include-untracked",
        "-m",
        AUTOSTASH_MESSAGE,
    ])?;
    println!("stashed uncommitted changes in {label} (restored afterward)");
    Ok(true)
}

fn restore_dirty_tree(
    path: &Path,
    stashed: bool,
) -> Result<(), String> {
    if !stashed {
        return Ok(());
    }
    run_git(path, ["stash", "pop"]).map_err(|error| {
        format!(
            "changes were stashed in {} before this operation but could not be restored automatically ({error}); run `git -C {} stash list` to recover them",
            path.display(),
            path.display()
        )
    })
}

fn combine_results(
    primary: Result<(), String>,
    secondary: Result<(), String>,
) -> Result<(), String> {
    match (primary, secondary) {
        (Err(primary), Err(secondary)) =>
            Err(format!("{primary}; additionally, {secondary}")),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(())) => Ok(()),
    }
}



fn handle_remove(
    name: &str,
    force: bool,
    dry_run: bool,
) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(main_checkout).map_err(|error| error.to_string())?;
    let worktree = find_worktree(&git, name)?;
    let dirty_paths = git
        .dirty_paths(&worktree.path)
        .map_err(|error| error.to_string())?;
    if !force && !dirty_paths.is_empty() {
        let paths = dirty_paths
            .iter()
            .map(|path| path.path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "worktree {name} has uncommitted changes: {paths}"
        ));
    }

    let mut plan = LifecyclePlan::default();
    plan.add(format!("remove {} with force", worktree.path.display()));
    plan.add("prune removed worktree registrations");
    if nested_worktree_parent(git.main_checkout(), &worktree.path).is_some() {
        plan.add(format!(
            "remove the session directory if {} is empty",
            worktree
                .path
                .parent()
                .expect("worktree has a parent")
                .display()
        ));
    }
    if dry_run {
        plan.emit();
        return Ok(());
    }

    git.worktree_remove_force(&worktree.path)
        .map_err(|error| error.to_string())?;
    git.worktree_prune().map_err(|error| error.to_string())?;
    remove_empty_nested_parent(git.main_checkout(), &worktree.path)
}

fn handle_rename(
    source_name: &str,
    target_name: &str,
    dry_run: bool,
) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(main_checkout).map_err(|error| error.to_string())?;
    let source = find_worktree(&git, source_name)?;
    let source_relative = worktree_relative_path(&git, &source)?;
    let target_relative = rename_target_path(&source_relative, target_name)?;
    let target_path = git
        .main_checkout()
        .join(".worktrees")
        .join(&target_relative);
    let target_branch = branch_for_relative_path(&target_relative)?;
    let mut plan = LifecyclePlan::default();
    plan.add(format!(
        "move {} to {}, repair Git metadata, and rename its branch to {target_branch}",
        source.path.display(),
        target_path.display()
    ));
    if dry_run {
        plan.emit();
        return Ok(());
    }

    git.rename_worktree(&source.name, &target_relative, &target_branch)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn handle_finish(
    name: &str,
    dry_run: bool,
) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(main_checkout).map_err(|error| error.to_string())?;
    let worktree = find_worktree(&git, name)?;
    let mut plan = LifecyclePlan::default();
    plan.add(format!(
        "rebase {} onto local main",
        worktree.path.display()
    ));
    plan.add(format!("remove {} with force", worktree.path.display()));
    plan.add("prune removed worktree registrations");
    plan.add(FINISH_READY_TO_MERGE_MARKER);
    if dry_run {
        plan.emit();
        return Ok(());
    }

    rebase_onto_local_main(&worktree.path)?;
    git.worktree_remove_force(&worktree.path)
        .map_err(|error| error.to_string())?;
    git.worktree_prune().map_err(|error| error.to_string())?;
    remove_empty_nested_parent(git.main_checkout(), &worktree.path)?;
    println!("{FINISH_READY_TO_MERGE_MARKER}");
    Ok(())
}

fn find_worktree(
    git: &WorktreeGit,
    name: &str,
) -> Result<session_worktree_provision::WorktreeRef, String> {
    let worktrees = git.list_worktrees().map_err(|error| error.to_string())?;
    if name.contains('/') {
        let relative_path = nested_relative_path(name)?;
        let path = git.main_checkout().join(".worktrees").join(relative_path);
        return worktrees
            .into_iter()
            .find(|worktree| worktree.path == path)
            .ok_or_else(|| format!("worktree '{name}' was not found"));
    }

    let mut matches = worktrees
        .into_iter()
        .filter(|worktree| worktree.name == name);
    let Some(worktree) = matches.next() else {
        return Err(format!("worktree '{name}' was not found"));
    };
    if matches.next().is_some() {
        return Err(format!(
            "ambiguous worktree name '{name}'; use <full-session-uuid>/<slug> for nested worktrees"
        ));
    }
    Ok(worktree)
}

fn validate_full_session_uuid(session_uuid: &str) -> Result<(), String> {
    let valid = session_uuid.len() == 36
        && session_uuid.chars().enumerate().all(|(index, character)| {
            matches!(index, 8 | 13 | 18 | 23) && character == '-'
                || !matches!(index, 8 | 13 | 18 | 23)
                    && character.is_ascii_hexdigit()
        });
    if valid {
        Ok(())
    } else {
        Err(format!(
            "session UUID must be a full UUID such as 12345678-1234-1234-1234-123456789abc; short id '{session_uuid}' is not accepted"
        ))
    }
}

fn nested_relative_path(name: &str) -> Result<PathBuf, String> {
    let mut parts = name.split('/');
    let session_uuid = parts.next().unwrap_or_default();
    let slug = parts.next().unwrap_or_default();
    if parts.next().is_some() || slug.is_empty() {
        return Err(format!(
            "nested worktree name '{name}' must be <full-session-uuid>/<slug>"
        ));
    }
    validate_full_session_uuid(session_uuid)?;
    Ok(Path::new(session_uuid).join(slug))
}

fn worktree_relative_path(
    git: &WorktreeGit,
    worktree: &session_worktree_provision::WorktreeRef,
) -> Result<PathBuf, String> {
    worktree
        .path
        .strip_prefix(git.main_checkout().join(".worktrees"))
        .map(Path::to_path_buf)
        .map_err(|_| {
            format!(
                "worktree {} is outside .worktrees",
                worktree.path.display()
            )
        })
}

fn rename_target_path(
    source_relative: &Path,
    target_name: &str,
) -> Result<PathBuf, String> {
    if source_relative.components().count() == 2 {
        let target_relative = nested_relative_path(target_name)?;
        if target_relative.parent() != source_relative.parent() {
            return Err(
                "nested worktree rename must keep the same full session UUID"
                    .to_owned(),
            );
        }
        return Ok(target_relative);
    }
    if target_name.contains('/') {
        return Err(
            "legacy worktree rename target must be a flat name".to_owned()
        );
    }
    Ok(PathBuf::from(target_name))
}

fn branch_for_relative_path(relative_path: &Path) -> Result<String, String> {
    let value = relative_path
        .to_str()
        .ok_or("worktree path must be valid UTF-8")?;
    Ok(format!("agent/{}", value.replace('\\', "/")))
}

fn nested_slug_directories(
    main_checkout: &Path,
    session_uuid: &str,
) -> Result<Vec<String>, String> {
    let parent = main_checkout.join(".worktrees").join(session_uuid);
    if !parent.is_dir() {
        return Ok(Vec::new());
    }
    let mut slugs = std::fs::read_dir(parent)
        .map_err(|error| error.to_string())?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|kind| kind.is_dir())
                .map(|_| entry)
        })
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();
    slugs.sort();
    Ok(slugs)
}

fn nested_worktree_parent(
    main_checkout: &Path,
    worktree_path: &Path,
) -> Option<PathBuf> {
    let relative = worktree_path
        .strip_prefix(main_checkout.join(".worktrees"))
        .ok()?;
    if relative.components().count() == 2 {
        worktree_path.parent().map(Path::to_path_buf)
    } else {
        None
    }
}

fn remove_empty_nested_parent(
    main_checkout: &Path,
    worktree_path: &Path,
) -> Result<(), String> {
    let Some(parent) = nested_worktree_parent(main_checkout, worktree_path)
    else {
        return Ok(());
    };
    if parent.is_dir()
        && std::fs::read_dir(&parent)
            .map_err(|error| error.to_string())?
            .next()
            .is_none()
    {
        std::fs::remove_dir(parent).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn rebase_onto_local_main(worktree: &std::path::Path) -> Result<(), String> {
    let output = git_command(worktree)
        .args(["rebase", "main"])
        .output()
        .map_err(|error| format!("failed to start git rebase: {error}"))?;
    if output.status.success() {
        return Ok(());
    }

    Err(format!(
        "git rebase main failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GitlinkState {
    Ok,
    Behind,
    Orphan,
    NotContained,
    Unresolvable,
}

#[derive(Debug)]
struct GitlinkStatus {
    submodule_path: String,
    recorded_sha: Oid,
    main_sha: Oid,
    state: GitlinkState,
}

fn verify_gitlink_containment(
    repo_root: &Path
) -> Result<Vec<GitlinkStatus>, String> {
    let superproject =
        Repository::open(repo_root).map_err(|error| error.to_string())?;
    let head = superproject.head().map_err(|error| error.to_string())?;
    let commit = head.peel_to_commit().map_err(|error| error.to_string())?;
    let tree = commit.tree().map_err(|error| error.to_string())?;
    let paths = WorktreeGit::open(repo_root)
        .map_err(|error| error.to_string())?
        .submodule_paths()
        .map_err(|error| error.to_string())?;

    paths
        .into_iter()
        .map(|submodule_path| {
            let recorded_sha = tree
                .get_path(Path::new(&submodule_path))
                .map_err(|error| error.to_string())?
                .id();
            let submodule = Repository::open(repo_root.join(&submodule_path))
                .map_err(|error| format!("failed to open submodule {submodule_path}: {error}"))?;
            let main = submodule
                .find_branch("main", BranchType::Local)
                .map_err(|error| format!("submodule {submodule_path} has no local main branch: {error}"))?;
            let main_sha = main
                .get()
                .target()
                .ok_or_else(|| format!("submodule {submodule_path} main has no target"))?;
            let state = match submodule.find_commit(recorded_sha) {
                Ok(_) => {
                    let contained_in_main = main_sha == recorded_sha
                        || submodule
                            .graph_descendant_of(main_sha, recorded_sha)
                            .map_err(|error| error.to_string())?;
                    if contained_in_main {
                        if main_sha == recorded_sha {
                            GitlinkState::Ok
                        } else {
                            GitlinkState::Behind
                        }
                    } else if branch_contains(&submodule, recorded_sha)? {
                        GitlinkState::NotContained
                    } else {
                        GitlinkState::Orphan
                    }
                }
                Err(error) if error.code() == git2::ErrorCode::NotFound =>
                    GitlinkState::Unresolvable,
                Err(error) => return Err(error.to_string()),
            };
            Ok(GitlinkStatus {
                submodule_path,
                recorded_sha,
                main_sha,
                state,
            })
        })
        .collect()
}

fn branch_contains(
    repository: &Repository,
    commit: Oid,
) -> Result<bool, String> {
    for branch in repository
        .branches(Some(BranchType::Local))
        .map_err(|error| error.to_string())?
    {
        let (branch, _) = branch.map_err(|error| error.to_string())?;
        let Some(tip) = branch.get().target() else {
            continue;
        };
        if tip == commit
            || repository
                .graph_descendant_of(tip, commit)
                .map_err(|error| error.to_string())?
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn reject_gitlink_violations(statuses: &[GitlinkStatus]) -> Result<(), String> {
    let violations = statuses
        .iter()
        .filter(|status| matches!(
            status.state,
            GitlinkState::Orphan | GitlinkState::NotContained | GitlinkState::Unresolvable
        ))
        .map(|status| match status.state {
            GitlinkState::Unresolvable => format!(
                "submodule {} recorded gitlink {} is not present in that submodule's object database; it was never fetched or has been garbage-collected. Fetch it (`git -C {} fetch <remote-or-local-path>`) or restore it from a rescue branch before merging.",
                status.submodule_path,
                status.recorded_sha,
                status.submodule_path,
            ),
            _ => format!(
                "submodule {} recorded {} is {:?}; local main is {}; run `git -C {} checkout main && git -C {} merge --ff-only {}` (a named feature branch, or the recorded commit sha itself if no branch points at it), then bump the gitlink",
                status.submodule_path,
                status.recorded_sha,
                status.state,
                status.main_sha,
                status.submodule_path,
                status.submodule_path,
                status.recorded_sha,
            ),
        })
        .collect::<Vec<_>>();
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "gitlink containment failed:\n{}",
            violations.join("\n")
        ))
    }
}

// Split gitlink violations into the ones with exactly one safe resolution
// (recorded commit is a strict fast-forward ahead of local main, so
// fast-forwarding main to it is the only possible fix) versus everything
// else, which needs a human to pick a side (true divergence or a missing
// commit).
fn partition_gitlink_statuses(
    repo_root: &Path,
    statuses: Vec<GitlinkStatus>,
) -> Result<(Vec<GitlinkStatus>, Vec<GitlinkStatus>), String> {
    let mut fixable = Vec::new();
    let mut blocking = Vec::new();
    for status in statuses {
        if status.state == GitlinkState::Unresolvable {
            blocking.push(status);
            continue;
        }
        if !matches!(
            status.state,
            GitlinkState::Orphan | GitlinkState::NotContained
        ) {
            continue;
        }
        let submodule = Repository::open(repo_root.join(&status.submodule_path))
            .map_err(|error| error.to_string())?;
        let fast_forwardable = status.main_sha == status.recorded_sha
            || submodule
                .graph_descendant_of(status.recorded_sha, status.main_sha)
                .map_err(|error| error.to_string())?;
        if fast_forwardable {
            fixable.push(status);
        } else {
            blocking.push(status);
        }
    }
    Ok((fixable, blocking))
}

fn repository_has_branch(
    path: &Path,
    branch: &str,
) -> Result<bool, String> {
    let repository = match Repository::open(path) {
        Ok(repository) => repository,
        Err(_) => return Ok(false),
    };
    match repository.find_branch(branch, BranchType::Local) {
        Ok(_) => Ok(true),
        Err(error) if error.code() == git2::ErrorCode::NotFound => Ok(false),
        Err(error) => Err(error.to_string()),
    }
}

fn reject_unmerged_submodule_branch(
    repository_path: &Path,
    branch: &str,
    submodule_path: &str,
) -> Result<(), String> {
    let repository =
        Repository::open(repository_path).map_err(|error| error.to_string())?;
    let feature_sha = repository
        .find_branch(branch, BranchType::Local)
        .map_err(|error| error.to_string())?
        .get()
        .target()
        .ok_or_else(|| {
            format!("submodule {submodule_path} branch {branch} has no target")
        })?;
    let main_sha = repository
        .find_branch("main", BranchType::Local)
        .map_err(|error| error.to_string())?
        .get()
        .target()
        .ok_or_else(|| {
            format!("submodule {submodule_path} main has no target")
        })?;
    if main_sha == feature_sha
        || repository
            .graph_descendant_of(main_sha, feature_sha)
            .map_err(|error| error.to_string())?
    {
        Ok(())
    } else {
        Err(format!(
            "submodule {submodule_path} branch {branch} ({feature_sha}) is not contained in local main ({main_sha}); run `git -C {submodule_path} checkout main && git -C {submodule_path} merge --ff-only {branch}` before merging the superproject"
        ))
    }
}

fn checkout_and_rebase(
    worktree: &Path,
    branch: &str,
) -> Result<(), String> {
    run_git(worktree, ["checkout", branch])?;
    rebase_onto_local_main(worktree)
}

fn commit_rebased_gitlink(
    worktree: &Path,
    submodule: &str,
) -> Result<(), String> {
    let repository =
        Repository::open(worktree).map_err(|error| error.to_string())?;
    let parent = repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map_err(|error| error.to_string())?;
    let mut index = repository.index().map_err(|error| error.to_string())?;
    index
        .add_path(Path::new(submodule))
        .map_err(|error| error.to_string())?;
    let tree_id = index.write_tree().map_err(|error| error.to_string())?;
    let tree = repository
        .find_tree(tree_id)
        .map_err(|error| error.to_string())?;
    if tree.id() == parent.tree_id() {
        return Ok(());
    }
    index.write().map_err(|error| error.to_string())?;
    let signature =
        repository.signature().map_err(|error| error.to_string())?;
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("rebase submodule {submodule} onto local main"),
            &tree,
            &[&parent],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn handle_doctor(dry_run: bool) -> Result<(), String> {
    let main_checkout =
        env::current_dir().map_err(|error| error.to_string())?;
    let git =
        WorktreeGit::open(&main_checkout).map_err(|error| error.to_string())?;
    let mut plan = LifecyclePlan::default();

    for submodule in git.submodule_paths().map_err(|error| error.to_string())? {
        let path = git.main_checkout().join(&submodule);
        if let Some(config) =
            stale_worktree_config(git.main_checkout(), &submodule)?
        {
            println!(
                "submodule={submodule} status=stale-core-worktree path={}",
                config.display()
            );
            plan.add(format!(
                "unset stale core.worktree for submodule {submodule}"
            ));
            plan.add(format!(
                "prune nested worktree registrations for submodule {submodule}"
            ));
        }
        if Repository::open(&path).is_err() {
            println!("submodule={submodule} status=deinitialized");
            plan.add(format!(
                "initialize and update deinitialized submodule {submodule}"
            ));
        }
    }
    plan.add("prune stale superproject worktree registrations");
    if dry_run {
        plan.emit();
        return Ok(());
    }

    for submodule in git.submodule_paths().map_err(|error| error.to_string())? {
        let path = git.main_checkout().join(&submodule);
        if stale_worktree_config(git.main_checkout(), &submodule)?.is_some() {
            unset_core_worktree(git.main_checkout(), &submodule)?;
            run_git(&path, ["worktree", "prune"])?;
        }
        if Repository::open(&path).is_err() {
            initialize_submodule(git.main_checkout(), &submodule)?;
        }
    }
    git.worktree_prune().map_err(|error| error.to_string())?;
    println!("doctor: repairs complete");
    Ok(())
}

fn submodule_status(
    git: &WorktreeGit,
    worktree: &Path,
) -> Result<String, String> {
    let missing = git
        .submodule_paths()
        .map_err(|error| error.to_string())?
        .into_iter()
        .filter(|submodule| Repository::open(worktree.join(submodule)).is_err())
        .collect::<Vec<_>>();
    Ok(if missing.is_empty() {
        "initialized".to_owned()
    } else {
        format!("missing({})", missing.join(","))
    })
}

fn lifecycle_status(
    git: &WorktreeGit,
    activity: &SessionStoreActivity,
    worktree: &session_worktree_provision::WorktreeRef,
    policy: &ProvisionPolicy,
) -> Result<String, String> {
    match evaluate_reclaim_candidate(git, activity, worktree, policy)
        .map_err(|error| error.to_string())?
    {
        ReclaimEligibility::Reclaimable => Ok("reclaimable".to_owned()),
        ReclaimEligibility::Rejected(reason) =>
            Ok(format!("preserved reason={}", rejection_reason(&reason))),
    }
}

fn rejection_reason(reason: &ReclaimRejectionReason) -> String {
    match reason {
        ReclaimRejectionReason::OutsideWorktreeRoot =>
            "outside-worktree-root".to_owned(),
        ReclaimRejectionReason::SessionActive => "session-active".to_owned(),
        ReclaimRejectionReason::Detached => "detached".to_owned(),
        ReclaimRejectionReason::Dirty => "dirty".to_owned(),
        ReclaimRejectionReason::ContainsCurrentDirectory =>
            "contains-current-directory".to_owned(),
        ReclaimRejectionReason::NotIdle => "not-idle".to_owned(),
        ReclaimRejectionReason::DirtySubmodule { path } =>
            format!("dirty-submodule:{}", path.display()),
        ReclaimRejectionReason::AheadOfMain => "ahead-of-main".to_owned(),
    }
}

fn merge_ff_only(
    repository: &Path,
    branch: &str,
) -> Result<(), String> {
    run_git(repository, ["merge", "--ff-only", branch]).map_err(|error| {
        format!(
            "merge --ff-only failed for {} from {branch}: {error}; rebase the feature branch onto local main and retry",
            repository.display()
        )
    })
}

fn stale_worktree_config(
    main_checkout: &Path,
    submodule: &str,
) -> Result<Option<PathBuf>, String> {
    let config_path = main_checkout
        .join(".git")
        .join("modules")
        .join(submodule)
        .join("config");
    if !config_path.exists() {
        return Ok(None);
    }
    let config =
        git2::Config::open(&config_path).map_err(|error| error.to_string())?;
    let value = match config.get_string("core.worktree") {
        Ok(value) => value,
        Err(error) if error.code() == git2::ErrorCode::NotFound =>
            return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    let configured_path = PathBuf::from(value);
    let resolved_path = if configured_path.is_absolute() {
        configured_path
    } else {
        config_path
            .parent()
            .ok_or("submodule config has no parent")?
            .join(configured_path)
    };
    Ok((!resolved_path.exists()).then_some(resolved_path))
}

fn initialize_submodule(
    main_checkout: &Path,
    submodule: &str,
) -> Result<(), String> {
    let repository =
        Repository::open(main_checkout).map_err(|error| error.to_string())?;
    let mut handle = repository
        .find_submodule(submodule)
        .map_err(|error| error.to_string())?;
    handle.init(true).map_err(|error| error.to_string())?;
    handle.update(true, None).map_err(|error| error.to_string())
}

fn unset_core_worktree(
    main_checkout: &Path,
    submodule: &str,
) -> Result<(), String> {
    let config = main_checkout
        .join(".git")
        .join("modules")
        .join(submodule)
        .join("config");
    let mut config =
        git2::Config::open(&config).map_err(|error| error.to_string())?;
    config
        .remove("core.worktree")
        .map_err(|error| error.to_string())
}

fn run_git<const N: usize>(
    repository: &Path,
    arguments: [&str; N],
) -> Result<(), String> {
    let output = git_command(repository)
        .args(arguments)
        .output()
        .map_err(|error| format!("failed to start git: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

fn git_command(repository: &Path) -> ProcessCommand {
    let mut command = ProcessCommand::new("git");
    command.current_dir(normalized_git_path(repository));
    command
}

fn normalized_git_path(path: &Path) -> String {
    let path = path.to_string_lossy();
    let path = path
        .strip_prefix(r"\\?\UNC\")
        .map(|path| format!("//{path}"))
        .or_else(|| path.strip_prefix(r"\\?\").map(str::to_owned))
        .unwrap_or_else(|| path.into_owned());
    path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{
        BRANCH_TEMPLATE,
        Cli,
        Command,
        DIRTY_MAIN_UNCOMMITTED_CHANGES_MESSAGE,
        FINISH_READY_TO_MERGE_MARKER,
        PRESERVE_MAIN_CHANGES_HINT,
        WORKTREE_PATH_OUTPUT_PREFIX,
        WORKTREE_PATH_TEMPLATE,
    };

    #[test]
    fn defines_lifecycle_output_contract_constants() {
        assert_eq!(WORKTREE_PATH_OUTPUT_PREFIX, "WORKTREE_PATH=");
        assert_eq!(FINISH_READY_TO_MERGE_MARKER, "ready-to-merge");
        assert_eq!(
            DIRTY_MAIN_UNCOMMITTED_CHANGES_MESSAGE,
            "uncommitted changes"
        );
        assert_eq!(PRESERVE_MAIN_CHANGES_HINT, "preserve-main-changes");
        assert_eq!(
            WORKTREE_PATH_TEMPLATE,
            ".worktrees/<full-session-uuid>/<slug>"
        );
        assert_eq!(BRANCH_TEMPLATE, "agent/<full-session-uuid>/<slug>");
    }

    #[test]
    fn parses_new_with_all_flags() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "new",
            "12345678-1234-1234-1234-123456789abc",
            "worktree-ctl",
            "--dry-run",
            "--preserve-main-changes",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::New {
                session_uuid: "12345678-1234-1234-1234-123456789abc".to_owned(),
                slug: "worktree-ctl".to_owned(),
                dry_run: true,
                preserve_main_changes: true,
            }
        );
    }

    #[test]
    fn parses_list() {
        let cli = Cli::try_parse_from(["worktree-ctl", "list"]).unwrap();

        assert_eq!(cli.command, Command::List { dry_run: false });
    }

    #[test]
    fn parses_rebase_with_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "rebase",
            "example",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Rebase {
                name: "example".to_owned(),
                dry_run: true,
                auto_commit: false,
            }
        );
    }

    #[test]
    fn parses_merge_with_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "merge",
            "example",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Merge {
                name: "example".to_owned(),
                dry_run: true,
                auto_commit: false,
            }
        );
    }

    #[test]
    fn parses_sync_with_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "sync",
            "example",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Sync {
                name: "example".to_owned(),
                dry_run: true,
                auto_commit: false,
            }
        );
    }

    #[test]
    fn parses_sync_with_auto_commit() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "sync",
            "example",
            "--auto-commit",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Sync {
                name: "example".to_owned(),
                dry_run: false,
                auto_commit: true,
            }
        );
    }

    #[test]
    fn parses_remove_with_force_and_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "remove",
            "example",
            "--force",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Remove {
                name: "example".to_owned(),
                force: true,
                dry_run: true,
            }
        );
    }

    #[test]
    fn parses_rename_with_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "rename",
            "old-name",
            "new-name",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Rename {
                source_name: "old-name".to_owned(),
                target_name: "new-name".to_owned(),
                dry_run: true,
            }
        );
    }

    #[test]
    fn parses_finish_with_dry_run() {
        let cli = Cli::try_parse_from([
            "worktree-ctl",
            "finish",
            "example",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(
            cli.command,
            Command::Finish {
                name: "example".to_owned(),
                dry_run: true,
            }
        );
    }

    #[test]
    fn parses_doctor_with_dry_run() {
        let cli = Cli::try_parse_from(["worktree-ctl", "doctor", "--dry-run"])
            .unwrap();

        assert_eq!(cli.command, Command::Doctor { dry_run: true });
    }

    #[test]
    fn accepts_dry_run_for_every_mutating_subcommand() {
        for args in [
            vec![
                "new",
                "12345678-1234-1234-1234-123456789abc",
                "slug",
                "--dry-run",
            ],
            vec!["rebase", "example", "--dry-run"],
            vec!["merge", "example", "--dry-run"],
            vec!["sync", "example", "--dry-run"],
            vec!["remove", "example", "--dry-run"],
            vec!["rename", "old", "new", "--dry-run"],
            vec!["finish", "example", "--dry-run"],
            vec!["doctor", "--dry-run"],
        ] {
            let mut command_line = vec!["worktree-ctl"];
            command_line.extend(args);
            assert!(Cli::try_parse_from(command_line).is_ok());
        }
    }
}
