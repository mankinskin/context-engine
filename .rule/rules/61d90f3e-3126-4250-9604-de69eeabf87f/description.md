1. Lint and typecheck for each affected frontend package.
2. Nearest unit/component tests where available (for example Vitest in log-viewer frontend).
3. Browser end-to-end checks where available (for example Playwright flows in ticket-viewer frontend).
4. Contract checks for changed API integration paths.