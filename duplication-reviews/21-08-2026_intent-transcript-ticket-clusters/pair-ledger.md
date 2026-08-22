# Pair Ledger

Comparison scope (narrowed, per user request — three related guidance clusters, not the full `.agents/` corpus): 14 files, `n × (n-1) / 2 = 91` unordered pairs.

## File Index (sorted `F_1 .. F_14`)

| # | Path |
|---|---|
| F1 | [.agents/agents/mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) |
| F2 | [.agents/agents/ticket-refinement.agent.md](../../.agents/agents/ticket-refinement.agent.md) |
| F3 | [.agents/agents/transcription.agent.md](../../.agents/agents/transcription.agent.md) |
| F4 | [.agents/instructions/orchestration/intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) |
| F5 | [.agents/instructions/orchestration/prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) |
| F6 | [.agents/instructions/ticket/board.instructions.md](../../.agents/instructions/ticket/board.instructions.md) |
| F7 | [.agents/instructions/ticket/engine.instructions.md](../../.agents/instructions/ticket/engine.instructions.md) |
| F8 | [.agents/instructions/ticket/lifecycle.instructions.md](../../.agents/instructions/ticket/lifecycle.instructions.md) |
| F9 | [.agents/instructions/ticket/workflow.instructions.md](../../.agents/instructions/ticket/workflow.instructions.md) |
| F10 | [.agents/instructions/transcripts/audio-transcript.instructions.md](../../.agents/instructions/transcripts/audio-transcript.instructions.md) |
| F11 | [.agents/prompts/refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) |
| F12 | [.agents/prompts/ticket-next.prompt.md](../../.agents/prompts/ticket-next.prompt.md) |
| F13 | [.agents/prompts/tickets.prompt.md](../../.agents/prompts/tickets.prompt.md) |
| F14 | [.agents/prompts/transform-transcript.prompt.md](../../.agents/prompts/transform-transcript.prompt.md) |

## Batches (anchor-fixed, `MAX_FILES_PER_BATCH = 8`, char cap not binding — all files small)

| Batch | Anchor | Targets | Pairs | Phase |
|---|---|---|---|---|
| A1a | F1 | F2-F8 | 7 | 1 |
| A1b | F1 | F9-F14 | 6 | 1 |
| A2a | F2 | F3-F9 | 7 | 1 |
| A2b | F2 | F10-F14 | 5 | 1 |
| A3a | F3 | F4-F10 | 7 | 1 |
| A3b | F3 | F11-F14 | 4 | 1 |
| A4a | F4 | F5-F11 | 7 | 2 |
| A4b | F4 | F12-F14 | 3 | 2 |
| A5a | F5 | F6-F12 | 7 | 2 |
| A5b | F5 | F13-F14 | 2 | 2 |
| A6a | F6 | F7-F13 | 7 | 2 |
| A6b | F6 | F14 | 1 | 2 |
| A7 | F7 | F8-F14 | 7 | 3 |
| A8 | F8 | F9-F14 | 6 | 3 |
| A9 | F9 | F10-F14 | 5 | 3 |
| A10 | F10 | F11-F14 | 4 | 3 |
| A11 | F11 | F12-F14 | 3 | 3 |
| A12 | F12 | F13-F14 | 2 | 3 |
| A13 | F13 | F14 | 1 | 4 |

19 batches, 4 phases (width 6). Total pairs = 91.

## Pairs

