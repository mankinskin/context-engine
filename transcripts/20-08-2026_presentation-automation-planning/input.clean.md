# Presentation Automation Planning Brief

We have a basic presentation that is not yet rich in content. Each submodule or repository can already provide its own presentation, and those presentations can be combined into one presentation spanning multiple repositories. This is a valuable foundation for the goal of a modular, extensible system.

The next step is to make the presentations informative, understandable, and visually effective. Their primary purpose is to communicate information clearly to readers or listeners, not merely to look attractive.

## Audience and Purpose

The initial audience is software developers who want to write code with AI. The presentation must show how to use the existing repositories for that purpose. It should address human developers and AI agents in the same way: both should be able to understand how the repository is used and how it is intended to be used.

The presentation should provide solution-oriented motivation:

- explain the available tools, their benefits, their design rationale, and recommended uses;
- present the system as workflow tooling, where each tool has a distinct role within a larger workflow;
- show how these workflows can improve over time and automate most individual tasks;
- emphasize that work should run locally where possible, minimizing expensive API usage; and
- ensure that people can always read the data, follow decisions, and assess quality.

## Presentation Structure

Structure the presentation from the outside inward, introducing topics gradually and increasing detail while preserving consistent long-range context across all slides.

Begin with the learner and the contexts of software development, AI assistants, and a software project. Then explain how the tools relate to one another, followed by the components of each tool. In the long term, the hierarchy may reach individual functions or even lines of code. Understanding context is more important than presenting isolated content.

The presentation and any nested presentations for individual components should use the same global scheme. Each slide must have a clear central statement or task, and every slide should contribute to the overall task: introduce newcomers to the tools, provide the repository vocabulary, and explain how the system can be used.

Include an overview diagram of the information flow and tool sequence in a typical session: from input to an output that fulfils the input requirements. It should show verification loops, planning methods, improvement loops, and how the finished output is produced. This gives newcomers a project-level view before they need to understand every component in detail.

## Automated Repository-Derived Content

Keep natural-language content in the presentation to a minimum. Generate descriptions and representations from repository files and stored entries wherever possible, so the presentation stays current without manual work. Allow presentation-specific choices, such as colours and alternative data representations, to be controlled through a configuration file or a dedicated presentation area.

Investigate automated sources and representations including:

- **Repository and code hierarchy:** Use the still-incomplete `Peek` tool to represent Git repositories, Cargo workspaces, Rust crates, and modules as a hierarchy or graph suitable for presentation. Extend the abstraction represented by an abstract syntax tree so it can include crate hierarchies, Cargo workspaces, and Git repositories.
- **Selectable detail:** Allow `Peek` to render hierarchy nodes at different resolutions. For example, render a Rust file as a skeleton of defined functions and types; render a crate as its modules; render modules with signatures; or render a Git repository as a tree of packages or crates.
- **Language scope:** Rust is the primary focus, but the hierarchy should eventually be able to include TypeScript, JavaScript, and Python. The immediate priority is to collect Git repository and crate hierarchies through `Peek` and thereby complete more of its intended functionality.
- **Documentation:** Consider automatically generated Rust documentation as a source. It might be integrated with `Peek`, although it may not provide sufficient control.
- **Public tool surfaces:** For MCP and CLI servers, collect and present public or offered commands as lists, trees, diagrams, help output, or individual command slides.
- **Usage examples:** Present selected tests to demonstrate how tools are used and which behaviour is expected.
- **Specifications:** Present specification entries, potentially after extending the specification tool to generate presentation-ready representations. In a later direction, specifications could become the bridge from which presentations are generated: a specification defines the repository goal, an implementation fulfils it, documentation describes the implementation, and the specification summarizes the feature for presentation to users.

## Visual Language

Define a global visual vocabulary and a consistent mapping from vocabulary objects to their appearance. Keep that mapping coherent across the whole presentation and across nested component presentations.

Use, where appropriate:

- colour coding;
- distinct edge styles;
- a uniform syntax for formal statements;
- typography-based information coding;
- diagrams for multi-faceted relationships; and
- tables and nested lists for many discrete points.

