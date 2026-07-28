# T9: Image Extraction (Cuttable)

## Objective

Implement the `ExtractImages` variant: extract embedded raster images from a PDF
to standalone image files.

## Status: Deliberately Last, Deliberately Cuttable

Locked decision 7. **Nothing depends on this ticket.** It is sequenced after
everything else so it can be dropped without blocking the release of the rest of
the track.

This is the highest-risk capability in the epic. T0's findings may show there is
no viable pure-Rust path, in which case **cancel this ticket** rather than
forcing it — do not weaken locked decision 3 (pure Rust only) to make image
extraction work.

## Files

- `memory-api/crates/pdf-api/src/extract_images.rs` (new)
- `memory-api/crates/pdf-api/src/lib.rs` (wire into `execute()`)
- `memory-api/crates/pdf-api/tests/extract_images.rs` (new)
- `memory-api/crates/pdf/src/mcp.rs` — register `pdf_extract_images`
- `memory-api/crates/pdf/src/bin/pdf-cli.rs` — add `extract-images`
- `.agents/skills/pdf/SKILL.md` — add to the capability mapping

## Design

### The actual problem

PDF images are XObject streams with a filter chain. Extracting them means:
decoding the stream filter (FlateDecode, DCTDecode/JPEG, JPXDecode/JPEG2000,
CCITTFaxDecode, RunLengthDecode), interpreting the colorspace (DeviceRGB, Gray,
CMYK, Indexed, ICCBased), applying any soft mask, and re-encoding to a normal
image format. This is substantially more work than the other capabilities and is
why it is scoped last.

### Scope control

Do not attempt full coverage. Handle the common cases and fail cleanly on the
rest:

- Support the filters T0 confirmed a pure-Rust decoder exists for.
- For unsupported filters/colorspaces, skip the image and report it in the
  response as a named skipped entry with the reason. Never emit a corrupt file,
  and never fail the whole operation because one image is exotic.
- JPEG2000 in particular is likely unsupported in pure Rust — expect to skip it.

### Output

- Writes into `output_dir`, subject to the full T2 safety layer.
- Validate-all-then-write, same as split (T4).
- Response returns every written path, plus the skipped list with reasons.
- Naming template with a documented default including page number and index.

### Deduplication

The same image XObject referenced from many pages must not be written N times.
Deduplicate by object identity and report which pages reference each image.

## Acceptance Criteria

- [ ] Extracts FlateDecode and DCTDecode images correctly — verified by opening
      the output files, not just checking they exist.
- [ ] Colorspace handling is correct for at least DeviceRGB and DeviceGray;
      CMYK either converts correctly or is skipped with a reason.
- [ ] Unsupported filters/colorspaces are skipped and reported, not fatal.
- [ ] No corrupt output file is ever written.
- [ ] A PDF containing no images returns an empty result, not an error.
- [ ] Images referenced from multiple pages are written once and reported with
      all referencing pages.
- [ ] Every output path passes the T2 sandbox and write-policy checks.
- [ ] A rejected run writes no files.
- [ ] Registered in both the CLI and MCP surfaces.
- [ ] Skill capability mapping updated.

## Validation

```bash
cargo test -p pdf-api extract_images
```

Open the extracted files and confirm they are visually correct — a byte-count
assertion is not sufficient evidence for image extraction.

## Depends On

T8.

## Blocks

Nothing. This is intentional.
