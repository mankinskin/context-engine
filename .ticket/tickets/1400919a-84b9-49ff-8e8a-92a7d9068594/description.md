# Goal
Track and resolve high-impact tool-calling reliability and session-evidence quality issues observed in session `a0228f9f-bbac-4c82-b1e6-8a628aa91ec1`.

## Scope
- Terminal command lifecycle reliability for long-running `run_in_terminal` sync calls.
- Session artifact fidelity for post-hoc debugging and handoff quality.
- Event-stream noise reduction for session optimization workflows.

## Related specs
- `9e04ff58-9160-4766-b307-74c0fb32a92c` (`context-engine/handoff-workflow-prompts`)
- `f5e0df47-d0ec-4456-b268-689f2a41ecd7` (`agent-workflow/session-context-packing`)
- `09f96d83-4795-4f19-9259-64ad0d452387` (`context-engine/session-api/vscode-copilot-capture-hook-sync`)

## Done when
All child bug tickets are implemented, validated, and linked to evidence updates in session capture and handoff workflows.
