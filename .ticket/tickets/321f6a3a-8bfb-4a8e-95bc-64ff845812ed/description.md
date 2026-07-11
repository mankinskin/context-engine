# [repo-guidance] Model cost-awareness and tiered model-routing guidance

Add durable agent-policy guidance that encourages model cost awareness and delegation of cheap, routine work to smaller/cheaper models via subagents — especially inside sessions driven by large, expensive models.

## Motivation
We want a workflow where a large, smart model opens a session and actively performs model routing for subtasks to save token cost. Large/expensive models should be reserved for large-scope planning, high-level reasoning, and review of dense content or individual artifacts. Routine work (command batches, output summarization, research across many large files/artifacts) should be delegated to cheaper models.

## Scope
- Add a "Model Cost Awareness & Routing" guidance block to the token-efficiency instructions (rule store body + regenerate).
- Add a tiered model ladder (smartness vs cost) describing where to use which tier.
- Encourage using `runSubagent` with a cheaper `model` for command batches, output summarization, and large-file/artifact summarization.
- Cross-link peek-cli and other inspection tools for rendering reduced, focused artifact views before spending expensive-model tokens.
- Add a short model-routing workflow note to session-optimization instructions.

## Acceptance Criteria
1. token-efficiency instructions include a model cost-awareness section with a tiered ladder and explicit delegation guidance.
2. Guidance explicitly tells large-model sessions to route routine batches/summarization to cheaper subagent models.
3. Guidance references peek-cli / bounded inspection for reduced artifact views.
4. session-optimization instructions mention starting with a smart routing model and recording the active model.
5. Generated instruction files regenerated via `rule sync-targets` (no hand edits to generated output); rule check passes.
