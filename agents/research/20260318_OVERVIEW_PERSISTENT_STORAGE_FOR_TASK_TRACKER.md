# Status: TODO

# Persistent Data Storage For Hyperlinked Task/Ticket Systems In Rust

## Objective

Research Rust-accessible persistent storage systems for a task tracker with:
- Task dependencies and rich references
- Rich text descriptions, checkboxes, progress states
- Ticket validation and highlighting
- Incremental ticket refinement
- Transactional updates
- Versioning/history
- CLI + HTTP interfaces
- Hybrid artifacts per ticket (JSON/TOML/text/images/binary)

## Existing System Landscape (Rust, 2026)

### Category A: Embedded SQL
- SQLite via `rusqlite`
- libSQL/Turso ecosystem (SQLite-compatible + distributed/sync-focused deployments)

Strengths:
- Mature ACID transactions
- Strong query language for dependencies/workflows
- Rich ecosystem (migrations, FTS, JSON handling)

Tradeoffs:
- File/folder artifact handling still needs a sidecar filesystem strategy
- Schema design discipline required for evolving ticket models

### Category B: Embedded KV / B-Tree / LSM
- `redb` (pure Rust, ACID, MVCC)
- `heed` (typed LMDB wrapper)
- `rocksdb` (feature-rich LSM via C++ library)
- `fjall` (Rust LSM KV engine)
- `sled` (KV with ACID tx, but warns about maturity/stability concerns in its own docs)

Strengths:
- Excellent for high-throughput key-value workloads
- Good fit for event journals, materialized caches, index tables
- Can store compact binary payloads and blobs efficiently

Tradeoffs:
- No first-class relational query language
- More application-level query/index design work
- Some engines have process model constraints (single-process/opening rules)

### Category C: Filesystem-First + Metadata Index
- Ticket as directory with required/optional files
- JSON/TOML/text/image assets in canonical structure
- Use a DB for metadata/search/dependencies/history pointers

Strengths:
- Human-inspectable data model
- Easy attachment and mixed content handling
- Interoperable with git and external tooling

Tradeoffs:
- Must solve consistency between file tree and metadata index
- Requires explicit transaction boundary design for FS + DB dual writes

### Category D: Search/Index Sidecars
- `tantivy` for full-text search and ranking

Strengths:
- Powerful search and scoring across ticket content
- Fast incremental indexing

Tradeoffs:
- Additional index lifecycle and reindex policies
- Eventual consistency considerations if asynchronously updated

## Fit Against Requested Capabilities

| Capability | Best-Primary Fit | Notes |
|---|---|---|
| Transactional updates | SQLite/libSQL, redb, LMDB/heed, RocksDB TxDB | SQL gives strongest out-of-box workflow modeling |
| Folder-structured mixed assets | Filesystem-first hybrid | DB-only can store blobs, but folder UX is better for humans/tools |
| Rich references/dependencies | SQL relational model | KV works but requires custom graph/index layers |
| Versioning/history | Event log + snapshots, or git-backed history | Most robust with append-only journal and periodic compaction |
| CLI + HTTP adapters | Existing project already demonstrates this pattern | Keep storage layer behind commands/service trait |
| Validation + schema rules | JSON Schema/serde-validated manifests + DB constraints | Hybrid validation pipeline recommended |
| Search/highlighting | Tantivy side index | Keep primary store normalized and index derived |

## Recommended Architecture Direction (Initial)

1. Hybrid model: filesystem tickets + embedded SQL metadata store.
2. Use per-ticket folder as canonical artifact container.
3. Keep relational metadata DB for dependencies, state transitions, references, and query workloads.
4. Add append-only event journal for history/audit and deterministic reconstruction.
5. Add Tantivy side-index for full-text and highlighting across textual assets.

## Candidate Primary Stacks

### Stack 1 (most balanced)
- Primary metadata store: SQLite (`rusqlite`)
- Artifact store: filesystem folders
- Search: Tantivy
- History: event journal table + snapshot checkpoints

### Stack 2 (all-Rust embedded KV-centric)
- Primary metadata store: `redb`
- Artifact store: filesystem folders
- Search: Tantivy
- History: append-only event stream in redb tables

### Stack 3 (distributed-forward)
- Primary metadata store: libSQL/Turso family
- Artifact store: filesystem or object-store abstraction
- Search: Tantivy local or service-backed index
- History: SQL event journal + branch/version model

## Online Source Highlights

- redb docs/repo: ACID, MVCC, savepoints/rollbacks, stable format claim
  - https://docs.rs/redb/latest/redb/
  - https://github.com/cberner/redb
- sled docs/repo: transactions and watchers; README warns about beta/format changes
  - https://docs.rs/sled/latest/sled/
  - https://github.com/spacejam/sled
- RocksDB Rust bindings: transactions, column families, checkpoints, compression
  - https://docs.rs/rocksdb/latest/rocksdb/
  - https://github.com/rust-rocksdb/rust-rocksdb
- heed (LMDB wrapper): typed LMDB transactions and iterators
  - https://docs.rs/heed/latest/heed/
  - https://github.com/meilisearch/heed
- SQLite WAL model/concurrency notes:
  - https://www.sqlite.org/wal.html
- rusqlite bindings/features:
  - https://github.com/rusqlite/rusqlite
- Tantivy search engine in Rust:
  - https://docs.rs/tantivy/latest/tantivy/
- git/libgit2 foundations for object history and version semantics:
  - https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain
  - https://docs.rs/git2/latest/git2/

## Open TODO Research Questions

- TODO: Define canonical ticket folder schema versions (v1/v2 migration strategy).
- TODO: Decide whether metadata DB is source of truth or projection of filesystem manifests.
- TODO: Define atomicity contract for FS + DB dual writes (two-phase, WAL, compensation).
- TODO: Benchmark read/write/search workloads for realistic ticket volumes.
- TODO: Define cross-ticket reference model (ID-only, path-based, or hybrid references).
- TODO: Decide conflict policy for multi-user edits (single-writer lock vs optimistic merge).
