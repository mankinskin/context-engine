## Problem

`update_body(&self, id_or_slug, content: &str)` in memory-api/crates/spec-api/src/store.rs (~L692-699) calls `write_body(&indexed.path, content)` with no guard and returns `Ok(())` whether `content` is empty or byte-identical to the existing body. In complex workflows this forces agents to re-read the spec just to confirm anything actually changed.

## Decisions (interview-resolved)

- Empty content: reject, but allow it through an explicit force flag for the rare intentional case.
- Byte-identical content: reject as a no-op error. A successful call must mean something changed.

## Notes

Q5 was answered "no": do NOT update .agents/instructions/spec/spec-system.instructions.md as part of this ticket.