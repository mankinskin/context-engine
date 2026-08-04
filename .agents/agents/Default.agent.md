---
name: Default
description: Describe what this custom agent does and when to use it.
argument-hint: The inputs this agent expects, e.g., "a task to implement" or "a question to answer".
tools: [vscode/askQuestions, execute, read, agent, edit, search, web, 'audit-mcp/*', 'feedback-mcp/*', 'fs-mcp/*', 'log-viewer-mcp/*', 'peek-mcp/*', 'spec-mcp/*', 'test-mcp/*', 'ticket-mcp/*']
model: "GPT-5.6 Terra"
---

<!-- MCP Tool Grant: scaffold template; context-mcp and session-mcp dropped by default per ticket cd19fed4's audit. Re-add with a one-line justification if the concrete agent role needs them. -->
