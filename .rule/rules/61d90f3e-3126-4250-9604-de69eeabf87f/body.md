1. Lint and typecheck for each affected frontend package.
2. Nearest unit/component tests where available (for example Vitest in log-viewer frontend).
3. Browser end-to-end checks where available. For browser-hosted frontend code, first try the MCP Playwright/browser tools; if they are unavailable or insufficient for the scenario, fall back to repo-local Playwright flows (for example ticket-viewer frontend).
4. Contract checks for changed API integration paths.