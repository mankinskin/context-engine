# Summary

Browser-hosted frontend testing guidance must prefer MCP Playwright/browser tools before repo-local Playwright wrappers or manual browser steps.

## Requirements

- For browser-hosted frontend code, agents should first try MCP Playwright/browser tools when they are available.
- If MCP Playwright/browser tools are unavailable or insufficient for the needed validation, agents may fall back to repo-local Playwright commands or manual external-browser verification.
- Manual browser verification still uses an external Chromium-family browser, not VS Code's integrated browser.

## Traceability

- Ticket: `.ticket/tickets/fe5232d9-537a-4217-b8c0-b8e3ca81d95b`.
- Docs/instructions:
	- `.rule/rules/61d90f3e-3126-4250-9604-de69eeabf87f/body.md`
	- `memory-viewers/.rule/rules/397b0447-135e-4d35-ad05-bcc69047d2c0/body.md`
	- `AGENTS.md`
	- `memory-viewers/AGENTS.md`
	- `memory-api/AGENTS.md`
	- `viewer-api/AGENTS.md`
	- `.agents/instructions/frontend.instructions.md`
- Validation:
	- `get_errors` on the touched instruction files: no errors.
	- `git --no-pager diff -- AGENTS.md memory-viewers/AGENTS.md memory-api/AGENTS.md viewer-api/AGENTS.md .agents/instructions/frontend.instructions.md`
	- `./target/debug/rule.exe sync-targets --config rule-targets.yaml --json`
	- `./target/debug/rule.exe sync-targets --config memory-viewers/rule-targets.yaml --json`
