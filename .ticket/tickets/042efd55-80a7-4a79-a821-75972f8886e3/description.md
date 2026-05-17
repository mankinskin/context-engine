# Problem

The repository workflow now requires documentation updates and documentation validation for changed codebases and generated guidance surfaces, but doc-tool coverage is still partial.

At the moment the workflow has to fall back to ad hoc manual checks or narrow validation commands, which makes documentation review inconsistent and hard to trace.

# Scope

Expand documentation tooling so workflow-driven work can verify docs and guidance surfaces in a consistent way.

This should cover both authored docs and generated guidance where practical.

The tool improvements should support:

- identifying or accepting the docs affected by a change
- validating updated docs and generated guidance surfaces
- reporting missing coverage or unsupported cases explicitly
- emitting structured output that can be referenced from ticket/spec summaries

# Acceptance criteria

- Documentation tooling can validate the relevant docs or generated guidance surfaces for a change.
- The tooling emits structured results suitable for workflow summaries and review checklists.
- Unsupported or partial coverage is reported explicitly instead of silently skipped.
- Repository guidance can point to the improved doc-tool path where appropriate.
