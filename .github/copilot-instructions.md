
# GitHub Copilot Instructions

This file is intentionally minimal.

## Source of Truth

All behavioral and workflow guidance lives in [AGENTS.md](../AGENTS.md).
Path-scoped guidance lives in [.agents/instructions/](../.agents/instructions/).
Workflow prompts live in [.agents/prompts/](./prompts/).

## Optional Copilot CLI MCP Config

Repository-local Copilot CLI MCP config guidance lives alongside the relevant tool README files:

See [context-mcp README](../context-stack/tools/mcp/context-mcp/README.md) for the current repository-local Copilot CLI MCP config example.

## Hooks

Hook reminders are configured in [.github/hooks/](./hooks/).

# RTK — Token-Optimized CLI

**rtk** is a CLI proxy that filters and compresses command outputs, saving 60-90% tokens.

## Install

```bash
cargo install --git https://github.com/rtk-ai/rtk
```

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
