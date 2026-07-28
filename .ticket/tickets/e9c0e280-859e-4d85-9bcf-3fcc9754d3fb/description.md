# T2: `pdf-api` Core — Request/Response Types, Error, Dispatch, Safety Layer

## Objective

Define the `pdf-api` contract and implement the **shared safety layer** that
every operation must pass through. This is the single most important ticket in
the track: sandboxing and write-safety live here, once, so no individual
operation can forget them.

All operation variants are declared here; their bodies return
`PdfError::NotImplemented` until T3/T4/T5/T9 fill them in.

## Files To Create

```
memory-api/crates/pdf-api/src/lib.rs        # execute() dispatch + re-exports
memory-api/crates/pdf-api/src/request.rs    # tagged request enum
memory-api/crates/pdf-api/src/response.rs   # response types
memory-api/crates/pdf-api/src/error.rs      # PdfError (thiserror)
memory-api/crates/pdf-api/src/security.rs   # root confinement + write policy
memory-api/crates/pdf-api/tests/security.rs # safety-layer tests
```

## Design

### Shape

Mirror `memory-api/crates/peek-api/src/lib.rs`:

- Tagged serde enum for requests: `#[serde(tag = "kind", rename_all = "snake_case")]`.
- One dispatch entry point:
  `pub fn execute(request: &PdfRequest) -> Result<PdfResponse, PdfError>`.
- `thiserror`-derived error enum.
- All bounding/validation happens here; transports stay thin.

### `PdfRequest` variants (all declared in this ticket)

| Variant | Owner ticket | Inputs |
|---|---|---|
| `ExtractText` | T3 | `input`, optional page range |
| `Merge` | T4 | `inputs: Vec<PathBuf>` (N inputs), `output` |
| `Split` | T4 | `input`, `output_dir`, split spec (per-page or ranges) |
| `EditPages` | T4 | `input`, `output`, ops (delete/reorder/rotate) |
| `GetMetadata` | T4 | `input` |
| `SetMetadata` | T4 | `input`, `output`, field map |
| `Create` | T5 | creation spec (programmatic or typst source), `output` |
| `ExtractImages` | T9 | `input`, `output_dir` |

Every variant that writes carries a `WriteTarget`; every variant carries the
sandbox `root`.

### Multi-file request shapes

The tagged-enum + single-`execute()` convention must still express N-input and
1→N-output operations:

- **Merge (N→1)**: `inputs: Vec<PathBuf>` — each element validated
  independently against the root. Order of `inputs` is the page order of the
  result. An empty `inputs` vec is a user error, not an empty PDF.
- **Split (1→N)**: `output_dir` plus a naming template; the response returns the
  full list of written paths so the caller never has to guess filenames. Every
  produced path is validated and write-policy-checked before any file is
  written — validate all, then write, so a rejection midway does not leave a
  half-written output set.

### Safety layer (`security.rs`) — the core of this ticket

**Root confinement.** Reuse the existing precedent rather than reinventing it:
`fs-api::security::validate_path_within_root(path, root, label)` in
`memory-api/crates/fs-api/src/security.rs`. It canonicalizes both path and root
to defeat symlink and Windows-junction escapes, and handles not-yet-existing
destinations by canonicalizing the parent and re-appending the filename.

Decide and record which of these two you do, and why:
- depend on `fs-api` from `pdf-api` and call it directly, or
- if that creates an unwanted coupling, port the function and keep behavior
  identical.

Prefer reuse. There is **no opt-out** — every input path and every output path
in every variant passes through it.

**Write policy.** Mirror `memory-api/crates/fs-api/src/mutation.rs`:

- An explicit output path is always required. Never derive an output path by
  mutating the input path silently.
- If the destination already exists and `overwrite` is not `true`, fail with a
  distinct error variant naming the conflicting path.
- Overwriting the **input file itself** requires `overwrite: true` as well, and
  must be implemented write-to-temp-then-rename so a mid-operation failure
  cannot destroy the source.

### `PdfError`

`thiserror` enum, with variants separable into user errors vs internal errors so
the MCP layer (T7) can map them to `invalid_params` vs `internal_error`. Expose
a predicate (e.g. `PdfError::is_user_error()`) so T7 does not have to match on
every variant. Include at least: `PathOutsideRoot`, `DestinationExists`,
`InputNotFound`, `NotAPdf`, `EmptyInputSet`, `InvalidPageRange`,
`ToolUnavailable`, `Parse`, `Io`, `NotImplemented`.

## Acceptance Criteria

- [ ] `PdfRequest` covers all eight variants and round-trips through
      serde JSON with a `kind` tag.
- [ ] `execute()` dispatches every variant; unimplemented ones return
      `NotImplemented`, never panic.
- [ ] A path outside the root is rejected — including via a symlink and, on
      Windows, via a directory junction.
- [ ] A relative-traversal path (`../../etc/passwd`) is rejected.
- [ ] Writing to an existing destination without `overwrite` fails with a
      distinct error naming the path; with `overwrite: true` it succeeds.
- [ ] In-place overwrite is implemented via temp-file + rename, verified by a
      test that injects a failure mid-operation and asserts the original file is
      intact.
- [ ] `Merge` with an empty input set is a user error.
- [ ] `Split` validates every output path before writing any of them.
- [ ] `is_user_error()` classifies every variant.
- [ ] No operation can reach the filesystem without passing the safety layer —
      assert this structurally (e.g. a single private helper is the only fs
      entry point).

## Validation

```bash
cargo test -p pdf-api
```

Security tests must cover symlink escape (Unix) and junction escape (Windows),
matching the platform-split coverage documented in
`memory-api/crates/fs-api/src/security.rs`.

## Depends On

T1.

## Blocks

T3, T4, T5, T9.
