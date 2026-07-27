## Model Tiering

Model selection for delegated phases is **non-uniform**:

- **Review phase:** one tier ABOVE the cheap threshold. Prefer "Claude Sonnet 5 (copilot)" (in $2/M, cache read $0.20/M, out $10/M, 1M ctx). Escalate to "GPT-5.3-Codex (copilot)" (out $14/M) or "GPT-5.6 Terra (copilot)" (out $15/M, 1.05M ctx) only for dense, cross-cutting reviews.
- **Interview, Commit, Handoff phases:** AT the cheap threshold (T3). Prefer "GPT-5 mini (copilot)" (in $0.25/M, cache read $0.025/M, out $2/M, 400k ctx) as the default, "GPT-5.6 Luna (copilot)" (in $1/M, out $6/M, 1.05M ctx) when the input exceeds 400k, or "GPT-5.4 mini (copilot)" (in $0.75/M, out $4.5/M, 400k ctx) when the phase needs real reasoning.

Model names must match the dispatch surface's roster exactly; the price table is a vendor catalogue and lists models `runSubagent` will refuse. The canonical tier ladder and the dated verified roster live in [model-routing.instructions.md](.agents/instructions/orchestration/model-routing.instructions.md).

When models are equal in cost, prefer the latest generation, then the larger context window.

"Claude Sonnet 4.5 (copilot)" is superseded as the default: Claude Sonnet 5 is cheaper on input ($2 vs $3/M), output ($10 vs $15/M), and cache read ($0.20 vs $0.30/M) at the same 1M context window. Do not route new work to it.

Rationale: Review requires careful acceptance-criteria evaluation and finding articulation, justifying the higher cost. Interview, Commit, and Handoff are more mechanical and can run on cheaper models.