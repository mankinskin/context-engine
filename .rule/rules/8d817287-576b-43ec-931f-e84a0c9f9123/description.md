## Tool order

1. Prefer the MCP Playwright/browser tools when they cover the scenario.
2. Fall back to repo-local Playwright commands only when MCP cannot drive the page.
3. As a last resort, launch the external browser through the repo's browser-open task helpers.