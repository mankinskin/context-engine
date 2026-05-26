<!-- spec-api:file generated=true -->

<!-- spec-api:entry id=ef02dffa-16e2-42ec-9ea1-8494cbf63a33 slug=context-engine/recurring-principles/browser-validation/browser-validation-contract/l1 -->
# Browser validation contract

Any change to a server interface or to a frontend feature must be verified in an external Chromium-family browser before the work moves to `in-review`. VS Code's integrated browser is not a substitute and must not be used for this gate.

<!-- spec-api:entry id=8d817287-576b-43ec-931f-e84a0c9f9123 slug=context-engine/recurring-principles/browser-validation/browser-validation-contract/tool-order/l5 -->
## Tool order

1. Prefer the MCP Playwright/browser tools when they cover the scenario.
2. Fall back to repo-local Playwright commands only when MCP cannot drive the page.
3. As a last resort, launch the external browser through the repo's browser-open task helpers.

<!-- spec-api:entry id=bdb369eb-4199-40f1-b83b-e2d45def1426 slug=context-engine/recurring-principles/browser-validation/browser-validation-contract/evidence/l11 -->
## Evidence

- Capture screenshots for UI-facing changes so the rendered state is visually confirmed and not only inferred from DOM assertions.
- For modals, overlays, drawers, popovers, and menus, include at least one screenshot with the surface open, and a before/after pair when useful.
- Record the browser window or display resolution used whenever layout, rendering, or responsive behavior could affect the result.

<!-- spec-api:entry id=e18b66cf-4718-4096-a2e7-7e5b68440e35 slug=context-engine/recurring-principles/browser-validation/browser-validation-contract/tests/l17 -->
## Tests

End-to-end Playwright suites cover the browser-facing surface:

<!-- spec-api:entry id=f9783545-b05f-415e-bd4d-20c3600244dd slug=context-engine/recurring-principles/browser-validation/browser-validation-contract/tests/l21 -->
- Shared managed-viewer suites under `memory-viewers/viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.
- Spec-viewer release suite at `memory-viewers/spec-viewer/frontend/dioxus/` (`npm run test:e2e:release`).
- Ticket-viewer release suite at `memory-viewers/ticket-viewer/frontend/dioxus/` (`npm run test:e2e:release`).
- Doc-viewer and log-viewer keep local Playwright wrappers under `tools/viewer/doc-viewer/e2e/` and `tools/viewer/log-viewer/e2e/` that import the shared suites.
