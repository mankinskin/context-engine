# T0: Verification Spike — PDF Crate Selection

## Status: Scope Reduced

A dedicated research pass with real network access has **already discharged the
survey half of this spike**. Versions, licences, maintenance status, pure-Rust
confirmation, the typst CLI contract, and the T4 merge object-ID renumbering risk
are all resolved below and must **not** be re-investigated.

What remains is the part that can only be answered by reading crate source and
running code. That is the whole of this ticket now.

**Nothing downstream may add a PDF dependency until this ticket is done.**

## Already Verified — Do Not Re-Research

Fetched from the crates.io API, the GitHub API, and raw source. All licences are
MIT / Apache-2.0 compatible. All are pure Rust: no `*-sys`, no C/C++ dependency
appears anywhere in the trees that were pulled.

| Crate | Version | Licence | Last push | Notes |
|---|---|---|---|---|
| `lopdf` | 0.44.0 | MIT | active | MSRV: README says 1.85, manifest field says 1.88 — reconcile |
| `pdf-extract` | 0.12.0 | MIT | active | Depends on `lopdf ^0.42`; also `adobe-cmap-parser`, `cff-parser`, `postscript`, `type1-encoding-parser` |
| `printpdf` | 0.12.4 | MIT | active | **Dropped** — T5 now uses `krilla` |
| `pdf` (pdf-rs) | 0.10.0 | MIT | active | Self-describes writes as "still experimental"; **do not evaluate further** |
| `typst` / `typst-cli` | 0.15.1 | Apache-2.0 | active | CLI contract verified from source, recorded in T5 |
| `krilla` | 0.8.2 | MIT OR Apache-2.0 | active | **Selected** creation backend for T5; built on `pdf-writer`, font subsetting via `skrifa`/`subsetter`/`rustybuzz` |
| `pdf-writer` | 0.15.0 | MIT OR Apache-2.0 | active | Low-level serializer underneath `krilla`; minimal deps |
| `hayro` | 0.7.1 | Apache-2.0 OR MIT | active | Pure-Rust PDF **page rasterizer** — possible T9 angle, but renders pages rather than extracting embedded XObjects |
| `oxidize-pdf` | 4.2.1 | MIT | active | Broad-scope, AI/RAG-targeted, but only ~1 year old vs lopdf's ~9 — treat maturity as unproven |

**Merge object-ID renumbering is a solved problem.** `lopdf::Document` exposes
`renumber_objects()` and `renumber_objects_with(max_id)`, and lopdf's README ships
a working merge example that renumbers each source document before combining
object maps and rebuilding the page tree and trailer. T4 should follow that
reference rather than inventing an approach.

**Encryption behaviour is known.** `lopdf` detects `/Encrypt`, exposes
`is_encrypted()` and `encryption_state`, and auto-decrypts empty-password PDFs.
Handling is specified in T2; nothing to research here.

**`pdf-extract` pulls its own `lopdf`.** It depends on `lopdf ^0.42` while we
would use 0.44 directly, so Cargo will build two copies. This is **not** a
correctness hazard: `pdf-extract`'s surface is string-in/string-out and does not
leak `lopdf` types across its boundary. It costs compile time and binary size
only. Note it in the dependency table so a later reader does not rediscover and
over-worry about it.

**The `lopdf` MSRV discrepancy is cosmetic.** `rust-toolchain.toml` pins
`channel = "nightly"`, which satisfies either 1.85 or 1.88. Record the actual
figure for documentation; it gates nothing.

**`pdfium-render` and any C/C++-backed crate remain excluded by decision 3.** Do
not evaluate, do not propose.

## What This Ticket Still Has To Do

Five items. Each needs code or source-reading, not a web search.

1. **`lopdf` page operations.** Confirm the exact `Document` API for page
   delete, reorder, and rotate. The capability class is confirmed by the page-tree
   model; the literal method names are not. Record signatures for T4.
2. **`krilla` text and font requirement.** Determine whether it needs a supplied
   font asset or offers a no-embed text path. If a font must be vendored, choose
   a permissively licensed one (OFL / Apache-2.0) and record licence and
   provenance for T5.
3. **Image filter coverage in `lopdf`.** Read its filter handling directly.
   FlateDecode is certain (`flate2` is a direct dep) and DCTDecode is likely via
   the optional `image` feature. **CCITTFaxDecode (G3/G4 fax) and JPXDecode
   (JPEG 2000) are unconfirmed and assumed absent.** Confirm or refute by
   reading source. If absent, that is fine — R10 already prescribes skip-and-report
   — but record it so T8 can document it and T9 can be scoped honestly.
4. **`pdf-extract` failure modes.** Upstream documents none. Determine
   empirically: encrypted input, missing ToUnicode CMap, scanned page with no text
   layer, malformed xref. T3 needs to know which of these produce errors versus
   silent empty output.
5. **Fixture strategy.** Where test PDFs live, how they are generated or sourced,
   and their licensing. Include an encrypted fixture and an empty-password
   fixture for T2, and a colliding-object-ID pair for T4's merge test.

## Method

- Build a throwaway scratch crate **outside** the workspace (e.g. under
  `target/tmp/`) and actually exercise each item above. Do not add anything to
  the workspace `Cargo.toml` in this ticket.
- Read crate source directly for item 3 — a docs.rs API listing will not tell you
  which filters are implemented.
- Do NOT fabricate findings. An accurate "unknown, blocked by X" is the correct
  output when something cannot be determined.

## Deliverable

Append a **Findings** section to this description containing:

- Answers to all five items above, with the source file or test output as
  evidence.
- A capability → crate + API entry point binding table covering all six epic
  capabilities, with any unmet capability called out explicitly as a gap.
- A dependency set with exact version requirements, ready to paste into
  `pdf-api/Cargo.toml`.
- The fixture strategy.

## Acceptance Criteria

- [ ] `lopdf` page delete/reorder/rotate method signatures are recorded.
- [ ] `krilla`'s font requirement is determined; if an asset is needed, a
      specific licence-compatible font is named with its provenance.
- [ ] CCITTFaxDecode and JPXDecode support in `lopdf` is confirmed or refuted by
      reading source, and the result recorded for T8 and T9.
- [ ] `pdf-extract` behaviour is recorded empirically for: encrypted input,
      missing ToUnicode, scanned/no-text-layer page, malformed xref.
- [ ] All six capabilities are mapped to a crate + API entry point, or recorded
      as an unmet gap.
- [ ] A fixture strategy is recorded, including encrypted, empty-password, and
      colliding-object-ID fixtures.
- [ ] Every recommended crate has been exercised against a real PDF in the
      scratch project, not just read about.
- [ ] The workspace `Cargo.toml` is unchanged by this ticket.
- [ ] If image extraction has no viable pure-Rust path, that is recorded here so
      T9 can be cut early rather than discovered late.

## Validation

The scratch project compiles and its capability probes run successfully:

```bash
cargo run --manifest-path target/tmp/pdf-spike/Cargo.toml
```

Paste the output into the Findings section as evidence.

## Blocks

T1 (scaffolding) and everything after it.
