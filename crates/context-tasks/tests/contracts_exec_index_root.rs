//! Contract test: `ticket exec` must require an explicit index root (either
//! `--index-root` flag or `TICKET_INDEX_ROOT` env var). Silently falling back
//! to the workspace resolution chain is not allowed for the agent protocol,
//! since it could write to the wrong store.

mod common;
use common::Sandbox;

use std::process::Command;

const TICKET: &str = env!("CARGO_BIN_EXE_ticket");

/// Run exec WITHOUT --index-root and WITHOUT TICKET_INDEX_ROOT set.
/// Expect failure with a diagnostic message referencing index-root.
#[test]
fn exec_without_explicit_index_root_fails() {
    let out = Command::new(TICKET)
        .arg("--json")
        .arg("exec")
        .env_remove("TICKET_INDEX_ROOT")
        // No --index-root flag
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn failed")
        .wait_with_output()
        .expect("wait failed");

    assert!(
        !out.status.success(),
        "exec without --index-root must fail, but it succeeded"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let combined = format!("{stderr}\n{stdout}");

    assert!(
        combined.to_lowercase().contains("index")
            || combined.to_lowercase().contains("root"),
        "error message must mention index/root, got:\nstderr={stderr}\nstdout={stdout}"
    );
}

/// Exec with --index-root explicitly provided must succeed.
#[test]
fn exec_with_explicit_index_root_succeeds() {
    let s = Sandbox::new();
    // Basic create via exec — should succeed because --index-root is given.
    let result = s.ticket_exec(
        r#"{"command":"create","title":"Index root test","type":"tracker-improvement"}"#,
    );
    assert_eq!(result["status"], "ok");
}