The project has substantial creative freedom to make the presentation clear and compelling, while deriving its content from actual code and memory entries instead of hard-coding it.

## Requested Next Work

Collect the existing information and plans for presentations. Investigate the newly defined ideas, refine them into clearer specifications, plan the implementation and verification steps, and identify open questions or missing specifications. Prioritize capabilities that can be implemented and tested quickly, while planning and preparing lower-priority capabilities for later work.

## Open Questions

- How much control can an integration of automatically generated Rust documentation with `Peek` provide?
- What exact configuration mechanism or presentation area should control visual and representation choices?
- Which selected tests and specification entries best demonstrate usage and expected behaviour?

## Refined Track: Specification-Derived Conceptual Decks

### Settled Decisions

- Generated slide outputs are owned by the generator and are overwritten through an explicit replacement path. Git patch review is the review mechanism for generated changes.
- Specifications are authoritative. Generated slides make specifications conceptual, digestible, and suitable for a live human audience. Implementation and documentation disagreements are recorded as sidecar signals; they do not replace the specification.
- This refined track primarily serves live humans. Broader AI-agent, TypeScript/JavaScript/Python, documentation, CLI/MCP, and test-derived automation remain future work rather than current scope.
- The initial structural scope combines Git repository/submodule topology with Cargo workspace and Rust crate topology. These are distinct named projections, not one implied tree: every node and edge has a type and a source.
- Workflows are extracted from formal declarations. Durable session telemetry may appear only as explicitly illustrative examples and cannot change normative claims. Future end-to-end sessions may become test fixtures.
- Extraction adapters normalize specification facts, Git/Cargo projections, declarative workflows, and optional telemetry examples. `presentation-api` continues to own deck persistence, materialization, builds, and traceability.

### Guardrails and Contracts

- A source lock records specification paths and sections with content hashes, transform and theme/preset versions, and the Git base. A changed lock marks a deck stale or causes regeneration to fail explicitly; it never silently republishes a mismatched deck.
- Generation writes only declared generated paths. It rejects path traversal and symlink escapes, keeps generated sources separate from any human-owned overlay, and preflights unexpected modifications before an explicit replace.
- Each claim carries a source selector and citation and declares whether it is quoted or synthesized. A structured disagreement sidecar records category, severity, owner, resolution state, and source locations. Unresolved material contradictions block publication or visibly qualify the affected slide.
- Visual provenance is `synthetic` or a pinned `snapshot`, never `live` in this track. Generated conceptual slides require presenter notes unless they explicitly declare `no notes required`.
- Theme work is deferred. Before flagship structural slides, define a topology visual preset contract with a required legend, named node and edge roles, density limits, and baseline screenshots.

### Staged Work

1. Define conceptual-input, claim/citation, source-lock, sidecar, overwrite-boundary, and visual-provenance contracts, including discovery/migration for legacy singleton `.presentation/deck.toml` sources and deterministic cross-repository imports before any multi-deck registry becomes canonical.
2. Implement specification and Git/Cargo extraction adapters with fixtures covering containment, workspace/crate membership, and dependency projections independently.
3. Implement deterministic managed deck generation, presenter notes, source locks, and disagreement sidecars with explicit stale and replace behavior.
4. Integrate the generator with `presentation-api`, then validate static output per slide. Cross-language parsing and telemetry-derived normativity are out of scope.

### Verification Plan

- Materialize a managed deck and build static output from its locked inputs.
- Derive the expected slide count from the deck manifest, visit every slide at a fixed viewport, and capture a screenshot for each.
- Assert citation and legend presence where required; fail on console errors or missing assets. Basic title-page checks alone are insufficient.
- Use fixtures to prove typed source projections, source-lock staleness, path containment, deterministic imports, presenter-note coverage, and publication blocking or visible qualification for unresolved material contradictions.

### Remaining Decisions

- The concrete schema and serialization format for source locks, claim citations, and disagreement sidecars.
- The first topology preset's measurable density limits and baseline viewport dimensions.