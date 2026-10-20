Implement stricter validation for handoff package repo paths.

Acceptance criteria:
- Handoff package ingestion rejects packages that reference non-existent repo-root-relative paths.
- Error explains which path(s) failed and where in the package they were referenced.
- Existing handoff packages are migrated or flagged with a remediation plan.

Traceability:
- References ticket bd5e9aee-f89b-4d38-be80-80d6c8c1a3b5 (compact-terminal implementation) for example failures.
- Related spec: 63c60c9d-adbe-4ddb-8c1d-6156610d0753 (compact-terminal spec) for expected package layout.

Notes:
- Workspace: C:/Users/linus/git/context-engine/.ticket

## 2026-10-13 repair evidence

Fixed no-worktree package-path validation in nested `workflow-tools/session` commit `cddadba3dcf12c1a6dfaf9096efd18abf3189e6e`. The validation root now derives from the explicit owning session-store root; active assigned worktrees retain precedence. Regression coverage exercises canonical `.workflow-tools/session`, legacy `.session`, and relocated roots. Focused test and `cargo test -p session-api --lib` passed; the actual rebuilt `session.exe --workspace C:\Users\linus\git\3\meta-workspace handoff` accepted `context-agent/Cargo.toml`, read back from `.workflow-tools/session/sessions/29ad8d96-9400-4627-a032-e02a3d8bd971/handoffs/678c5d45-86bf-4731-afbf-fc339b7d1e8a/handoff.json`.

Canonical validation evidence: `.workflow-tools/test/specs/session-handoff-owning-root-regression.json`; `.workflow-tools/test/executions/exec-session-handoff-owning-root-20261013.json`.