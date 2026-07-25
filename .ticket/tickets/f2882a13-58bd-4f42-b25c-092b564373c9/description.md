Phase B. Pre-create the `interview` repository (owner mankinskin) as a placeholder for the interview tool, which is planned but not yet built (see interview-api ticket 7639449a). Establish the standard tool repo skeleton (api crate stub + transport stubs + artifact stores) so future implementation lands in its own repo from the start.

Follow the common per-tool extraction recipe (see parent tracker) to the extent components exist; otherwise scaffold the skeleton and register the placeholder in workflow-tools.

## Acceptance criteria
- `interview` repo exists with the standard tool skeleton (api + transport stubs + stores).
- Registered as a workflow-tools dependency (placeholder allowed).
- Linked to the interview-api design ticket so implementation targets this repo.