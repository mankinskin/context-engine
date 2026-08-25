# 02 - Build the Minimal Consumer Fixture

## Outcome

Create `minimal-demo`, a checked-in top-level meta-workspace consumer project and Docker fixture that demonstrate an application, a few ticket records, a few specifications, and one useful workflow operation end to end.

## Requirements

- `minimal-demo` is an independent top-level meta-workspace consumer submodule beside `workflow-tools` and context-engine, and is intentionally smaller than context-engine.
- The fixture contains a tiny Rust application with one public workflow-tools domain dependency.
- The fixture includes only enough ticket and specification data to demonstrate discovery and one mutation or query described in the tutorial.
- The fixture is reproducible from an empty working directory without borrowing context-engine's submodules, workflow-tools source paths, or local patches.
- The Docker fixture starts from a fresh image, checks out `workflow-minimal-demo` through one entry point, and invokes the commit-pinned GitHub `install.sh` through one `curl | bash` command.
- The Docker fixture drives the interactive `install-ctl` TUI with `ratatui-testlib`, selecting the required tools and one supported installation home.
- The Docker fixture verifies workflow tools, agent client tools, instructions, and hooks, plus selected binaries below `<installation-home>/.workflow-tools/bin/`.
- The Docker build retains prebuilt delivered binaries after the first run and reuses those binaries on later runs; the fixture does not compile delivered workflow-tools binaries from source.
- The tutorial explains setup, execution, expected output, and teardown in the same order used by CI.

## Non-Goal

Do not turn the fixture into a second production application, a benchmark corpus, or a comprehensive representative-population test suite.

## Validation

A Docker scenario creates a fresh image and consumer checkout, runs the documented bootstrap, builds the application, exercises the selected installed transport against the fixture stores, and confirms expected records by reading the resulting store artifacts. The scenario confirms installation and environment wiring independently of source-build validation for delivered binaries.