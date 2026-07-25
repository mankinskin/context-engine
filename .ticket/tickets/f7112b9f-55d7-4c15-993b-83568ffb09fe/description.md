Vendor the confirmed proven skills.sh skills into the repo, normalized to the contract.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — AC3. Depends on the contract ticket.

Skills to vendor (re-verify each against the live registry via find-skills before adopting):
- Rust: wshobson/agents@rust-async-patterns
- Rust: apollographql/skills@rust-best-practices
- Browser: microsoft/playwright-cli
- Playwright: currents-dev@playwright-best-practices
- WebGPU/3D: heygen-com/hyperframes@typegpu
- WebGPU+Three.js: dgreenheck@webgpu-threejs-tsl
- Interviewing: refoundai/lenny-skills@conducting-user-interviews
- Skill authoring: anthropics/skills@doc-coauthoring

Method: vendor normalized copies into `.agents/skills/<name>/SKILL.md` and commit (no install-on-setup dependency). Record upstream source/provenance in each skill (and/or skills-lock.json).

Acceptance criteria (verifiable):
- AC-1: Each of the 8 skills exists as `.agents/skills/<name>/SKILL.md` in skills.sh-native shape.
- AC-2: Each vendored skill records its upstream source id.
- AC-3: Each appears in the master skills index with a description trigger.
- AC-4: Registry re-verification result recorded (kept vs dropped, with reason if dropped).