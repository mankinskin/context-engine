use std::{ffi::OsStr, fs, path::{Path, PathBuf}, process::{Command, Output}};
use tempfile::TempDir;

struct Fixture { _temp: TempDir, main: PathBuf, tool: PathBuf }
impl Fixture {
    fn worktree(&self, name: &str) -> PathBuf { self.main.join(".worktrees").join(name) }
    fn run<I, S>(&self, args: I) -> Output where I: IntoIterator<Item = S>, S: AsRef<OsStr> {
        Command::new(&self.tool).args(args).current_dir(&self.main).env("GIT_ALLOW_PROTOCOL", "file").output().expect("tool starts")
    }
}
fn fixture_repo(tool: &Path) -> Fixture {
    let temp = tempfile::tempdir().expect("tempdir"); let source = temp.path().join("submodule-source"); let main = temp.path().join("main");
    init_repo(&source); fs::write(source.join("file.txt"), "initial\n").unwrap(); git(&source, &["add", "file.txt"]); git(&source, &["commit", "-m", "initial"]);
    init_repo(&main); fs::write(main.join("README"), "fixture\n").unwrap(); git(&main, &["add", "README"]); git(&main, &["commit", "-m", "initial"]);
    git(&main, &["-c", "protocol.file.allow=always", "submodule", "add", source.to_str().unwrap(), "modules/example"]); git(&main, &["commit", "-m", "add submodule"]);
    Fixture { _temp: temp, main, tool: tool.to_path_buf() }
}
fn init_repo(path: &Path) { git_in(path.parent().unwrap(), &["init", "--initial-branch=main", path.to_str().unwrap()]); git(path, &["config", "user.email", "test@example.invalid"]); git(path, &["config", "user.name", "test"]); }
fn git(repo: &Path, args: &[&str]) { git_in(repo, args) }
fn git_in(directory: &Path, args: &[&str]) { let output = Command::new("git").args(args).current_dir(directory).output().unwrap(); ok(&output, "git command"); }
fn git_out(repo: &Path, args: &[&str]) -> String { let output = Command::new("git").args(args).current_dir(repo).output().unwrap(); ok(&output, "git command"); out(&output).trim().to_owned() }
fn recorded_sha(repo: &Path, path: &str) -> String { git_out(repo, &["rev-parse", &format!("HEAD:{path}")]) }
fn out(output: &Output) -> String { String::from_utf8_lossy(&output.stdout).into_owned() }
fn all(output: &Output) -> String { format!("{}{}", out(output), String::from_utf8_lossy(&output.stderr)) }
fn ok(output: &Output, label: &str) { assert!(output.status.success(), "{label} failed: {}", all(output)); }
fn fails_with(output: &Output, expected: &str) { assert!(!output.status.success(), "command unexpectedly succeeded: {}", all(output)); assert!(all(output).contains(expected), "expected {expected:?}: {}", all(output)); }
fn tool() -> PathBuf { PathBuf::from(env!("CARGO_BIN_EXE_worktree-ctl")) }
fn create(fixture: &Fixture, id: &str, slug: &str) { ok(&fixture.run(["new", id, slug]), "new worktree") }

