# T5: PDF Creation — Programmatic Primitive + Optional Typst Path

## Objective

Implement the `Create` variant with two modes, per locked decision 4.

## Files

- `memory-api/crates/pdf-api/src/create.rs` (new)
- `memory-api/crates/pdf-api/src/typst.rs` (new — detection + invocation)
- `memory-api/crates/pdf-api/src/lib.rs` (wire into `execute()`)
- `memory-api/crates/pdf-api/tests/create.rs` (new)

## Design

### Mode A — programmatic primitive (always available)

Uses the pure-Rust creation crate T0 selected. Deliberately minimal: this is a
primitive for agents that need to emit a simple document, not a layout engine.

Scope it to: page size, pages, text blocks with a basic font/size, and simple
line breaks. Explicitly out of scope: tables, images, columns, styling systems.
If a caller needs real typesetting, they should use Mode B.

Font handling is the main trap — decide and document whether the built-in
standard-14 fonts are used or whether a font must be supplied, and make
non-ASCII text either work or fail with a clear error rather than silently
emitting garbage glyphs.

### Mode B — typst (optional, external process)

- Input is typst markup (source string or a source file inside the sandbox
  root).
- `typst-cli` is invoked as an **external process**. It is NOT a build
  dependency and NOT a linked library.
- Detect it on `PATH` at call time.
- If absent, return `PdfError::ToolUnavailable` with an actionable message
  naming the binary and how to install it. Never panic. Never silently fall back
  to Mode A — a silent downgrade would produce a document that looks nothing
  like what was asked for.

**Security — this is the sharp edge of the whole track.** Invoking an external
process with agent-supplied input requires care:

- Pass the source via a temp file inside the sandbox root, not via a shell
  string.
- Invoke the binary directly with argument vectors; never through a shell.
- Do not let caller input inject additional CLI flags — anything caller-supplied
  goes in a position where it cannot be parsed as a flag, or is validated
  against an allowlist.
- Constrain typst's own root/input access so markup cannot read arbitrary files
  via typst's include/read features and exfiltrate them into the output PDF.
- Apply a wall-clock timeout so a pathological document cannot hang the caller
  indefinitely.
- Cap output size.
- The output path is still subject to the T2 write policy.

### Mode selection

The request carries an explicit mode. Do not auto-detect from content —
ambiguity here becomes a silent-wrong-output bug.

## Acceptance Criteria

- [ ] Mode A produces a valid, openable PDF with the requested pages and text.
- [ ] Mode A's font/encoding behavior is documented; non-ASCII either works or
      errors clearly.
- [ ] Mode B produces a valid PDF when `typst-cli` is present.
- [ ] Mode B returns `ToolUnavailable` with an actionable message when
      `typst-cli` is absent — verified by a test that runs with a `PATH` that
      excludes it.
- [ ] Mode B never silently falls back to Mode A.
- [ ] typst is invoked without a shell; a source containing shell metacharacters
      and flag-like tokens cannot alter the invocation — asserted by test.
- [ ] typst markup cannot read files outside the sandbox root — asserted by a
      test that attempts it.
- [ ] A timeout aborts a runaway typst invocation and returns a clean error.
- [ ] Output path honors T2 copy-on-write / `overwrite` policy.
- [ ] Tests that require `typst-cli` skip cleanly (not fail) when it is absent,
      so CI without typst stays green.

## Validation

```bash
cargo test -p pdf-api create
```

Record whether `typst-cli` was present in the validating run, since that changes
which tests actually executed.

## Depends On

T2.
