use std::{
    ffi::OsStr,
    fs,
    path::{
        Path,
        PathBuf,
    },
    process::{
        Command,
        Output,
    },
};

use tempfile::TempDir;

struct Fixture {
    _temp: TempDir,
    main: PathBuf,
    tool: PathBuf,
}

impl Fixture {
    fn worktree(
        &self,
        name: &str,
    ) -> PathBuf {
        self.main.join(".worktrees").join(name)
    }

    fn run<I, S>(
        &self,
        args: I,
    ) -> Output
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new(&self.tool)
            .args(args)
            .current_dir(&self.main)
            .env("GIT_ALLOW_PROTOCOL", "file")
            .output()
            .expect("tool starts")
    }
}

fn fixture_repo() -> Fixture {
    let temp = tempfile::tempdir().expect("tempdir");
    let source = temp.path().join("submodule-source");
    let main = temp.path().join("main");
    init_repo(&source);
    fs::write(source.join("file.txt"), "initial\n").expect("write source");
    git(&source, &["add", "file.txt"]);
    git(&source, &["commit", "-m", "initial"]);

    init_repo(&main);
    fs::write(main.join("README"), "fixture\n").expect("write readme");
    git(&main, &["add", "README"]);
    git(&main, &["commit", "-m", "initial"]);
    git(
        &main,
        &[
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            source.to_str().expect("utf-8 path"),
            "modules/example",
        ],
    );
    git(&main, &["commit", "-m", "add submodule"]);

    Fixture {
        _temp: temp,
        main,
        tool: PathBuf::from(env!("CARGO_BIN_EXE_worktree-ctl")),
    }
}

fn init_repo(path: &Path) {
    git_in(
        path.parent().expect("repo parent"),
        &[
            "init",
            "--initial-branch=main",
            path.to_str().expect("utf-8 path"),
        ],
    );
    git(path, &["config", "user.email", "test@example.invalid"]);
    git(path, &["config", "user.name", "test"]);
}

fn git(
    repository: &Path,
    arguments: &[&str],
) {
    git_in(repository, arguments);
}

fn git_in(
    directory: &Path,
    arguments: &[&str],
) {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(directory)
        .output()
        .expect("git starts");
    assert!(output.status.success(), "git failed: {}", all(&output));
}

