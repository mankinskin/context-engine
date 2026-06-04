## Token-Efficient Output

Keep terminal output, file reads, and structural exploration bounded to avoid unnecessary token consumption.

- **Compact by default**: prefer `--toon` over `--json`; prefix commands with `rtk` for automatic filtering.
- **Bounded file reads**: use `repo_map.toon` and interface skeletons before opening source files; read targeted line windows instead of whole files.
- **Differential patching**: use `replace_string_in_file` with context lines instead of full-file rewrites.
- **Long output handling**: when `rtk` or the compact-terminal MCP tool truncates output, inspect the transient file via bounded read/search before replaying the command.
- **TOON vs JSON**: use `--toon` for tool-to-tool pipelines; use `--json` only when piping to external tools (jq, Python). Never request JSON and discard most of it.

Full guidance: `.agents/instructions/token-efficiency.instructions.md`
