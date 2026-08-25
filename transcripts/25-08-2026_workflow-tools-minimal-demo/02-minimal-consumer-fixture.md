# 02 - Build the Minimal Consumer Fixture

## Outcome

Create `minimal-demo`, a checked-in top-level meta-workspace consumer project that demonstrates an application, a few ticket records, a few specifications, and one useful workflow operation end to end.

## Requirements

- `minimal-demo` is an independent top-level meta-workspace consumer submodule beside `workflow-tools` and context-engine, and is intentionally smaller than context-engine.
- The fixture contains a tiny Rust application with one public workflow-tools domain dependency.
- The fixture includes only enough ticket and specification data to demonstrate discovery and one mutation or query described in the tutorial.
- The fixture is reproducible from an empty working directory without borrowing context-engine's submodules, workflow-tools source paths, or local patches.
- The tutorial explains setup, execution, expected output, and teardown in the same order used by CI.

## Non-Goal

Do not turn the fixture into a second production application, a benchmark corpus, or a comprehensive representative-population test suite.

## Validation

A shell scenario creates a fresh copy or checkout, runs the documented bootstrap, builds the application, exercises the selected transport against the fixture stores, and confirms expected records by reading the resulting store artifacts.