# Summary

Establish agent-policy guidance for model cost awareness and tiered model routing so that expensive, high-capability models are reserved for large-scope planning, high-level reasoning, and review of dense content, while routine work is delegated to cheaper models via subagents.

# Problem

Sessions are frequently opened with a large, expensive model. That model then spends premium tokens on routine work — running command batches, summarizing tool output, and reading/summarizing many large files or artifacts — that a smaller, cheaper model could handle. There is no standing guidance that (a) encourages model cost awareness, (b) defines a tiered smartness-vs-cost ladder, or (c) tells large-model sessions to route routine subtasks to cheaper subagent models.

# Scope

- Agent-workflow guidance surfaced through token-efficiency and session-optimization instruction files.
- A tiered model ladder describing which tier to use for which class of work.
- Delegation pattern: large-model sessions use `runSubagent` with a cheaper `model` for command batches, output summarization, and large-file/artifact summarization.
- Cross-reference bounded inspection tooling (peek-cli) for reduced, focused artifact views.
- Session transcript observability: record the active model responding to each turn.

# Non-Goals

- Automatic/programmatic model selection or a router service.
- Changing the subagent execution engine or model catalog.

# Acceptance Criteria

1. token-efficiency instructions define a model cost-awareness section with a tiered ladder (high-capability vs mid vs cheap) and explicit "delegate routine work to cheaper subagent models" guidance.
2. Guidance names concrete delegation targets: command/tool-call batches, summarizing large tool outputs, and research/summarization across many large files or artifacts.
3. Guidance reserves high-capability models for large-scope planning, high-level reasoning, and review of dense artifacts.
4. Guidance references peek-cli / bounded inspection for reduced artifact views prior to spending expensive-model tokens.
5. session-optimization instructions describe the smart-model-as-router workflow and require the active model to be recorded per turn in the session-api transcript.
6. session-api transcript records the active model per turn (SessionTurn.model), inheriting the session-level model when unspecified.

# Traceability / Evidence

- Ticket: `321f6a3a-8bfb-4a8e-95bc-64ff845812ed` — [repo-guidance] Model cost-awareness and tiered model-routing guidance.
- Ticket: `11d3b412-7d70-4144-932d-589256af488a` — [session-api] Record active model per transcript turn.
- Validation: `cargo test -p session-api`; `rule sync-targets --config rule-targets.yaml --check`.

# Related Specs

- `8c880efc-7083-4e1d-bf06-96b8254be913` — Dynamic session bootstrapping and just-in-time context routing (adjacent session-api runtime routing behavior).