#[test] fn all_lifecycle_ops_support_dry_run() {
    let f = fixture_repo(&tool()); let before = git_out(&f.main, &["rev-parse", "main"]); ok(&f.run(["new", "dry", "run", "--dry-run"]), "dry-run new"); assert!(!f.worktree("dry-run").exists()); create(&f, "dry", "run");
    ok(&f.run(["rebase", "dry-run", "--dry-run"]), "dry-run rebase"); ok(&f.run(["rename", "dry-run", "dry-renamed", "--dry-run"]), "dry-run rename"); assert!(f.worktree("dry-run").is_dir()); assert!(!f.worktree("dry-renamed").exists());
    ok(&f.run(["remove", "dry-run", "--dry-run"]), "dry-run remove"); assert!(f.worktree("dry-run").is_dir()); ok(&f.run(["finish", "dry-run", "--dry-run"]), "dry-run finish"); assert!(f.worktree("dry-run").is_dir()); assert_eq!(before, git_out(&f.main, &["rev-parse", "main"]));
}
#[test] fn bootstrap_populates_submodule_offline() { let f = fixture_repo(&tool()); let sha = recorded_sha(&f.main, "modules/example"); create(&f, "offline", "bootstrap"); let sub = f.worktree("offline-bootstrap/modules/example"); assert!(sub.join("file.txt").is_file()); git(&sub, &["cat-file", "-e", &sha]); }
#[test] fn bootstrap_resolves_main_only_commit() { let f = fixture_repo(&tool()); let main_sub = f.main.join("modules/example"); fs::write(main_sub.join("file.txt"), "initial\nmain-only\n").unwrap(); git(&main_sub, &["commit", "-am", "main-only commit"]); git(&f.main, &["add", "modules/example"]); git(&f.main, &["commit", "-m", "record main-only submodule commit"]); let sha = recorded_sha(&f.main, "modules/example"); create(&f, "local", "object"); let sub = f.worktree("local-object/modules/example"); git(&sub, &["cat-file", "-e", &sha]); assert_eq!(sha, git_out(&sub, &["rev-parse", "HEAD"])); }
#[test] fn create_preserves_dirty_main_checkout() { let f = fixture_repo(&tool()); fs::write(f.main.join("README"), "fixture\ndirty main change\n").unwrap(); fails_with(&f.run(["new", "preserve", "dirty"]), "README"); assert!(!f.worktree("preserve-dirty").exists()); let result = f.run(["new", "preserve", "dirty", "--preserve-main-changes"]); ok(&result, "preserved new"); assert!(all(&result).contains("README")); assert!(git_out(&f.main, &["stash", "list"]).contains("preserve-main-changes")); }
#[test] fn create_requires_acknowledgement_when_dirty() { let f = fixture_repo(&tool()); fs::write(f.main.join("README"), "fixture\nunacknowledged change\n").unwrap(); fails_with(&f.run(["new", "acknowledge", "dirty"]), "uncommitted changes"); assert!(fs::read_to_string(f.main.join("README")).unwrap().contains("unacknowledged change")); assert!(!f.worktree("acknowledge-dirty").exists()); }
#[test] fn dry_run_plan_has_no_origin() { let f = fixture_repo(&tool()); let result = f.run(["new", "dryrun", "plan", "--dry-run"]); ok(&result, "dry-run plan"); let plan = all(&result); assert!(!plan.contains("fetch origin")); assert!(!plan.contains("origin/main")); }
#[test] fn finish_rebases_marks_ready_and_removes() { let f = fixture_repo(&tool()); create(&f, "finish", "ready"); let worktree = f.worktree("finish-ready"); fs::write(worktree.join("completed.txt"), "completed\n").unwrap(); git(&worktree, &["add", "completed.txt"]); git(&worktree, &["commit", "-m", "completed"]); let before = git_out(&f.main, &["rev-parse", "main"]); let result = f.run(["finish", "finish-ready"]); ok(&result, "finish"); assert_eq!(before, git_out(&f.main, &["rev-parse", "main"])); assert!(!worktree.exists()); assert!(git_out(&f.main, &["branch", "--list", "agent/finish-ready"]).contains("agent/finish-ready")); assert!(all(&result).contains("ready-to-merge")); }
#[test] fn no_origin_references() { let f = fixture_repo(&tool()); let result = f.run(["new", "no-origin", "behavior"]); ok(&result, "new without origin"); let output = all(&result); assert!(!output.contains("fetch origin")); assert!(!output.contains("origin/main")); }
#[test] fn no_submodule_deinit() { let f = fixture_repo(&tool()); create(&f, "deinit", "guard"); let worktree = f.worktree("deinit-guard"); fs::write(worktree.join("dirty.txt"), "preserve main module\n").unwrap(); ok(&f.run(["remove", "deinit-guard", "--force"]), "forced remove"); assert!(f.main.join("modules/example/file.txt").is_file()); }
#[test] fn no_worktree_move() { let f = fixture_repo(&tool()); create(&f, "move", "source"); ok(&f.run(["rename", "move-source", "move-target"]), "rename with submodule"); assert!(!f.worktree("move-source").exists()); assert!(f.worktree("move-target/modules/example/file.txt").is_file()); }
#[test] fn remove_refuses_dirty_worktree() { let f = fixture_repo(&tool()); create(&f, "remove", "dirty"); let worktree = f.worktree("remove-dirty"); fs::write(worktree.join("dirty.txt"), "do not lose me\n").unwrap(); fails_with(&f.run(["remove", "remove-dirty"]), "dirty.txt"); assert!(worktree.join("dirty.txt").is_file()); ok(&f.run(["remove", "remove-dirty", "--force"]), "forced remove"); assert!(!worktree.exists()); }
#[test] fn rename_is_remove_and_recreate() { let f = fixture_repo(&tool()); create(&f, "rename", "source"); ok(&f.run(["rename", "rename-source", "rename-target"]), "rename"); let worktree = f.worktree("rename-target"); let sha = recorded_sha(&worktree, "modules/example"); assert!(!f.worktree("rename-source").exists()); git(&worktree.join("modules/example"), &["cat-file", "-e", &sha]); }
#[test] fn rename_preserves_commit_ahead_of_gitlink() { let f = fixture_repo(&tool()); create(&f, "ahead", "source"); let sub = f.worktree("ahead-source/modules/example"); fs::write(sub.join("file.txt"), "initial\nahead\n").unwrap(); git(&sub, &["commit", "-am", "ahead"]); let sha = git_out(&sub, &["rev-parse", "HEAD"]); ok(&f.run(["rename", "ahead-source", "ahead-target"]), "rename"); git(&f.worktree("ahead-target/modules/example"), &["cat-file", "-e", &sha]); }
#[test] fn second_worktree_requires_explicit_override() { let f = fixture_repo(&tool()); create(&f, "duplicate", "first"); fails_with(&f.run(["new", "duplicate", "second"]), "--allow-additional"); ok(&f.run(["new", "duplicate", "second", "--allow-additional"]), "explicit second worktree"); assert!(f.worktree("duplicate-first").is_dir()); assert!(f.worktree("duplicate-second").is_dir()); }
#[test] fn session_reuses_existing_worktree() { let f = fixture_repo(&tool()); let first = f.run(["new", "reuse", "session"]); ok(&first, "first new"); let second = f.run(["new", "reuse", "session"]); ok(&second, "second new"); let first_output = out(&first); let second_output = out(&second); let one = first_output.lines().find_map(|line| line.strip_prefix("WORKTREE_PATH=")).expect("first worktree path"); let two = second_output.lines().find_map(|line| line.strip_prefix("WORKTREE_PATH=")).expect("second worktree path"); assert_eq!(one, two); let count = fs::read_dir(f.main.join(".worktrees")).unwrap().filter_map(Result::ok).filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir())).count(); assert_eq!(count, 1); }
