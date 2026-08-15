## Problem

The repository dependency graph currently contains a cycle that blocks two
per-tool extractions:

```
test-cli  -> log-api    (log_api::{ValidationLogKind, ...} used by the `test` CLI)
log-api   -> test-api   (test_api::{ValidationExecution, ValidationLinks, traits})
```

Once `test` and `log` become separate repositories, this becomes a repository-level
cycle `test -> log -> test`, which git-URL dependency resolution cannot express.
Ticket `e8a5c061` already mandates a documented remediation decision before either
extraction proceeds; this ticket **is** that decision.

## Measured coupling

After the neutral traits are promoted to the kernel (ticket `4cbe11d3`), the only
remaining `log-api -> test-api` edges are three convenience adapters in
`memory-api/crates/log-api/src/lib.rs`:

- `impl From<&ValidationExecution> for ValidationLogLinks`
- `impl From<ValidationLinks> for ValidationLogLinks`
- `ValidationLogCapture::from_execution(id, execution, kind, captured_at, media_type, locator)`

Every one of them merely copies `spec_ids`, `acceptance_criterion_ids`,
`ticket_ids`, `doc_evidence_ids`, and the execution id into log-owned fields.
`ValidationLogCapture` itself stores `validation_execution_id: String` — a plain
id, not a test-domain type — so the log store has no structural need for
`test-api` at all.

## Decision

Cut the `log-api -> test-api` edge. `log-api` depends on `memory-kernel` only.
The execution-to-log-links adapter moves to the test side, which already depends
on both.

Orphan-rule note: the adapters cannot simply move as `From` impls, because
`impl From<Local> for Foreign` is rejected — the `Self` type (`ValidationLogLinks`)
is checked first and would be foreign in the test crate. The adapters therefore
become free functions in the test domain crate, and `log-api` gains a plain
constructor that takes the already-flattened link ids.

## Resulting graph

```
memory-kernel
  ^        ^
  |        |
log-api  test-api
  ^        ^
  |        |
  +-- test (cli/mcp) --+
```

Acyclic: `test -> log`, one direction only.

## Rejected alternatives

- **Move `ValidationLogLinks` into `memory-kernel`.** Rejected: the type is
  log-domain-specific (it names doc-evidence and validation-execution ids), and the
  kernel is a neutrality-scoped layer.
- **Invert to `test-api -> log-api` and keep the `From` impls.** Rejected: orphan
  rules forbid the impls in the inverted position, so the inversion buys nothing.
- **Merge `log-api` into `test-api`.** Rejected: the log tool has its own store,
  viewer, and extraction ticket (`2736c3dc`); merging would collapse two tools
  into one repository against the per-tool split.
- **Keep both in one repository.** Rejected: it would permanently exempt two of
  the eleven tools from the per-tool split tracked by `858c5286`.

## Non-goals

- No change to on-disk log or test record formats.
- No repository extraction — this only makes the extractions unblockable.
