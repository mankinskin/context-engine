Add generated `/handoff` and `/handoff-tickets` prompt surfaces for short, reference-centric session jumpstart handoffs. Scope includes rule-target config, canonical prompt rule entries, generated prompt files, and prompt guidance for optional ticket/tracker creation in the handoff workflow.

Implemented slice:
- added prompt targets for `.agents/prompts/handoff.prompt.md` and `.agents/prompts/handoff-tickets.prompt.md` in `rule-targets/30-agents-prompts.yaml`
- created canonical rule entries `084fd4e6-660b-4227-a13e-514edf44e393` and `a634822a-0f7d-407f-b313-b6465b647d2f`
- updated both prompt bodies to reference ticket board lifecycle concerns through `ticket-system.instructions.md`
- updated both prompt bodies to reference persisted `session-api` history captured by the Stop hook when that improves restart speed
- regenerated both prompt outputs and verified them with rule target `--check`

Validation:
- passed `cargo run -p rule-cli --bin rule -- generate-target --config rule-targets.yaml --target context-engine-prompt-handoff`
- passed `cargo run -p rule-cli --bin rule -- generate-target --config rule-targets.yaml --target context-engine-prompt-handoff-tickets`
- passed `cargo run -p rule-cli --bin rule -- generate-target --config rule-targets.yaml --target context-engine-prompt-handoff --check`
- passed `cargo run -p rule-cli --bin rule -- generate-target --config rule-targets.yaml --target context-engine-prompt-handoff-tickets --check`

Remaining work:
- decide whether the handoff prompts should mention any repo-local session query tooling beyond the existing Stop-hook persistence path