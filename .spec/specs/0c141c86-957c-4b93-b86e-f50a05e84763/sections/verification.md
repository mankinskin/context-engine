# Verification protocol

Future `crane-cli` use should pass this sequence before a production import for a new migration shape.

## 1. Local tool validation

Run the focused crate tests first.

```bash
cargo test -p crane-cli
```

This must cover at least one end-to-end temp-repo transplant with path remapping.

## 2. Real-source dry run

Run `crane-cli transplant --dry-run` against the actual source and target repositories for the intended mapping set.

The dry-run review must confirm:

- the selected mappings are the exact intended source trees;
- the anchor commit is the first relevant commit for that combined path set;
- the computed range is narrow and plausible;
- the target branch and import branch names are correct.

## 3. Target safety checks

Before a live import:

- the target repository must be clean;
- the import branch must be disposable or intentionally reusable;
- the operator must know whether the import will be merged immediately or inspected first.

## 4. Post-import inspection

After a live import, review:

- `git status --short` in the target repository;
- recent target history to confirm the import branch and merge commit shape;
- the imported file set under the destination paths;
- any destination-specific dependency or workspace breakage introduced by the move.

## 5. Escalation rule

If a future migration needs a different rewrite shape, such as collapsing selected paths directly to branch root, treat that as a separate implementation change to `crane-cli` and re-run the full preflight sequence before production use.
