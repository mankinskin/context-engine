## Gap

`.agents/agents/Default.agent.md` is nine lines total. Its frontmatter description is the literal placeholder `Describe what this custom agent does and when to use it.`, it declares `model: "GPT-5.6 Terra"`, and it has zero `##` contract sections. Yet `.agents/agents/orchestrator.agent.md` routes `a general task that fits no specialist pattern` to Default at T2. A dispatch receives no task contract.

## Session Evidence

The roster exposes a routing path with no scope, constraints, workflow, or return contract, making an unspecialized dispatch unpredictable.

## Required Corrected State

Present and evaluate both options before implementation: (1) give Default Agent a real contract, preserving the general-task route but requiring defined scope, constraints, workflow, and output; or (2) remove Default Agent from the routing table, eliminating the no-contract route but requiring a named specialist/fallback for general tasks. Record the selected decision. Reference related ticket `4bf9b3b4` `Agent template roster redesign`.