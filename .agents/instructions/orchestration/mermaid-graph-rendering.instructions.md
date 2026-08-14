---
description: "Use when rendering ticket dependency or execution-order graphs in an agent response."
applyTo: "**"
---

## Single-Block Contract

Render every ticket graph as one `mermaid` fenced code block. The block MUST contain a `subgraph Legend` that explains every edge class present and includes a sample node for every ticket type used. A copied block MUST reproduce the graph's complete meaning without adjacent prose.

## Direction And Edges

Use `flowchart RL`: prerequisites render to the left of the tickets they block, so scanning left-to-right lists blockers before blocked tickets. Read a solid dependency arrow from the blocked ticket on the right toward its prerequisite on the left.

- Stored `depends_on`: `A --> B` means "A is blocked by B"; the arrowhead points at prerequisite `B`.
- Derived execution order: `A -.-> B` means "A runs before B". When both relations describe one pair, the arrows point in opposite directions.
- Stored `linked`: `A -.- B` is an undirected association, not an execution dependency.

## Nodes And Layout

Label each ticket as `<b>{type} · {short-id}</b><br/>{wrapped title}`. Use `htmlLabels: true`, generous `nodeSpacing` and `rankSpacing`, and padded `classDef` styles so wrapped titles are readable. Define and apply a distinct colour class for every type used: `epic`, `task`, `feat` (feature), `bug`, or `external`.

## Reference Diagram

```mermaid
%%{init: {"flowchart": {"htmlLabels": true, "nodeSpacing": 42, "rankSpacing": 66, "useMaxWidth": false}}}%%
flowchart RL
    taskA["<b>task · 659a39a4</b><br/>Verify delegated commands"] --> epicA["<b>epic · 7bc328d7</b><br/>Guidance learnings"]
    taskB["<b>task · cd7aceca</b><br/>Render readable ticket graphs"] -.-> taskA
    taskB -.- extA["<b>external · 2e07430b</b><br/>Ticket CLI defect"]

    subgraph Legend["Legend"]
        direction TB
        depA["<b>depends_on</b><br/>blocked"] --> depB["<b>depends_on</b><br/>prerequisite"]
        orderA["<b>execution order</b><br/>runs first"] -.-> orderB["<b>execution order</b><br/>runs later"]
        linkA["<b>linked</b>"] -.- linkB["<b>linked</b>"]
        legendEpic["<b>epic · id</b><br/>title"]
        legendTask["<b>task · id</b><br/>title"]
        legendFeat["<b>feat · id</b><br/>title"]
        legendBug["<b>bug · id</b><br/>title"]
        legendExternal["<b>external · id</b><br/>title"]
    end

    classDef epic fill:#e6d5f7,stroke:#6b3fa0,color:#241133,padding:12px
    classDef task fill:#d6ecff,stroke:#246a92,color:#102b3c,padding:12px
    classDef feat fill:#d9f2df,stroke:#317044,color:#14351d,padding:12px
    classDef bug fill:#ffe0dc,stroke:#a73d32,color:#451510,padding:12px
    classDef external fill:#f5e7c6,stroke:#806323,color:#3c2f0c,padding:12px
    class taskA,taskB,legendTask task
    class epicA,legendEpic epic
    class extA,legendExternal external
    class legendFeat feat
    class legendBug bug
```