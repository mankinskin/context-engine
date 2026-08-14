# Source line-ending normalization and whitespace-dirt handling

Extend `.gitattributes`, which currently contains only `*.sh text eol=lf`, to normalize tracked source files including `*.toml`, `*.rs`, and `*.md` so CRLF and whitespace-only churn does not appear as semantic dirt. Also update `.agents/instructions/commit/pre-commit.instructions.md` to state that whitespace-only churn is discarded and never committed.

The changes are needed because unnormalised `Cargo.toml` files repeatedly appeared as whitespace-only dirty diffs, consuming investigation time before the changes were discarded. The current single `.gitattributes` rule covers neither `*.toml`, `*.rs`, nor `*.md`.

Exact targets: `.gitattributes` for normalization and `.agents/instructions/commit/pre-commit.instructions.md` for the commit-handling rule. Ticket `f76169f7` is done and covered preserving line endings in generated outputs; `f76169f7` is related but does not cover tracked source-file normalization or the no-whitespace-churn commit rule.