fn git_revision(
    repository: &Path,
    arguments: &[&str],
) -> String {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(repository)
        .output()
        .expect("git starts");
    assert!(output.status.success(), "git failed: {}", all(&output));
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn all(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn create(
    fixture: &Fixture,
    id: &str,
    slug: &str,
) {
    let output = fixture.run(["new", id, slug]);
    assert!(output.status.success(), "new failed: {}", all(&output));
}

#[test]
fn list_reports_lifecycle_state_and_rejection_reason() {
    let fixture = fixture_repo();
    create(&fixture, "list", "state");

    let output = fixture.run(["list", "--dry-run"]);

    assert!(output.status.success(), "list failed: {}", all(&output));
    let report = all(&output);
    assert!(report.contains("branch=agent/list-state"), "{report}");
    assert!(report.contains("submodules=initialized"), "{report}");
    assert!(
        report.contains("lifecycle=preserved reason=not-idle"),
        "{report}"
    );
}

#[test]
fn merge_refuses_non_fast_forward() {
    let fixture = fixture_repo();
    create(&fixture, "merge", "non-ff");
    let worktree = fixture.worktree("merge-non-ff");
    fs::write(worktree.join("feature.txt"), "feature\n")
        .expect("write feature");
    git(&worktree, &["add", "feature.txt"]);
    git(&worktree, &["commit", "-m", "feature"]);
    fs::write(fixture.main.join("main.txt"), "main\n").expect("write main");
    git(&fixture.main, &["add", "main.txt"]);
    git(&fixture.main, &["commit", "-m", "main advanced"]);

    let output = fixture.run(["merge", "merge-non-ff"]);

    assert!(
        !output.status.success(),
        "merge unexpectedly succeeded: {}",
        all(&output)
    );
    assert!(
        all(&output).contains("merge --ff-only failed"),
        "{}",
        all(&output)
    );
}

#[test]
fn doctor_repairs_stale_core_worktree() {
    let fixture = fixture_repo();
    let config = fixture
        .main
        .join(".git")
        .join("modules")
        .join("modules")
        .join("example")
        .join("config");
    let missing = fixture
        .main
        .join(".worktrees")
        .join("manually-deleted")
        .join("modules/example");
    git(
        &fixture.main,
        &[
            "config",
            "--file",
            config.to_str().expect("utf-8 config path"),
            "core.worktree",
            missing.to_str().expect("utf-8 missing path"),
        ],
    );

    let output = fixture.run(["doctor"]);

    assert!(output.status.success(), "doctor failed: {}", all(&output));
    assert!(
        all(&output).contains("stale-core-worktree"),
        "{}",
        all(&output)
    );
    let config_output = Command::new("git")
        .args([
            "config",
            "--file",
            config.to_str().expect("utf-8 config path"),
            "--get",
            "core.worktree",
        ])
        .current_dir(&fixture.main)
        .output()
        .expect("config read starts");
    assert!(
        !config_output.status.success(),
        "core.worktree remains configured"
    );
    let status = Command::new("git")
        .args(["status", "--short"])
        .current_dir(&fixture.main)
        .output()
        .expect("status starts");
    assert!(status.status.success(), "status failed: {}", all(&status));
}

#[test]
fn merge_accepts_bottom_up_gitlink_integration() {
    let fixture = fixture_repo();
    create(&fixture, "merge", "bottom-up");
    let worktree = fixture.worktree("merge-bottom-up");
    let nested = worktree.join("modules/example");
    git(&nested, &["checkout", "-b", "agent/merge-bottom-up"]);
    fs::write(nested.join("file.txt"), "initial\nbottom-up\n")
        .expect("write nested change");
    git(&nested, &["commit", "-am", "nested feature"]);
    git(
        &fixture.main.join("modules/example"),
        &["merge", "--ff-only", "agent/merge-bottom-up"],
    );
    git(&worktree, &["add", "modules/example"]);
    git(&worktree, &["commit", "-m", "bump nested gitlink"]);

    let output = fixture.run(["merge", "merge-bottom-up"]);

    assert!(output.status.success(), "merge failed: {}", all(&output));
    assert_eq!(
        git_revision(&fixture.main, &["rev-parse", "HEAD:modules/example"]),
        git_revision(
            &fixture.main.join("modules/example"),
            &["rev-parse", "main"]
        )
    );
}

#[test]
fn merge_rejects_orphan_gitlink_before_mutation() {
    let fixture = fixture_repo();
    create(&fixture, "merge", "orphan");
    let worktree = fixture.worktree("merge-orphan");
    fs::write(worktree.join("feature.txt"), "feature\n")
        .expect("write feature");
    git(&worktree, &["add", "feature.txt"]);
    git(&worktree, &["commit", "-m", "feature"]);
    let submodule = fixture.main.join("modules/example");
    git(&submodule, &["checkout", "--orphan", "replacement"]);
    git(&submodule, &["rm", "-rf", "."]);
    fs::write(submodule.join("replacement.txt"), "replacement\n")
        .expect("write replacement");
    git(&submodule, &["add", "replacement.txt"]);
    git(&submodule, &["commit", "-m", "replacement"]);
    git(&submodule, &["branch", "-f", "main", "replacement"]);
    git(&submodule, &["checkout", "main"]);
    git(&submodule, &["branch", "-D", "replacement"]);
    let before = git_revision(&fixture.main, &["rev-parse", "main"]);

    let output = fixture.run(["merge", "merge-orphan"]);

    assert!(
        !output.status.success(),
        "merge unexpectedly succeeded: {}",
        all(&output)
    );
    assert!(all(&output).contains("Orphan"), "{}", all(&output));
    assert_eq!(before, git_revision(&fixture.main, &["rev-parse", "main"]));
}

#[test]
fn merge_allows_backward_gitlink_and_dry_run_mutates_nothing() {
    let fixture = fixture_repo();
    create(&fixture, "merge", "behind");
    let worktree = fixture.worktree("merge-behind");
    let submodule = fixture.main.join("modules/example");
    fs::write(submodule.join("file.txt"), "initial\nmain ahead\n")
        .expect("write main change");
    git(&submodule, &["commit", "-am", "main ahead"]);
    fs::write(worktree.join("feature.txt"), "feature\n")
        .expect("write feature");
    git(&worktree, &["add", "feature.txt"]);
    git(&worktree, &["commit", "-m", "feature"]);
    let before = git_revision(&fixture.main, &["rev-parse", "main"]);

    let dry_run = fixture.run(["merge", "merge-behind", "--dry-run"]);
    assert!(
        dry_run.status.success(),
        "dry-run failed: {}",
        all(&dry_run)
    );
    assert!(
        all(&dry_run).contains("skip modules/example"),
        "{}",
        all(&dry_run)
    );
    assert_eq!(before, git_revision(&fixture.main, &["rev-parse", "main"]));
    let merge = fixture.run(["merge", "merge-behind"]);
    assert!(merge.status.success(), "merge failed: {}", all(&merge));
}
