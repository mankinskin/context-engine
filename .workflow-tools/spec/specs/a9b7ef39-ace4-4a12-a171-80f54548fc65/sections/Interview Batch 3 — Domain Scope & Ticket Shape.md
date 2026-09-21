User-confirmed scope + ticket shape for the foundation slice:

Skills to vendor (all 8 confirmed; re-verify each against live registry before adopting):
- Rust: wshobson/agents@rust-async-patterns
- Rust: apollographql/skills@rust-best-practices
- Browser: microsoft/playwright-cli
- Playwright: currents-dev@playwright-best-practices
- WebGPU/3D: heygen-com/hyperframes@typegpu
- WebGPU+Three.js: dgreenheck@webgpu-threejs-tsl
- Interviewing: refoundai/lenny-skills@conducting-user-interviews
- Skill authoring: anthropics/skills@doc-coauthoring

Dioxus skill scope (hand-authored, all 5 areas): signals/state + component patterns; server functions + fullstack data flow; WASM build/bundle + trunk/dx toolchain; integration with this repo's viewer-api managed viewers; styling/asset handling.

Validation: automated prompt-replay matrix asserting the agent loads the correct skill by description per domain, recorded as test-api evidence (satisfies AC7).

Ticket shape: epic + 6 children — (a) contract + re-home + index, (b) vendor adopted skills, (c) author Dioxus, (d) migrate 12 instructions off generator + delete rule-targets agent-guidance targets, (e) prune fixtures + consolidate ultra-granular specs, (f) validation matrix.

Sequencing: skills land first (a → b,c). Migration off the generator (d) and spec prune (e) run strictly last, after skills land. Validation (f) runs after d and e.