---
description: "Welcome the user to the workspace, explain the available workflow tools, or help bootstrap a fresh checkout."
name: "memory-setup"
argument-hint: "[fresh-checkout|current-workspace|tools]"
agent: "agent"
---

# Memory Setup

Help the user orient in the current checkout or bootstrap a fresh one without skipping the repository workflow.

Reference [README](../../README.md), [AGENTS](../../AGENTS.md), [rule-cli](../../memory-viewers/memory-api/tools/cli/rule-cli/README.md), [spec-cli](../../memory-viewers/memory-api/tools/cli/spec-cli/README.md), [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md), [audit-cli](../../memory-viewers/memory-api/tools/cli/audit-cli/README.md), and [viewer-ctl](../../memory-viewers/viewer-api/viewer-ctl/README.md).

## Workflow

1. Read the slash-command text and determine whether the user wants:
- a fresh checkout setup
- current workspace orientation
- a tour of the available workflow tools
2. Inspect the current repository layout and discover the nearest `.ticket`, `.spec`, and `.rule` stores before giving setup advice.
3. For a fresh checkout, guide the user through the minimum useful bootstrap commands:
- build or install `rule`, `spec`, `ticket`, and `audit`
- identify any viewer or browser helpers worth starting
- call out missing prerequisites explicitly instead of assuming they are installed
4. For an existing workspace, summarize the relevant workflow surfaces:
- ticket planning and board state
- spec authoring and health checks
- rule-backed generated guidance
- documentation and log viewers
- focused validation and audit tooling
5. Prefer concrete commands and paths over generic setup advice.
6. Ask one concise clarification only when the desired setup mode is still ambiguous after a focused inspection.

## Response

Return a concise setup summary containing:
- detected workspace state
- available workflow tools and what each is for
- missing prerequisites or binaries, if any
- the single best next action for the user