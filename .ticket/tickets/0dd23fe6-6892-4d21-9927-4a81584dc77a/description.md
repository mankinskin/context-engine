## Goal
Audit the currently active execute-style MCP surfaces and adjacent terminal-execution tooling to determine whether they already support terminal reuse, follow-up input, resumable execution, or persistent session identifiers, and document the gap against the repo’s new compact-terminal direction.

## Why
The repo now has `compact-terminal-mcp`, but the current implementation is single-shot (`run` + `read_spill`) and does not obviously support sending new input to an existing terminal process or reusing a persistent terminal session. Before adding more execution tooling, we need a focused inventory of what execute-style MCP tools already expose and where the gaps are.

## References
- Context execute MCP surface: `context-stack/tools/mcp/context-mcp/src/server.rs`
- Context execute input model: `context-stack/tools/mcp/context-mcp/src/server/types.rs`
- Compact terminal MCP implementation: `tools/mcp/compact-terminal-mcp/src/server.rs`
- Compact terminal MCP docs: `tools/mcp/compact-terminal-mcp/README.md`
- HTTP execute analogue for context: `context-stack/tools/http/context-http/src/rpc.rs`

## Scope
Research and document:
- which execute-style MCP surfaces currently exist in-repo
- whether they are single-shot or session-oriented
- whether they can reuse a terminal/process/session handle
- whether they can accept follow-up input after launch
- whether they expose resumable IDs, partial output reads, or spill-file continuation only
- what the minimal next-step implementation should be if interactive terminal reuse is needed

## Implementation plan
1. Inventory execute-style tool surfaces in the repository, starting with:
   - `context-mcp` `execute`
   - `compact-terminal-mcp` `run` / `read_spill`
   - any other MCP tools whose core interaction is command execution rather than data lookup
2. For each surface, record:
   - input model
   - output model
   - execution lifetime (single-shot vs persistent)
   - support for follow-up input
   - support for reuse/resume identifiers
   - failure / timeout behavior
3. Compare the inventory against the desired interactive capabilities:
   - send more input to an existing terminal
   - reuse an existing terminal session
   - preserve cwd/environment across calls
   - inspect partial output without rerunning
4. Produce a recommendation note describing whether to extend `compact-terminal-mcp`, add a new session-oriented MCP tool, or keep the current single-shot model.
5. If concrete follow-up work is identified, propose child tickets with bounded scope.

## Acceptance criteria
- The repository has a documented capability matrix for execute-style MCP tooling.
- The matrix clearly distinguishes single-shot command execution from reusable interactive terminal sessions.
- Gaps are identified for follow-up input, terminal reuse, persistent IDs, and resumable reads.
- The ticket ends with a concrete recommendation for the next implementation slice.

## Validation notes
This is primarily a research/refinement ticket.

Evidence to collect:
- file references for each audited surface
- command/tool signatures copied from the source
- a written gap matrix or summary attached to the ticket description or follow-up notes

If code examples or prototypes are added, validate with:
- `cargo build -p compact-terminal-mcp`
- any relevant MCP contract tests added during the audit

## Risks / design notes
- Avoid inventing requirements that are not grounded in existing repo code or the current agent workflow needs.
- Keep the output decision-oriented: the end result should make the next terminal-execution ticket obvious.
- Treat this as a refinement/research ticket unless the audit reveals a tiny, clearly bounded implementation that belongs here.