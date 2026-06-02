# Summary

Add failing blackbox tests and update the concrete authoring guidance surfaces so expectation-oriented specs are defined by intended properties, acceptance criteria, and evidence requirements rather than implementation-plan prose.

# Why

The current prompt and mirrored rule guidance actively encourage motivation, problem, and rollout narration in specs. If the blackbox contract is not nailed down first, the rest of the model work will only shuffle prose around.

# Scope

- define the observable contract for expectation-oriented specs using create, update, get, search, and health behavior
- update [.github/prompts/spec.prompt.md](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.github/prompts/spec.prompt.md) so ticket-level problem and rollout details are no longer treated as core spec content
- update the mirrored authoring guidance in [.rule/rules/0719f0c1-9036-4983-912c-599de3a37d23/body.md](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.rule/rules/0719f0c1-9036-4983-912c-599de3a37d23/body.md) to keep the rule-backed guidance in sync with the prompt contract
- add regression coverage that fails if either guidance surface drifts or reintroduces motivation, problem, current-state, or rollout sections as core spec requirements
- preserve the current markdown-oriented spec shell during the first slice unless a specific layout change is required by the tests

# Assumptions To Prove

- the current markdown shell can host the new meaning without a mandatory file-layout rewrite in the first slice
- blackbox tests can distinguish a contract-focused spec from a rollout-focused spec without relying on manual review
- the prompt and mirrored rule guidance can be validated mechanically enough to prevent drift between those surfaces

# Acceptance Criteria

- A red/green test suite defines the minimum observable contract for an expectation-oriented spec.
- [.github/prompts/spec.prompt.md](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.github/prompts/spec.prompt.md) and [.rule/rules/0719f0c1-9036-4983-912c-599de3a37d23/body.md](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.rule/rules/0719f0c1-9036-4983-912c-599de3a37d23/body.md) describe the same contract.
- The default authoring guidance tells users to keep current-state analysis, rollout sequencing, blockers, and implementation notes in tickets rather than in the spec contract.
- The first slice keeps the current format stable unless the tests prove a structure change is necessary.
- Regression coverage exists for both legacy current-format specs and newly authored expectation-oriented specs, plus drift between the two guidance surfaces.

# Validation

- A focused failing-then-passing test suite for the targeted authoring workflow.
- A focused check that the prompt and mirrored rule guidance stay aligned.