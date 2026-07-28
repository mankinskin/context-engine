## Validation Evidence

Recorded at [cd19fed4](.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0/ticket.toml) review (2026-07-28):

- 16/16 `.agents/agents/*.agent.md` templates checked; verified: 14 carry a
  literal `## MCP Tool Grant` markdown heading, `default.agent.md` carries the
  equivalent contract as an HTML comment (`<!-- MCP Tool Grant: ... -->`), and
  `orchestrator.agent.md` legitimately needs none — its only tool is the
  sub-agent dispatch tool (no direct MCP access at all). Every wildcard grant
  spot-checked (audit/commit/default/explore/handoff/implement/interview/
  iteration/orchestrator/research/roast/spec/testing/ticket-refinement/
  transcription) carries a one-line role justification.
- AC4 (`tool_search` availability) resolved as documented-unavailable: VS Code
  client-level mechanism, not repo-controlled. See
  `.agents/prompts/tool-grant-regression-probe.prompt.md`.
- AC5 (regression-probe method) confirmed in the same prompt file's
  Measurement Log table: consistent self-report method, before/after tool
  counts and derived schema-token estimates, 60-tool / 10k-token drift gates.
- 16/16 templates carry a `model:` frontmatter field (`66acb737`), confirmed
  by `git grep`; the contract for that field lives in
  `.agents/instructions/orchestration/model-routing.instructions.md` (line
  111 onward), not in this spec — matching this spec's Non-goals boundary.

Recommended and confirmed: `cd19fed4` and `66acb737` both `done`, both
`review_verdict = pass`.
