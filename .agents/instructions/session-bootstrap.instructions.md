---
description: "Minimal session bootstrap for discovering and pinning task-relevant guidance."
applyTo: "**"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=89330b3b-4d28-4c48-80dd-203311dbe855 slug=context-engine/instructions/session-bootstrap -->

## Session Bootstrap

1. Initialize or resume the durable workspace with the session tools.
2. Search the rule store for guidance relevant to the current task; do not load broad instruction bodies by default.
3. Pin the selected rule URNs to the workspace session.
4. Render the pinned rule instruction set through the session render operation and follow only that focused guidance.
5. Keep ticket, spec, and validation authority in their owning stores.
