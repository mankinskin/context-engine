<!-- aligned-structure:v1 -->

# Summary

Keep the PATH-first viewer workflow intact by making the shared repo installer refresh every shipped viewer binary.

## Behavior Story

Keep the PATH-first viewer workflow intact by making the shared repo installer refresh every shipped viewer binary.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Install tools refreshes viewer binaries

## Goal

Keep the PATH-first viewer workflow intact by making the shared repo installer refresh every shipped viewer binary.

## Requirements

- install-tools.sh exposes doc-viewer, log-viewer, spec-viewer, and ticket-viewer as installable tools.
- The default install set includes those viewer binaries alongside the existing CLI tools.
- The change does not alter ticket-vscode server launch precedence; PATH-installed binaries remain the preferred target.

## Related Ticket

- [d30e13e1 Install all viewer binaries](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/d30e13e1-3304-4128-9653-be7c47679f9f/ticket.toml)

## Validation

- ./install-tools.sh --tool doc-viewer --tool log-viewer --tool spec-viewer --tool ticket-viewer
- ./install-tools.sh --tool ticket-viewer (rerun after stopping stale ticket-viewer PID 8988 on Windows to release the locked PATH binary)
- ls -l "$HOME/.cargo/bin"/{doc-viewer.exe,log-viewer.exe,spec-viewer.exe,ticket-viewer.exe}
