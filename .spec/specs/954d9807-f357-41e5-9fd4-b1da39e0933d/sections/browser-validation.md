# Browser validation contract

Any change to a server interface or to a frontend feature must be verified in an external Chromium-family browser before the work moves to `in-review`. VS Code's integrated browser is not a substitute and must not be used for this gate.

## Tool order

1. Prefer the MCP Playwright/browser tools when they cover the scenario.
2. Fall back to repo-local Playwright commands only when MCP cannot drive the page.
3. As a last resort, launch the external browser through the repo's browser-open task helpers.

## Evidence

- Capture screenshots for UI-facing changes so the rendered state is visually confirmed and not only inferred from DOM assertions.
- For modals, overlays, drawers, popovers, and menus, include at least one screenshot with the surface open, and a before/after pair when useful.
- Record the browser window or display resolution used whenever layout, rendering, or responsive behavior could affect the result.

## Tests

End-to-end Playwright suites cover the browser-facing surface:

- Shared managed-viewer suites under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.
- Spec-viewer release suite at `memory-viewers/spec-viewer/frontend/dioxus/` (`npm run test:e2e:release`).
- Ticket-viewer release suite at `memory-viewers/ticket-viewer/frontend/dioxus/` (`npm run test:e2e:release`).
- Doc-viewer and log-viewer keep local Playwright wrappers under `memory-viewers/doc-viewer/e2e/` and `memory-viewers/log-viewer/e2e/` that import the shared suites.
