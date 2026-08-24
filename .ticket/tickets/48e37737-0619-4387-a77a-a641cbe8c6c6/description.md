# Waypoint 6: Worktree control component-pilot fixtures

## Governing specifications

- `.spec/specs/fa6e85c8-866c-4d53-bb50-b78bd651e8ce/body.md`
- `.spec/specs/191ceae7-663e-448b-bb04-46f46f38825d/body.md`
- `.spec/specs/c1d13a73-3265-42e1-8da0-5c44ef7b61ff/body.md`
- `.spec/specs/66fbd896-19d4-4bb7-898c-7cdc76375a5e/body.md`
- `.spec/specs/c40b790f-6704-4a5e-bc62-ae7599521a7c/body.md`
- `.spec/specs/a623ea02-e1a9-4c8c-81ea-f1f5fb3b4a9f/body.md`

## Owned implementation surface

- persisted W6.2 composition/criteria/edge/template fixture data for the worktree-control pilot
- W6.4 annotations applied to existing worktree control source/test items
- W6.5 health fixtures and validation evidence for the pilot

Apply the approved five-child worktree-control graph as the first evidence fixture. Do not change worktree-ctl commands, dispatch behavior, provisioning, sync, gitlink behavior, or introduce an MCP surface. The work is metadata/annotations/fixtures only, and begins after relationships/templates, annotations, and health policy exist.

## Acceptance criteria

1. The parent composes exactly the five approved immutable component ids; parent-owned ordinary criteria validate expected child shape and only the documented provider edges.
2. Persisted provider edges show CLI and provisioning consuming Git Operations, sync consuming Git Operations plus gitlink integrity, and no provisioning delegation from CLI or sync/gitlink.
3. Template bindings and source annotations resolve against the v2 model; valid annotations are limited to supported existing items, with no local-variable claims.
4. Health and focused existing worktree tests demonstrate the composed fixture without changing its operational behavior.

## Focused validation

- `cargo test --manifest-path workflow-tools/session/Cargo.toml -p worktree-ctl`
- `cargo test --manifest-path workflow-tools/session/Cargo.toml -p session-worktree-provision`
- `./target/debug/spec.exe --workspace . health --all`
- fixture assertions for exact composition/edges, annotations, and no provisioning-delegation claim

## Done condition

The approved pilot is a health-valid v2 composition/evidence fixture and existing worktree behavior is unchanged.