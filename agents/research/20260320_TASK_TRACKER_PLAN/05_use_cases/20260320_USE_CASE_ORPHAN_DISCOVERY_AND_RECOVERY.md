# Use Case: Orphan Discovery and Recovery

## Goal

Automatically detect ticket folders introduced by external moves/copies and integrate them into the index, with explicit diagnostics when parsing fails.

## Preconditions

- Filesystem watcher is active.
- Ticket parser and schema registry are loaded.
- Reconciliation queue supports retries.

## Scenario

1. User copies a ticket folder into a watched root manually.
2. Watcher emits CREATED/MOVED events.
3. Reconciler parses `ticket.toml` and discovers UUID.
4. If valid, index entry is created and dependency links are resolved.
5. If invalid, reconciler creates parse diagnostic with file path and error span.
6. User fixes schema issues; watcher detects update and retries integration.

## Data Flows

- FS events -> reconcile queue -> parser -> index upsert.
- Diagnostics store tracks unresolved parse or schema errors.
- Search excludes invalid tickets by default but can include them with `has:errors` filter.

## Concurrency Rules

- Reconcile worker shard by ticket UUID to avoid duplicate processing.
- Integration writes use per-ticket lock once UUID is known.
- Idempotent upsert prevents duplicates from duplicate watcher events.

## Failure Modes

- Folder contains duplicate UUID already indexed elsewhere: create collision diagnostic.
- Missing mandatory fields (`id`, `created_at`): reject and keep diagnostic open.
- Massive bulk imports flood queue: apply backpressure and chunked scanning.

## Success Metrics

- Mean time to integrate valid orphan tickets.
- Parse error resolution time.
- Duplicate UUID collision frequency.
