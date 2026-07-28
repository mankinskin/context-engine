# T0: Verification Spike — PDF Crate Selection

## Why This Exists

The session that authored this track had **no web-fetch capability**. Every
crate name in the epic came from an unverified research note. No version, API
signature, license, maintenance status, or feature flag has been confirmed. This
spike converts guesses into facts before a single dependency is added.

**Nothing downstream may add a PDF dependency until this ticket is done.**

## Objective

For each candidate, determine the facts and produce a binding recommendation.

## Candidates To Evaluate

| Crate | Claimed role |
|---|---|
| `lopdf` | Low-level PDF object/structure manipulation (page ops, metadata, merge/split) |
| `pdf-extract` | Text extraction |
| `pdf` (pdf-rs) | Low-level parsing / object tree access |
| `printpdf` | Programmatic PDF creation |
| `typst` / `typst-cli` | Markup → PDF generation (external process only) |

Explicitly excluded by decision 3: `pdfium-render` and anything else with a
C/C++ dependency. Do not evaluate it; do not propose it.

## What To Determine Per Crate

1. Latest published version on crates.io.
2. Maintenance status: last release date, open-issue responsiveness, whether it
   appears abandoned.
3. License — **must be MIT or Apache-2.0 compatible**. A crate with an
   incompatible license is disqualified regardless of capability.
4. Whether it is genuinely pure Rust (check for `build.rs` linking native libs,
   `*-sys` dependencies, or bundled binaries).
5. Which of the six epic capabilities it can actually deliver, with the concrete
   API entry points that deliver them.
6. MSRV and whether it is compatible with this workspace's toolchain
   (`rust-toolchain.toml`).

## Method

Prefer real evidence over documentation claims:

- `cargo add --dry-run` / `cargo search` / `cargo info` for versions and
  licenses.
- Fetch docs.rs pages if web access is available in the implementing session.
- Build a throwaway scratch crate **outside** the workspace (e.g. under
  `target/tmp/`) and actually exercise each candidate against a real PDF for the
  capability it claims. Do not add anything to the workspace `Cargo.toml` in
  this ticket.
- If web access is still unavailable in the implementing session, say so
  explicitly and use the vendored registry index / `cargo` metadata instead.
  Do NOT fabricate findings.

## Deliverable

Update this ticket's description with a **Findings** section containing:

- The per-crate fact table (version, license, maintained, pure-Rust, MSRV).
- A capability → crate binding table covering all six capabilities, with any
  capability that has **no viable pure-Rust crate** called out explicitly as a
  gap.
- A recommended dependency set with exact version requirements, ready to paste
  into `pdf-api/Cargo.toml`.
- Any disqualifications and the reason.
- A test PDF fixture strategy: where fixtures live, how they are generated or
  sourced, and their licensing.

## Acceptance Criteria

- [ ] Every candidate has a confirmed version, license, and maintenance status
      recorded in this ticket.
- [ ] Every license is confirmed MIT/Apache-2.0 compatible, or the crate is
      disqualified with the reason recorded.
- [ ] Every candidate is confirmed pure Rust (no `*-sys`, no native linking).
- [ ] All six capabilities are mapped to a specific crate + API entry point, or
      explicitly recorded as an unmet gap.
- [ ] Each recommended crate has been exercised against a real PDF in a scratch
      project, not just read about.
- [ ] A fixture strategy is recorded.
- [ ] The workspace `Cargo.toml` is unchanged by this ticket.
- [ ] If image extraction (capability 6) has no viable pure-Rust path, that is
      recorded here so T9 can be cut early rather than discovered late.

## Validation

The scratch project compiles and its capability probes run successfully:

```bash
cargo run --manifest-path target/tmp/pdf-spike/Cargo.toml
```

Paste the output into the Findings section as evidence.

## Blocks

T1 (scaffolding) and everything after it.
