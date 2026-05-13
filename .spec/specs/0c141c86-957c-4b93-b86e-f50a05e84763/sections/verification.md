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

## Current fixture coverage

The current automated verification slice now covers:

- a generic end-to-end temp-repo transplant proving filtered import and merge behavior;
- a dry-run fixture that asserts the review metadata emitted to the operator (`source_ref`, `source_commit`, `anchor_commit`, `range_spec`, `target_branch`, `import_branch`, `import_ref`, and mappings);
- the current context-stack-shaped same-path fixture for `tools/**/context-*` imports, including:
	- sibling leakage under `tools/**`;
	- same-prefix collisions such as `context-http` vs `context-http-extra`;
	- delete propagation inside selected paths;
	- proof that `--dry-run` leaves the target repository clean and does not create the import ref.

## Next fixture candidates

Useful next fixtures after the current slice:

- branch-root rewrites once that feature exists, so the same path set can be collapsed to repository root and validated end to end;
- explicit rename-heavy histories if a future migration depends on preserving rename/copy semantics beyond add/delete equivalence;
- target-repo preconditions such as a dirty worktree or pre-existing import branch;
- destination-repo overlap scenarios where the target already contains files under the mapped destination path and the operator needs a predictable inspection strategy before merge.
