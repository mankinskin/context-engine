# Objective

Add step-by-step worked traces to the existing `read_sequence` / `context-read pipeline` spec chain so the overlap algorithm is specified through concrete iterations, variable transitions, and commit behavior.

## Scope

Document the worked traces selected by the user:

- `heldld -> hell`
- `aabb -> aabbaabb`
- `xyyxy`
- `abcde -> bcdea -> cdeab -> deabc`
- `subdivision -> visualization` and `subvisu -> visub`
- `abcabababcaba`

## Required shape

The traces should use the clarified algorithm model:

- each matched range maps to at most one token;
- tokens may have multiple first-class decompositions;
- decomposition order is operational, not canonical;
- normalization is required only on the most abstract API surfaces;
- graph state is materialized after each overlap expansion step.

Each trace should explain the per-step rule:

1. search the largest next overlap;
2. complete the left/right complements from the path start/end;
3. commit the overlap to the current root, including edge cases.

## Done when

- the relevant draft specs include worked traces for the listed sequences;
- the traces explicitly name the important state variables (`root`, `anchor`, `flat_root`, segment inputs, overlap result, complements, committed root);
- the spec references validate successfully against the current repo layout.