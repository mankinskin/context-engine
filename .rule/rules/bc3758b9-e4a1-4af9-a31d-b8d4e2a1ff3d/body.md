### Rule Introduces Spec Obligation

During session construction, every spec must be governed and introduced by a PolicyRule, conditioned on the spec's computed readiness status:

- **implemented** — Present the spec as a live, fully dependable contract that dependents can immediately rely on.
- **partial-with-gaps** — Present the spec but list explicit unimplemented positions so agents do not assume gaps are complete.
- **coming-soon / not-implemented** — Present a "coming soon" note so agents are aware the spec is defined but unimplemented.

This conditioning makes spec availability highly legible to agents, preventing context bloat and ensuring all active specifications have active rules.
