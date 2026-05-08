| Benchmark | What it measures |
|---|---|
| `phase1_list_all_edges` | ReDB edge table scan (~630 edges) |
| `phase2_bfs_in_memory` | Pure in-memory BFS, no DB |
| `phase3_get_indexed_many` | Batch metadata fetch (1 ReDB transaction, 39 nodes) |
| `phase3_get_indexed_one_by_one` | Per-node fetch baseline (39 separate transactions) |
| `pipeline_full` | All 3 phases end-to-end |
| `pipeline_concurrent/{2,4,8,16,32}` | N threads barrier-synchronized |