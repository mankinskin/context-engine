## Quality Gates

- Tests relevant to your change must pass before completion.
- **Browser verification is mandatory** for any change to a server interface or frontend feature:
  open the affected viewer in the browser and confirm the feature works visually before marking work done.
- **Write Playwright end-to-end tests** for all browser-facing features and server interface changes.
  E2E tests live under `tools/viewer/e2e/`. Run them with `npx playwright test` from that directory.
- For tracing-based tests, use: