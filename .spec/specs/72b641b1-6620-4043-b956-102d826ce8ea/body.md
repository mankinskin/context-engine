<!-- aligned-structure:v2 -->

# Meta-workspace Workflow-Tools Consumer Topology

## Target Code Location

- [meta-workspace .gitmodules](../../../../.gitmodules) defines the top-level source and consumer checkouts.
- [context-engine .gitmodules](../../../.gitmodules) must no longer define a `workflow-tools` submodule after the consumer cutover.
- [workflow-tools session tooling](../../../../workflow-tools/session) owns workspace selection and worktree behavior for installed tools.
- [workflow hook manifest](../../../.github/hooks/hooks.json) owns workspace-relative hook command resolution.

## Naming Conventions

- `meta-workspace` is the superproject.
- `workflow-tools` is the sole top-level tooling source checkout.
- `minimal-demo` is a top-level consumer checkout and fixture.
- A `consumer workspace root` is the explicit directory whose stores, configuration, and source tree an installed workflow tool may access.

## Requester Input

> The meta-workspace should be the superproject for the workflow-tools submodule and multiple consumer workspaces, including the minimal demo workspace. The tools need to support working from a superproject and accessing nested workspaces needs to work as expected, without accidentally working in the wrong workspace.

## Reading Order

1. [Minimal external workflow-tools consumer](../bc639ab3-8eda-4268-a7a2-34289bfeba4d/body.md) - installation proof provided by `minimal-demo`.
2. [context-engine consumer cutover](../../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml) - removes the vendored context-engine tooling checkout.
3. [topology implementation ticket](../../../.ticket/tickets/389f90d9-fb06-49b7-948b-0dbd14dcfeca/ticket.toml) - implements this contract.
4. [worktree protocol](../../../.agents/instructions/commit/branch-worktree.instructions.md) - governs session worktree isolation.

## Responsibility

The meta-workspace topology provides one canonical `workflow-tools` checkout and independent consumer checkouts. Installed workflow tools must target an explicitly selected consumer workspace, never infer the consumer from a superproject CWD or from the location of the installed binary.

If this specification is implemented, a caller can invoke a workflow tool from the meta-workspace root or a consumer subdirectory and rely on the selected consumer workspace receiving all reads, writes, hooks, and validation.

## Interfaces And Dependencies

- The superproject registers `workflow-tools`, `context-engine`, `minimal-demo`, and future consumers as sibling submodules.
- Each tool invocation accepts or receives a consumer-root selector and resolves stores only beneath that selected root.
- Hook commands resolve from the superproject CWD without assuming the active consumer is `context-engine`.
- context-engine consumes published or installed workflow tools and no longer vendors `workflow-tools` as a nested submodule.

## Behavior

1. The superproject has exactly one top-level `workflow-tools` submodule.
2. `minimal-demo` is an independent top-level consumer submodule, not a child of `workflow-tools`.
3. A workflow operation against a consumer uses an explicit consumer-root path or an unambiguous consumer-local CWD.
4. A workflow operation launched from the superproject root requires an explicit consumer-root selector; it must not silently target `context-engine`.
5. A tool rejects a missing, nonexistent, or ambiguous consumer-root selection before it reads or mutates stores.

## Boundaries And Failure Cases

- The superproject is an orchestration container, not the default ticket, spec, session, or workflow store owner.
- `workflow-tools` source code is not evidence that a consumer's installation contract works.
- A consumer may not access another consumer's stores through parent-directory discovery.
- Nested session worktrees inherit their assigned consumer root; an outer meta-workspace path does not authorize mutation of a sibling consumer.
- Missing `minimal-demo` initialization, a duplicate `workflow-tools` checkout, or an ambiguous root selector must produce an actionable error rather than a fallback target.

## Provider/Consumer Contract

- `minimal-demo` consumes the version-pinned installer contract owned by [Minimal external workflow-tools consumer](../bc639ab3-8eda-4268-a7a2-34289bfeba4d/body.md).
- context-engine consumes the same installed tools through [context-engine consumer cutover](../../../.ticket/tickets/92741a14-d718-4f49-8843-040432a3d8da/ticket.toml).
- The topology implementation provides consumer-root selection to both consumers.

## Examples

From the meta-workspace root, a ticket command for the demo names `minimal-demo` as its workspace root and reads or writes only `minimal-demo/.ticket`. The same command without a consumer-root selector fails instead of using `context-engine/.ticket` because context-engine is merely a sibling consumer.

## Evidence

- A topology test creates or initializes the top-level `minimal-demo` checkout and confirms it is independent from `workflow-tools`.
- Tool and hook integration tests run from the meta-workspace root with both `minimal-demo` and `context-engine` present, then read back only the selected consumer's store artifact.
- A negative test proves an unqualified command from the meta-workspace root fails without modifying either consumer store.
- The final cutover check confirms [context-engine .gitmodules](../../../.gitmodules) contains no `workflow-tools` entry.

## Positions

- [meta-workspace .gitmodules](../../../../.gitmodules): partial - contains top-level `workflow-tools` and context-engine but no `minimal-demo` consumer.
- [context-engine .gitmodules](../../../.gitmodules): partial - still vendors `workflow-tools`.
- [workflow-tools session tooling](../../../../workflow-tools/session): partial - supports workspace paths but has no superproject consumer-selection contract.
- [workflow hook manifest](../../../.github/hooks/hooks.json): partial - resolves scripts from the meta-workspace root but does not select a consumer root.

## Guards

- `cargo test -p memory-kernel` covers consumer-root resolver behavior after the topology API is added.
- `bash workflow-tools/fixtures/minimal-demo/run-tutorial.sh` proves clean installation against the top-level consumer checkout after the fixture exists.
- A meta-workspace integration script verifies selected-store read-back and unqualified-command rejection.

## Governing Rule

[AGENTS.md](../../../AGENTS.md) requires explicit worktree and entity-store targeting; this specification extends that rule from session worktrees to sibling consumer workspaces.

## Scope

This specification covers repository topology and tool workspace selection. It does not define the tool bundle contents, artifact-store migration mechanics, or the full context-engine consumer migration.
