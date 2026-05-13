use std::{
    fs,
    path::Path,
    process::Command,
};

use tempfile::TempDir;

use crate::{
    TransplantArgs,
    cli::parse_mapping,
    execute,
};

#[test]
fn transplant_imports_filtered_history_into_target_repo() {
    let temp = TempDir::new().expect("tempdir should exist");
    let source_repo = temp.path().join("source");
    let target_repo = temp.path().join("target");

    init_repo(&source_repo);
    commit_file(&source_repo, "README.md", "root\n", "initial root commit");
    commit_files(
        &source_repo,
        &[
            ("tools/cli/context-cli/src/main.rs", "fn main() {}\n"),
            ("tools/http/context-http/src/lib.rs", "pub fn serve() {}\n"),
            ("tools/mcp/context-mcp/src/main.rs", "fn main() {}\n"),
        ],
        "shared tools scaffold",
    );
    commit_file(
        &source_repo,
        "tools/context-editor/kernel/Cargo.toml",
        "[package]\nname = \"context-editor-kernel\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        "context editor init",
    );
    commit_file(
        &source_repo,
        "unrelated/file.txt",
        "ignore me\n",
        "unrelated change",
    );
    commit_files(
        &source_repo,
        &[
            ("tools/cli/context-cli/src/main.rs", "fn main() { println!(\"cli\"); }\n"),
            ("tools/http/context-http/src/lib.rs", "pub fn serve() { println!(\"http\"); }\n"),
        ],
        "shared cli http update",
    );

    init_repo(&target_repo);
    commit_file(
        &target_repo,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = []\n",
        "target init",
    );

    let args = TransplantArgs {
        source_repo: source_repo.clone(),
        target_repo: target_repo.clone(),
        source_ref: "HEAD".to_string(),
        target_branch: "main".to_string(),
        import_branch: "crane/import".to_string(),
        anchor_commit: None,
        mappings: vec![
            parse_mapping("tools/cli/context-cli=tools/cli/context-cli")
                .expect("cli mapping should parse"),
            parse_mapping("tools/mcp/context-mcp=tools/mcp/context-mcp")
                .expect("mcp mapping should parse"),
            parse_mapping("tools/http/context-http=tools/http/context-http")
                .expect("http mapping should parse"),
            parse_mapping("tools/context-editor=tools/context-editor")
                .expect("editor mapping should parse"),
        ],
        no_merge: false,
        dry_run: false,
    };

    let outcome = execute(args).expect("transplant should succeed");

    assert!(outcome.merged);
    assert!(outcome.stats.is_some());
    assert_eq!(
        fs::read_to_string(target_repo.join("tools/cli/context-cli/src/main.rs"))
            .expect("cli file should exist"),
        "fn main() { println!(\"cli\"); }\n"
    );
    assert!(
        target_repo.join("tools/context-editor/kernel/Cargo.toml").exists(),
        "context-editor history should be imported"
    );

    let combined_log = git_output(
        &target_repo,
        &[
            "log",
            "--format=%s",
            "--all",
            "--",
            "tools/cli/context-cli",
            "tools/http/context-http",
            "tools/mcp/context-mcp",
            "tools/context-editor",
        ],
    );
    assert!(combined_log.contains("shared tools scaffold"));
    assert!(combined_log.contains("shared cli http update"));
    assert!(combined_log.contains("context editor init"));
    assert!(
        !combined_log.contains("unrelated change"),
        "unrelated history should be excluded"
    );
}

#[test]
fn parse_mapping_rejects_empty_destination() {
    let error = parse_mapping("tools/foo=").expect_err("mapping must fail");
    assert!(error.contains("destination path is empty"));
}

fn init_repo(path: &Path) {
    fs::create_dir_all(path).expect("repo dir should exist");
    run_git(path, &["init", "-b", "main"]);
    run_git(path, &["config", "user.name", "Crane Test"]);
    run_git(path, &["config", "user.email", "crane@example.com"]);
}

fn commit_file(
    repo: &Path,
    file_path: &str,
    contents: &str,
    message: &str,
) {
    commit_files(repo, &[(file_path, contents)], message);
}

fn commit_files(
    repo: &Path,
    files: &[(&str, &str)],
    message: &str,
) {
    for (file_path, contents) in files {
        let full_path = repo.join(file_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("parent dir should exist");
        }
        fs::write(full_path, contents).expect("file should be written");
    }
    run_git(repo, &["add", "."]);
    run_git(repo, &["commit", "-m", message]);
}

fn run_git(
    repo: &Path,
    args: &[&str],
) {
    let status = Command::new("git")
        .args(args)
        .current_dir(repo)
        .status()
        .expect("git command should run");
    assert!(status.success(), "git {:?} failed", args);
}

fn git_output(
    repo: &Path,
    args: &[&str],
) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("git command should run");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).expect("git output should be utf8")
}