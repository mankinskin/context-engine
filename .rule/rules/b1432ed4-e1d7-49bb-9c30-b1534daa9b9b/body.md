## Index Reconciliation (`scan --force`)

`scan` normally only integrates new/changed files it discovers. Use
`scan --force` to force a full reconciliation — every ticket.toml is re-read
from disk and both the SQLite index and Tantivy search index are rebuilt: