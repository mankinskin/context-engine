# Explicit CLI binary naming policy

Add an explicit CLI binary naming rule to `.github/copilot-instructions.md`, near the existing transport naming examples around line 88. The rule must state that the command-line binary uses the bare domain name, for example `ticket`, while `-mcp` and `-http` suffixes are retained for transport binaries.

The policy exists only as an example list today, not as a normative rule. The new text must distinguish package names from binary names: `spec-cli` remains a live package name today, but that package name does not establish the CLI binary naming convention.

Reference ticket `07a3eb2d` for the stale `ticket-cli` references in build and install tooling. The exact target is `.github/copilot-instructions.md`; placing the rule only in a workflow-tool extraction instruction would make the naming policy invisible to ordinary tool authors and retain the current example-only ambiguity.