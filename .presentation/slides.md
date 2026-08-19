---
theme: seriph
title: context-engine
info: |
  ## context-engine
  A tour of every repository in the superproject tree.
transition: slide-up
mdc: true
---

# context-engine

A superproject of git submodules: shared substrate, viewers, and the
workflow-tools domain-crate suite.

---

## Repository tree

<div grid="~ cols-2 gap-4">
<div>

- `context-stack` — context graph engine (trace/insert/read/search)
- `memory-api` — legacy shared surfaces still resident here (`compact-terminal`, `fs`)
- `memory-viewers` — `ticket-viewer`, `spec-viewer` frontends
- `viewer-api` — shared viewer server runtime (CORS, SSE, static files)

</div>
<div>

- `workflow-tools` — domain-crate suite (see next section)
  - nests `memory-kernel`, the shared filesystem-backed entity store
  - nests `contract-reference`, the minimal domain template

</div>
</div>

---
src: ../workflow-tools/.presentation/slides.md
---