| Pair | Batch | Verdict | Status |
|---|---|---|---|
| F1-F2 | A1a | near-duplicate | done |
| F1-F3 | A1a | near-duplicate | done |
| F1-F4 | A1a | thematic overlap | done |
| F1-F5 | A1a | thematic overlap | done |
| F1-F6 | A1a | thematic overlap | done |
| F1-F7 | A1a | no overlap | done |
| F1-F8 | A1a | thematic overlap | done |
| F1-F9 | A1b | thematic overlap | done |
| F1-F10 | A1b | thematic overlap | done |
| F1-F11 | A1b | near-duplicate | done |
| F1-F12 | A1b | thematic overlap | done |
| F1-F13 | A1b | near-duplicate | done |
| F1-F14 | A1b | thematic overlap | done |
| F2-F3 | A2a | no overlap | done |
| F2-F4 | A2a | thematic overlap | done |
| F2-F5 | A2a | thematic overlap | done |
| F2-F6 | A2a | thematic overlap | done |
| F2-F7 | A2a | thematic overlap | done |
| F2-F8 | A2a | thematic overlap | done |
| F2-F9 | A2a | thematic overlap | done |
| F2-F10 | A2b | thematic overlap | done |
| F2-F11 | A2b | near-duplicate | done |
| F2-F12 | A2b | near-duplicate | done |
| F2-F13 | A2b | near-duplicate | done |
| F2-F14 | A2b | near-duplicate | done |
| F3-F4 | A3a | thematic overlap | done |
| F3-F5 | A3a | near-duplicate | done |
| F3-F6 | A3a | no overlap | done |
| F3-F7 | A3a | no overlap | done |
| F3-F8 | A3a | no overlap | done |
| F3-F9 | A3a | no overlap | done |
| F3-F10 | A3a | near-duplicate | done |
| F3-F11 | A3b | exact duplicate | done |
| F3-F12 | A3b | no overlap | done |
| F3-F13 | A3b | no overlap | done |
| F3-F14 | A3b | exact duplicate | done |
| F4-F5 | A4a | exact duplicate | done |
| F4-F6 | A4a | no overlap | done |
| F4-F7 | A4a | no overlap | done |
| F4-F8 | A4a | no overlap | done |
| F4-F9 | A4a | no overlap | done |
| F4-F10 | A4a | thematic overlap | done |
| F4-F11 | A4a | near-duplicate | done |
| F4-F12 | A4b | thematic overlap | done |
| F4-F13 | A4b | thematic overlap | done |
| F4-F14 | A4b | near-duplicate | done |
| F5-F6 | A5a | thematic overlap | done |
| F5-F7 | A5a | thematic overlap | done |
| F5-F8 | A5a | thematic overlap | done |
| F5-F9 | A5a | thematic overlap | done |
| F5-F10 | A5a | near-duplicate | done |
| F5-F11 | A5a | near-duplicate | done |
| F5-F12 | A5a | thematic overlap | done |
| F5-F13 | A5b | thematic overlap | done |
| F5-F14 | A5b | near-duplicate | done |
| F6-F7 | A6a | no overlap | done |
| F6-F8 | A6a | no overlap | done |
| F6-F9 | A6a | near-duplicate | done |
| F6-F10 | A6a | no overlap | done |
| F6-F11 | A6a | no overlap | done |
| F6-F12 | A6a | near-duplicate | done |
| F6-F13 | A6a | no overlap | done |
| F6-F14 | A6b | no overlap | done |
| F7-F8 | A7 | thematic overlap | done |
| F7-F9 | A7 | thematic overlap | done |
| F7-F10 | A7 | no overlap | done |
| F7-F11 | A7 | no overlap | done |
| F7-F12 | A7 | thematic overlap | done |
| F7-F13 | A7 | thematic overlap | done |
| F7-F14 | A7 | no overlap | done |
| F8-F9 | A8 | thematic overlap | done |
| F8-F10 | A8 | no overlap | done |
| F8-F11 | A8 | no overlap | done |
| F8-F12 | A8 | thematic overlap | done |
| F8-F13 | A8 | near-duplicate | done |
| F8-F14 | A8 | no overlap | done |
| F9-F10 | A9 | no overlap | done |
| F9-F11 | A9 | thematic overlap | done |
| F9-F12 | A9 | thematic overlap | done |
| F9-F13 | A9 | near-duplicate | done |
| F9-F14 | A9 | no overlap | done |
| F10-F11 | A10 | near-duplicate | done |
| F10-F12 | A10 | no overlap | done |
| F10-F13 | A10 | no overlap | done |
| F10-F14 | A10 | near-duplicate | done |
| F11-F12 | A11 | thematic overlap | done |
| F11-F13 | A11 | thematic overlap | done |
| F11-F14 | A11 | near-duplicate | done |
| F12-F13 | A12 | near-duplicate | done |
| F12-F14 | A12 | no overlap | done |
| F13-F14 | A13 | near-duplicate | done |
