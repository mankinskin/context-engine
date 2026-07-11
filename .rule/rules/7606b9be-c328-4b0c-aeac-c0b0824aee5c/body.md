## Token-Efficient Output

Keep terminal output, file reads, and structural exploration bounded to avoid unnecessary token consumption.

- **Model-bound context first**: optimize what reaches the model API before tokens are spent; transcript capture after the fact is diagnostic only.
- **Model cost awareness & routing**: token cost depends on *which* model does the work. Reserve large, expensive models for large-scope planning, high-level reasoning, and review of dense content or individual artifacts. In large-model sessions, act as a router and delegate routine work — command/tool-call batches, summarizing large tool outputs, and research/summarization across many large files or artifacts — to cheaper models via `runSubagent` with an explicit cheaper `model`.
- **Compact by default**: prefer `--toon` over `--json`; prefix commands with `rtk` for automatic filtering.
- **Bounded file reads**: use the root `repo_map.toon` and interface skeletons before opening source files; read targeted line windows instead of whole files.
- **Differential patching**: use `replace_string_in_file` with context lines instead of full-file rewrites.
- **Long output handling**: when `rtk` or the compact-terminal MCP tool truncates output, inspect the transient file via bounded read/search before replaying the command.
- **Routine-action discipline**: do not spend reasoning budget narrating obvious next tool calls, unchanged state checks, or simple retries.
- **TOON vs JSON**: use `--toon` for tool-to-tool pipelines; use `--json` only when piping to external tools (jq, Python). Never request JSON and discard most of it.

Full guidance: `.agents/instructions/token-efficiency.instructions.md`
