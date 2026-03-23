# Status: TODO

# Embedded KV Options In Rust: redb, heed/LMDB, RocksDB, Fjall, sled

## Scope

Evaluate key-value engines for metadata, event logs, secondary indexes, and binary payload storage.

## redb

### Summary
- Pure Rust embedded KV store.
- ACID transactions, MVCC, savepoints/rollbacks.
- Project states stable/maintained and stable file format intent.

### Fit
- Excellent candidate for all-Rust embedded core.
- Good for append-only event streams + materialized views.

### Concerns
- Less query ergonomics than SQL; requires custom indexing patterns.

### Sources
- https://docs.rs/redb/latest/redb/
- https://github.com/cberner/redb

## heed (LMDB wrapper)

### Summary
- Typed LMDB wrapper with low overhead.
- Strong transactional and iteration semantics.

### Fit
- Great for high-performance read-heavy maps and ordered scans.

### Concerns
- LMDB operational model (single-writer constraints) must be explicitly designed around.

### Sources
- https://docs.rs/heed/latest/heed/
- https://github.com/meilisearch/heed

## RocksDB (via rust-rocksdb)

### Summary
- Rich LSM engine with column families, checkpoints, compression, transaction APIs.

### Fit
- Strong for large-scale write-heavy workloads and layered indexes.

### Concerns
- Native dependency/toolchain complexity (Clang/LLVM, C++ bindings).
- Operational tuning surface is larger.

### Sources
- https://docs.rs/rocksdb/latest/rocksdb/
- https://github.com/rust-rocksdb/rust-rocksdb

## Fjall

### Summary
- Rust log-structured KV engine with keyspaces and optional serializable tx modes.
- Notes multi-process/open constraints in docs.

### Fit
- Interesting modern Rust-native LSM option.

### Concerns
- Need deeper validation for maturity and migration guarantees under your workload.

### Sources
- https://github.com/fjall-rs/fjall

## sled

### Summary
- Embedded KV with ACID tx, watch prefixes, merge operators.

### Fit
- Fast prototyping and event-driven index updates.

### Concerns
- README itself warns on beta status and on-disk format instability concerns.

### Sources
- https://docs.rs/sled/latest/sled/
- https://github.com/spacejam/sled

## Comparative Fit Table

| Engine | Transaction Strength | Maturity Signal | Operational Complexity | Best Use Here |
|---|---|---|---|---|
| redb | High (ACID + MVCC) | Strong | Low-Medium | Primary metadata/event store (Rust-native) |
| heed/LMDB | High | Strong LMDB heritage | Medium | Read-heavy ordered indexes |
| RocksDB | High (with TxDB modes) | Strong | High | Heavy write + large scale indexes |
| Fjall | Medium-High | Emerging | Medium | Rust-native LSM exploration |
| sled | Medium-High | Mixed (beta warnings) | Low-Medium | Prototype only |

## TODO

- TODO: Implement microbench harness for ticket-like workload patterns.
- TODO: Validate crash-recovery behavior under power-loss simulations.
- TODO: Evaluate backup/restore and compaction operational workflows.
- TODO: Decide if non-SQL KV requires custom query DSL layer.
