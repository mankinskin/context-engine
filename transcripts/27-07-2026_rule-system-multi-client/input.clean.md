# Rule System: Multi-Client Rendering

## Context

The main benefit of the rule system has been partially neglected — to the point where we recently removed the rule targets from all of our instruction files, agent templates, and prompt templates.

The original problem was real: the rule-target system made working with rules significantly more complicated, and it delivered no added benefit because there was barely any exact duplication that could be replaced by shared rules rendered to multiple targets. It simply did not work out well.

However, I have now realized there is one use case where the system is genuinely very useful. The basis is already correct — we just need to improve the rule rendering and the templating to make it really applicable.

## The Valuable Use Case

The repository can be adapted to multiple different agent clients.

For example, OpenCode uses different front matter in instruction files and agent files than GitHub Copilot does. These are format differences — protocol differences — that carry the same information for different consumers of the files we generate.

Maintaining separate files that are 90% identical, differing only in a few adaptations for the consumer protocol, is exactly the duplication the rule system should eliminate.

## Existing Basis

We have (or will have) rule entries carrying the payload text. That payload feeds the templating system, which renders the real files consumed by agents in their workflows.

## Proposed Workflow

1. Define all shared fragments that we will render.
2. Define a small set of templates, working similarly to our current target system. Each template can generate a specific output file.
3. Provide a one-time execution call that installs the generated files — instruction files, skill files, agent files — for a specific platform, or for multiple platforms.
4. Make this part of the repository's installation flow / setup stage.

The user then installs the instruction files for the agent or client they want. The system generates the client-specific files from the templates plus the selected client, so we never maintain multiple near-duplicate files that differ only by consumer protocol.
