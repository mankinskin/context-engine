# T3: Text Extraction

## Objective

Implement the `ExtractText` variant: given a PDF, return its text content.

## Files

- `memory-api/crates/pdf-api/src/extract_text.rs` (new)
- `memory-api/crates/pdf-api/src/lib.rs` (wire into `execute()`)
- `memory-api/crates/pdf-api/tests/extract_text.rs` (new)

## Design

Use whichever crate T0 bound to capability 1. Do not hard-code an API here —
T0's Findings section is authoritative.

### Request

- `input: PathBuf` — validated through the T2 safety layer.
- `pages: Option<PageRange>` — optional page selection; absent means all pages.
  Page numbers follow T2's fixed convention: **1-based inclusive externally,
  0-based internally, converted only at request validation in `pdf-api`.** Do
  not introduce a second conversion point here.
- Read-only: this variant writes nothing and therefore has no `WriteTarget`.

### Response

- Extracted text.
- Page count of the source document.
- Which pages were actually extracted.

### Output bounding

Per the `peek-api` convention, bounding happens in the API crate, not the
transport. A 400-page PDF must not blow up an agent's context window. Provide:

- an optional `max_chars` cap with a documented default, and
- a truncation indicator in the response so the caller knows output was cut and
  can request a narrower page range.

Pick the default deliberately and record the reasoning in the code comment;
`peek-api`'s `DEFAULT_WINDOW` is the precedent for having a sane bounded
default rather than unbounded output.

### Behavior on non-extractable PDFs

A scanned/image-only PDF yields little or no text. This must not look like
success-with-empty-output. Detect the empty-or-near-empty case and return a
response that explicitly signals "no extractable text layer — this document may
be scanned; OCR is out of scope". This is the single most likely confusing
failure mode for an agent caller.

## Acceptance Criteria

- [ ] Extracts text from a normal text-layer PDF fixture.
- [ ] Page-range selection returns only the requested pages; page `1` selects
      the first page and page `0` is rejected as a user error.
- [ ] An out-of-bounds or inverted page range is a user error, not a panic.
- [ ] Output respects `max_chars` and sets the truncation indicator when cut.
- [ ] A scanned/image-only fixture returns the explicit "no text layer" signal
      rather than a bare empty string.
- [ ] A non-PDF file (e.g. a `.txt` renamed to `.pdf`) is a clean user error.
- [ ] An encrypted/password-protected PDF is a clean user error, not a panic.
- [ ] A path outside the sandbox root is rejected (inherited from T2, asserted
      here too).
- [ ] Malformed/truncated PDF input does not panic.

## Validation

```bash
cargo test -p pdf-api extract_text
```

## Depends On

T2.
