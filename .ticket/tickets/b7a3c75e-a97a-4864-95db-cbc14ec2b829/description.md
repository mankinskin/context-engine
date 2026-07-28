## Problem

Ticket authoring currently can run **before** research. In an observed incident, a PDF-domain ticket track locked in `printpdf` as the backend crate; research performed later showed this was the wrong choice — the merge risk that pick was meant to address had already been solved upstream, invalidating tickets that were already authored and had to be cancelled (see cancelled epic `84a9f497-fe5a-4c04-b1e1-ab99245e6ea0` and its children).

`AGENTS.md`'s Task Routing section currently says, for a feature/refactor: "use `.agents/prompts/tickets.prompt.md` to establish the ticket set, then `.agents/prompts/spec.prompt.md` to update the spec" and separately notes "Unfamiliar module or unclear behavior: follow `.agents/prompts/research.prompt.md` ... before locking the spec or implementation plan" — research is anchored to unlock the *spec*, not gated ahead of *ticket authoring* itself.

## Fix direction

- Update `AGENTS.md` Task Routing so research is an explicit precondition of ticket authoring for feature/refactor-scale work (not just spec-locking), so implementation-affecting choices (backend/library selection, architecture) are researched before they get baked into ticket acceptance criteria.
- Update `.agents/prompts/tickets.prompt.md` to require/sequence a research step (or explicit delegation to the Research Agent / research.prompt.md) before creating tickets whose scope is non-trivial.

## Surface

- `AGENTS.md` (Task Routing section).
- `.agents/prompts/tickets.prompt.md`.

## Notes

Ticket-authoring-only in this session; no edits to `AGENTS.md` or `.agents/prompts/tickets.prompt.md` performed here.