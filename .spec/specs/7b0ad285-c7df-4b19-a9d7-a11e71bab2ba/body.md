<!-- aligned-structure:v1 -->

# Summary

Integrate the Cline Agent Client (uses `.clinerules/` by default) while maintaining client agnosticity. The canonical standard remains `.agents/` + `AGENTS.md` as the source of truth for all agent guidance.

## Behavior Story

Integrate the Cline Agent Client (uses `.clinerules/` by default) while maintaining client agnosticity. The canonical standard remains `.agents/` + `AGENTS.md` as the source of truth for all agent guidance.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

## Overview

Integrate the Cline Agent Client (uses `.clinerules/` by default) while maintaining client agnosticity. The canonical standard remains `.agents/` + `AGENTS.md` as the source of truth for all agent guidance.

## Architecture

```
canonical rule graph (.rule/)
        │
        ▼
rule-target pipeline (rule-targets/*.yaml)
        │
        ├──▶ AGENTS.md                    (neutral constitution)
        ├──▶ .agents/instructions/*       (neutral path-scoped guidance)
        ├──▶ .agents/prompts/*            (neutral workflow prompts)
        ├──▶ .agents/agents/*             (neutral role/persona docs)
        │
        ├──▶ .github/copilot-instructions.md   (Copilot client adapter)
        └──▶ .clinerules/*.md                  (Cline client adapter)  ← NEW

shared hook scripts (tools/agent-hooks/)  ← NEW
        │
        ├──▶ .github/hooks/hooks.json     (Copilot hook manifest)
        └──▶ .clinerules/hooks/hooks.json (Cline hook manifest)
```

## Requirements

### R1 — Cline guidance adapter
- `rule-targets/25-cline.yaml` defines Cline adapter targets under `.clinerules/`.
- `.clinerules/00-source-of-truth.md` clearly identifies `AGENTS.md` + `.agents/` as canonical and `.clinerules/` as a generated adapter surface.
- `.clinerules/10-core-rules.md` is a Cline-friendly projection of the core agent rules.
- `.clinerules/20-workflows.md` is a workflow prompt index for Cline.
- `.clinerules/30-path-scoped.md` is a path-scoped guidance index for Cline.

### R2 — Neutral hook scripts
- Shared hook scripts live in `tools/agent-hooks/` (neutral, client-agnostic location).
- Both `.clinerules/hooks/hooks.json` and `.github/hooks/hooks.json` reference `tools/agent-hooks/` scripts.
- Scripts in `.github/hooks/` and `.clinerules/hooks/` are deprecated copies; `tools/agent-hooks/` is canonical.

### R3 — Canonical Sources updated
- `AGENTS.md` Canonical Sources section references the Cline adapter surface.

## Validation

- `rule sync-targets --check` passes.
- `.clinerules/00-source-of-truth.md` exists and contains source-of-truth pointer.
- `.clinerules/10-core-rules.md` exists and mirrors core agent rules.
- `.clinerules/20-workflows.md` exists and indexes workflow prompts.
- `.clinerules/30-path-scoped.md` exists and indexes path-scoped guidance.
- `tools/agent-hooks/validate-docs.sh` exists.
- `tools/agent-hooks/terminal-pwd.sh` exists.
- `tools/agent-hooks/session-capture-stop.sh` exists.
- `tools/agent-hooks/validate-docs-stop.sh` exists.
- Both `hooks.json` files reference `tools/agent-hooks/` scripts.

## Traceability

- Ticket: 37dfe6cc-0d8d-4b85-b1cb-e9c262a9de5f
- Files: `rule-targets/25-cline.yaml`, `.clinerules/*.md`, `tools/agent-hooks/*`, `.clinerules/hooks/hooks.json`, `.github/hooks/hooks.json`
