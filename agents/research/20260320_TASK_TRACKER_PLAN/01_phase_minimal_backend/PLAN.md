# Phase 1 — Minimal Backend

**Status:** BLOCKED (requires Phase 0 complete)

## Objective

Implement a working distributed ticket store: create, read, update, delete tickets with
dependency edges, using redb as the metadata index and the filesystem as the artifact
store. All writes must be crash-safe. The FS watcher must be live so discovered/orphaned
tickets are integrated automatically.

## Problem/Solution/Reference Baseline

1. Problem: multiple agents can race and create inconsistent local state.
Solution: strict per-ticket locks + serialized index mutations + idempotent reconcile.
Reference: concurrency goals inspired by `delightful-ai/beads-rs`.

2. Problem: operators and agents need predictable machine output for orchestration.
Solution: `ticket` CLI/HTTP contracts are JSON-first and schema-stable.
Reference: agent-first CLI posture in `Dicklesworthstone/beads_rust`.

3. Problem: running many agents in parallel without isolation increases cross-process interference risk.
Solution: define a pluggable execution backend contract with local-process default and optional Zeroboot isolation.
Reference: `zerobootdev/zeroboot` for COW sandboxing patterns (adopt as optional executor only).

## Deliverables

- [ ] `TicketFs::create(manifest, type_schema)` — atomic FS folder + redb index write
- [ ] `TicketFs::get(id)` — read manifest; validate against registered type schema
- [ ] `TicketFs::update(id, patch)` — validate state transition, atomic write, git commit
- [ ] `TicketFs::delete(id)` — soft-delete flag + remove from index
- [ ] `RedbIndexStore::add_edge(from, to, kind: String)` — open edge kind + cycle check
- [ ] `RedbIndexStore::list(filter)` — scan index with metadata predicates
- [ ] Per-ticket lock: `.ticket-lock` acquired before write, released after commit
- [ ] Short-lived global index lock for index row insertions/removals
- [ ] `FsWatcher` (notify): watches registered scan roots; on CREATED/MODIFIED triggers
      reconcile; on MOVED updates index path; on DELETED marks orphan
- [ ] `Reconciler::integrate_orphan(path)` — parse + validate; add to index or emit
      `ParseError` diagnostic
- [ ] CLI commands: `ticket create`, `ticket get`, `ticket update`, `ticket list`,
      `ticket delete`, `ticket scan` (full re-index)
- [ ] HTTP commands: same set via existing `Command` dispatch pattern

## Atomic Write Protocol

```
1. Acquire per-ticket lock (.ticket-lock via fs2)
2. Write ticket.toml + content files to temp folder (<uuid>.tmp/)
3. Begin redb write transaction (index lock acquired implicitly)
4. Rename temp folder → final UUID folder (atomic POSIX; best-effort Windows)
5. Insert/update redb index row
6. Commit redb transaction
7. git commit the changed files via git2 (history write; non-blocking on failure)
8. Release per-ticket lock
```

On crash between steps 4 and 5: `.tmp` folder present, no index row → `ticket scan`
detects and integrates or reports error.

## redb Tables (draft, finalised in Phase 0)

```rust
const TICKETS: TableDefinition<&str, &[u8]> = TableDefinition::new("tickets");
// key: uuid string, value: bincode(IndexedTicket { path, manifest_fields, type_id, ... })

const EDGES: TableDefinition<(&str, &str, &str), ()> = TableDefinition::new("edges");
// key: (from_uuid, to_uuid, kind_str), value: ()

const SCAN_ROOTS: TableDefinition<&str, &str> = TableDefinition::new("scan_roots");
// key: absolute path, value: registered label

const META: TableDefinition<&str, &str> = TableDefinition::new("meta");
// schema_version, index_root, git_repo_path, ...

const LEASES: TableDefinition<&str, &[u8]> = TableDefinition::new("leases");
// key: uuid string, value: bincode(LeaseInfo { working_by, lease_expires_at, work_intent })
```

## Cycle Detection

On `add_edge(A → B, kind)`: BFS/DFS from B; if A is reachable, reject with
`DependencyCycle` error. Run only for directed dependency-type edges; the type
definition declares whether an edge kind is acyclic-enforced.

## Key Interview Answers Applied Here

| Answer | Backend impact |
|--------|---------------|
| Q1 — Distributed FS | No single tickets/ root; index maps UUID → absolute path |
| Q2 — UUID | Folder name = UUID string; no sequential counter |
| Q4 — Open edge kinds | Edge table key includes kind as plain string |
| Q6 — Per-ticket lock | `.ticket-lock` per folder; short global lock for index ops |
| Q8 — Any attachments | `assets/` subdirectory created; index stores file list |
| Q10 — FS tracking | `FsWatcher` + `Reconciler` are Phase 1 deliverables, not deferred |

## Additional Swarm Deliverables

- [ ] Lease primitives: `ticket claim`, `ticket unclaim`, heartbeat renewal, TTL expiry handling
- [ ] Conflict visibility fields in index: `working_by`, `lease_expires_at`, `conflict_domain`
- [ ] Ready queue filter includes lease + blocker semantics
- [ ] `Executor` contract for agent work units (spawn, cancel, status, capability report)
- [ ] `LocalExecutor` implementation as default cross-platform backend
- [ ] `ZerobootExecutor` implementation behind feature/capability gate (Linux + KVM only)
- [ ] Scheduler fallback policy: if Zeroboot unavailable, route workload to `LocalExecutor`
- [ ] Persist executor metadata per lease: `executor_backend`, `sandbox_id` (if present), `spawn_started_at`

## Risks

- Windows does not guarantee atomic folder rename; document fallback behaviour.
- FS watcher events can fire multiple times for a single user operation (debounce needed).
- `notify` crate backend varies by OS (inotify / FSEvents / ReadDirectoryChangesW);
  test on all three.
- Zeroboot is currently Linux/KVM-oriented and prototype-grade; must remain optional and capability-detected.
- Some workloads need network access or multi-vCPU execution; scheduler must mark such tasks incompatible with Zeroboot and use local backend.

## TODO

- TODO: Write crash-safety integration test (kill process mid-write, verify `ticket scan` recovers).
- TODO: Define debounce window for watcher events (suggested: 200 ms).
- TODO: Confirm list filter set with first workflow definition.
- TODO: Map new ticket commands into existing `context-http` Command enum.
- TODO: Define executor capability model (`network`, `multi_vcpu`, `max_runtime_secs`) used by lease scheduler.
