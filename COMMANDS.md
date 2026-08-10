# Command & Hook Registry

Generated from `tools/install/artifacts.toml` (schema version 1). Do not edit by hand.

## mcp-toolmon

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/mcp-toolmon`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `mcp-toolmon`
- Lifecycle: Install, Inspect

## session-capture-hook

- Category: Misc
- Kind: RustBinary
- Source: `memory-api/crates/session-capture-hook`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session-capture-hook`
- Lifecycle: Install, Inspect

## ticket-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/ticket-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket`
- Lifecycle: Install, Inspect

## spec-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/spec-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec`
- Lifecycle: Install, Inspect

## audit-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/audit-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `audit`
- Lifecycle: Install, Inspect

## rule-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/rule-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `rule`
- Lifecycle: Install, Inspect

## feedback-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/feedback-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `feedback`
- Lifecycle: Install, Inspect

## session-cli

- Category: Cli
- Kind: RustBinary
- Source: `memory-api/tools/cli/session-cli`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session`
- Lifecycle: Install, Inspect

## context-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `context-stack/tools/mcp/context-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `context-mcp`
- Lifecycle: Install, Inspect

## ticket-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/ticket-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket-mcp`
- Lifecycle: Install, Inspect

## spec-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/spec-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec-mcp`
- Lifecycle: Install, Inspect

## test-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/test-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `test-mcp`
- Lifecycle: Install, Inspect

## feedback-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/feedback-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `feedback-mcp`
- Lifecycle: Install, Inspect

## session-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/session-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `session-mcp`
- Lifecycle: Install, Inspect

## peek-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/peek-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `peek-mcp`
- Lifecycle: Install, Inspect

## rule-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/rule-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `rule-mcp`
- Lifecycle: Install, Inspect

## audit-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/audit-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `audit-mcp`
- Lifecycle: Install, Inspect

## fs-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/fs-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `fs-mcp`
- Lifecycle: Install, Inspect

## compact-terminal-mcp

- Category: Mcp
- Kind: RustBinary
- Source: `memory-api/tools/mcp/compact-terminal-mcp`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `compact-terminal-mcp`
- Lifecycle: Install, Inspect

## doc-viewer

- Category: Service
- Kind: RustBinary
- Source: `memory-viewers/doc-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `doc-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## log-viewer

- Category: Service
- Kind: RustBinary
- Source: `memory-viewers/log-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `log-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## spec-viewer

- Category: Service
- Kind: RustBinary
- Source: `memory-viewers/spec-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `spec-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## ticket-viewer

- Category: Service
- Kind: RustBinary
- Source: `memory-viewers/ticket-viewer`
- Owner: tooling
- Safety: ApprovalRequired
- Bin: `ticket-viewer`
- Lifecycle: Install, Start, Stop, Restart, Uninstall, Inspect

## ticket-vscode

- Category: VscodeExtension
- Kind: VscodeExtension
- Source: `memory-api/tools/ticket-vscode`
- Owner: tooling
- Safety: ApprovalRequired
- Npm script: `install:vsix`
- Lifecycle: Install, Inspect

## worktree-ctl

- Category: Misc
- Kind: RustBinary
- Source: `tools/worktree/worktree-ctl`
- Owner: tooling
- Safety: Safe
- Bin: `worktree-ctl`
- Lifecycle: Inspect

## storeless-startup-matrix

- Category: Misc
- Kind: Script
- Source: `tools/validate-storeless-startup.sh`
- Owner: tooling
- Safety: Safe
- Bin: `bash tools/validate-storeless-startup.sh [--toon]`
- Lifecycle: Inspect

## hook-copilot-capture

- Category: Hook
- Kind: Hook
- Source: `memory-api/crates/session-api`
- Owner: tooling
- Safety: Safe
- Bin: `session-capture-hook --from-hook-stdin`
- Lifecycle: Inspect

## hook-rtk-hook-copilot

- Category: Hook
- Kind: Hook
- Source: `tools/agent-hooks/rtk-hook-copilot.sh`
- Owner: tooling
- Safety: Safe
- Bin: `bash tools/agent-hooks/rtk-hook-copilot.sh`
- Lifecycle: Inspect

## hook-preflight-write

- Category: Hook
- Kind: Hook
- Source: `tools/agent-hooks/preflight-write.sh`
- Owner: tooling
- Safety: Safe
- Bin: `bash tools/agent-hooks/preflight-write.sh`
- Lifecycle: Inspect

## hook-validate-docs

- Category: Hook
- Kind: Hook
- Source: `tools/agent-hooks/validate-docs.sh`
- Owner: tooling
- Safety: Safe
- Bin: `bash tools/agent-hooks/validate-docs.sh`
- Lifecycle: Inspect

## hook-terminal-pwd

- Category: Hook
- Kind: Hook
- Source: `tools/agent-hooks/terminal-pwd.sh`
- Owner: tooling
- Safety: Safe
- Bin: `bash tools/agent-hooks/terminal-pwd.sh`
- Lifecycle: Inspect
