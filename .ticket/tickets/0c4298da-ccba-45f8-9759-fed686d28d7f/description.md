## Problem

`COMMANDS.md` line 25 has a `## ticket-cli` heading and line 29 declares source `memory-api/tools/cli/ticket-cli`. `memory-api/README.md` line 9 links `tools/cli/ticket-cli/README.md`; line 106 installs `tools/cli/ticket-cli --bin ticket`; lines 110 and 127 link the same missing README.

All cited paths name a directory that no longer exists. The ticket tool is now the public `memory-api/crates/ticket` crate with feature-gated `ticket`, `ticket-mcp`, and `ticket-http` binaries. The current prose sends users to dead links and invalid install commands.

## Required State

Update the heading, source paths, links, and installation guidance to identify the `ticket` crate at `memory-api/crates/ticket` and explain its feature-gated binaries. `spec-cli` at `memory-api/tools/cli/spec-cli/` remains live and must not be changed.

Related migration tickets: `ba4aaa9c` and `0da6894c`.
