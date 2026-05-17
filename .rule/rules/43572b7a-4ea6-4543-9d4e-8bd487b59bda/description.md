1. Static checks: run lint and typecheck for each affected frontend package.
2. Component checks: run nearest unit/component tests for changed UI modules.
3. Browser checks: run at least one browser flow that exercises changed UX paths in an external fullscreen Chromium-family browser rather than VS Code's integrated browser.
4. Integration checks: verify viewer interaction with context-api and ticket-api contracts, or filesystem-backed endpoints, for changed paths.
5. Regression checks: when fixing a bug, include a reproducer assertion before or with the fix.
6. Evidence summary: report commands run, pass/fail outcome, which UX states were validated, and the browser window or display resolution used for any manual visual check.