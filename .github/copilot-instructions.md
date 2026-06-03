<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=604a8966-cd45-4403-b3fb-90777e0f7d8a slug=shared/copilot-instructions/github-copilot-instructions/l1 -->
# GitHub Copilot Instructions

This file is intentionally minimal.

<!-- rule-api:entry id=53503cbf-4d68-42ef-91f8-fa35a3c3095b slug=shared/copilot-instructions/github-copilot-instructions/source-of-truth/l5 -->
## Source of Truth

All behavioral and workflow guidance lives in [AGENTS.md](../AGENTS.md).
Path-scoped guidance lives in [.agents/instructions/](../.agents/instructions/).
Workflow prompts live in [.github/prompts/](./prompts/).

<!-- rule-api:entry id=a13bf9ab-3dd8-488d-8dc6-66f97e75aaf6 slug=shared/copilot-instructions/github-copilot-instructions/optional-copilot-cli-mcp-config/l11 -->
## Optional Copilot CLI MCP Config

Repository-local Copilot CLI MCP config guidance lives alongside the relevant tool README files:

<!-- rule-api:entry id=d9cce506-ef29-4217-abed-f3042ff48d6c slug=shared/copilot-instructions/github-copilot-instructions/optional-copilot-cli-mcp-config/l15 -->
See [context-mcp README](../context-stack/tools/mcp/context-mcp/README.md) for the current repository-local Copilot CLI MCP config example.

<!-- rule-api:entry id=1c0066e5-ce52-49e0-b7ca-1685785882ac slug=shared/copilot-instructions/github-copilot-instructions/hooks/l19 -->
## Hooks

Hook reminders are configured in [.github/hooks/](./hooks/).

<!-- rule-api:entry id=46fe5339-b0ac-4cf2-ac47-a23af84cbf74 slug=shared/copilot-instructions/github-copilot-instructions/rtk-token-optimized-cli/l23 -->
# RTK — Token-Optimized CLI

**rtk** is a CLI proxy that filters and compresses command outputs, saving 60-90% tokens.

## Rule

Always prefix shell commands with `rtk`:

```bash
# Instead of:              Use:
git status                 rtk git status
git log -10                rtk git log -10
cargo test                 rtk cargo test
docker ps                  rtk docker ps
kubectl get pods           rtk kubectl pods
```

When a repository CLI supports `--toon`, prefer `rtk <cmd> --toon ...` over `rtk <cmd> --json ...` for compact machine-readable output. Use the `toon-format` / `toon-rust` codec for encoding and decoding TOON instead of hand-rolled text transforms.

## Meta commands (use directly)

```bash
rtk gain              # Token savings dashboard
rtk gain --history    # Per-command savings history
rtk discover          # Find missed rtk opportunities
rtk proxy <cmd>       # Run raw (no filtering) but track usage
```
