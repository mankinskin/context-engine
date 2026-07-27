## Model Tiering

Model selection for delegated phases is **non-uniform**:

- **Review phase:** one tier ABOVE the cheap threshold. Prefer "Claude Sonnet 4.5 (copilot)".
- **Interview, Commit, Handoff phases:** AT the cheap threshold. Prefer "Claude Haiku 4.5 (copilot)", "GPT-5 mini (copilot)", or "Gemini Flash 2.0 (copilot)".

When models are equal in cost, prefer the latest generation.

Rationale: Review requires careful acceptance-criteria evaluation and finding articulation, justifying the higher cost. Interview, Commit, and Handoff are more mechanical and can run on cheaper models.