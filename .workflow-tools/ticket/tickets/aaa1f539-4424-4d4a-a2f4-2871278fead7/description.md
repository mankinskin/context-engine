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