# Goal
Give browser-facing end-to-end tests a stable correlation path into backend/server tracing so flaky or slow Playwright runs can be debugged from one test run id through the relevant HTTP/MCP/store spans.

# Scope
- Extend shared Playwright runtime/profiling helpers under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/` and managed-viewer wrappers to create or propagate a per-test correlation id.
- Ensure server-side tracing surfaces record that id for the relevant viewer/server request paths.
- Prefer structured metadata over ad hoc console scraping.
- Reuse existing browser trace helpers such as `withBrowserTrace` instead of duplicating profiling capture.

# Acceptance criteria
- A Playwright run can be tied to backend logs or trace sessions with one stable id.
- Shared viewer E2E helpers expose the correlation path for ticket-viewer, spec-viewer, doc-viewer, and log-viewer suites.
- Failure triage can distinguish browser-only issues from backend request/store behavior.