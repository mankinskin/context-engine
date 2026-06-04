Goal: coordinate the workspace-wide token-efficiency rollout for agent-facing tooling and guidance.

This tracker groups the immediate follow-up work needed to reduce terminal, file-inspection, and structural-discovery token costs while preserving debuggability.

Planned child tickets:
- compact terminal MCP tool with truncation + spill-to-file behavior
- static `.agent/repo_map.toon` generation and refresh hooks
- token-bounded file inspection utility
- interface skeletonization utility
- pre-flight save lint/format/syntax gate
- guidance updates across agent/repo instruction surfaces

This parent should remain in `new` until the child tickets are completed, then advance through implementation/review as the rollout is validated.