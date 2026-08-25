# 03 - Continuously Verify the Tutorial

## Outcome

Add GitHub Actions coverage that executes the minimal consumer tutorial from a clean runner and makes installation regressions visible before context-engine or pitch-scripts depend on the release path.

## Requirements

- The workflow runs the same documented scenario as the fixture tutorial rather than an independent approximation.
- The workflow checks out workflow-tools and resolves dependencies without an inherited root Cargo patch.
- The workflow verifies workflow-skill bootstrap, Cargo build, installed transport discovery, and a ticket/spec operation.
- The workflow retains diagnostics for failed setup steps.
- The first implementation targets Linux; a Windows matrix job follows once shell-independent installation behavior is available.

## Non-Goal

Do not require full viewer, Playwright, or all-domain transport coverage from the minimal fixture workflow.

## Validation

The workflow runs on pull requests and main, with a local equivalent command that exits nonzero on any failed tutorial step.