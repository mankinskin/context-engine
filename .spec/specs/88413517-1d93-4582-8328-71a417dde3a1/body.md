<!-- aligned-structure:v2 -->

# Motivation

The `.agents/agents/` roster needs a deliberate expansion that gives each
template one human-invocable responsibility, removes overlapping proposed
roles, and preserves a clear thread between worktree operations, research,
quality work, and delivery. The resulting contract replaces the previous
14-role taxonomy and nine-template consolidation proposal.

# Dependent Expectation

If this specification is implemented, a human or orchestrator can select an
agent template from a single roster, understand the template's sole
responsibility and boundary, and rely on every template to expose a consistent,
traceable authoring surface.

# Scope

- Add the 15 templates in the target roster below.
- Rewrite `orchestrator.agent.md` to route work across the complete roster and
	retain the red thread across tasks, sessions, and goals.
- Extend `simplify.agent.md` with rule-steward responsibilities for instruction
	files and agent templates.
- Delete `command.agent.md`; terminal work not covered by a specialist falls
	back to the Implement Agent.
- Use one epic and one ticket for each thematic batch on one branch during
	implementation.

# Non-Goals

- Implementing, deleting, or editing any agent template in this specification
	unit.
- Creating a dedicated Search Agent; `explore.agent.md` remains the owner of
	bounded read-only probing.
- Replacing the model-routing contract or the per-template MCP-grant contract.
- Making templates private orchestration-only endpoints.

# Decisions

1. Consolidate the originally proposed 24 agents into 15 new templates by
	 merging overlapping roles.
2. Delete `command.agent.md`; ad-hoc terminal execution falls back to the
	 Implement Agent.
3. Deliver one specification, one epic, and one ticket per thematic batch on
	 one branch.
4. Absorb the Instruction Agent role into `simplify.agent.md`; no separate
	 Instruction Agent template exists.
5. Reject a dedicated Search Agent because `explore.agent.md` owns bounded
	 read-only probing.

# Target Roster

| Batch | Filename | Agent name | Single responsibility | Assigned model tier |
| --- | --- | --- | --- | --- |
| Session & Worktree | `session-bootstrap.agent.md` | Session Bootstrap Agent | Initialize a session end to end: resolve the session UUID, provision and rename the worktree, check in the session and board, and pin task-relevant instructions. | T3 (`GPT-5 mini`) |
| Session & Worktree | `merge.agent.md` | Merge Agent | Integrate a completed feature branch bottom-up, enforce the gitlink invariant, fast-forward only, then tear down the merged worktree and branch. | T1 (`GPT-5.6 Terra`) |
| Session & Worktree | `cleanup.agent.md` | Cleanup Agent | Maintain workspace hygiene through safe temporary-file, duplication, stale-worktree, and stale-branch cleanup with audit and health checks. | T3 (`GPT-5.4 mini`) |
| Research & Writing | `structured-research.agent.md` | Structured Research Agent | Conduct dialectic research by establishing a thesis, expanding evidence, testing an antithesis, and synthesizing a conclusion. | T1 (`GPT-5.6 Terra`) |
| Research & Writing | `online-research.agent.md` | Online Research Agent | Search the web, evaluate source quality, and summarize supported findings. | T3 (`GPT-5 mini`) |
| Research & Writing | `writing.agent.md` | Writing Agent | Produce precise prose that carries a specific argument or knowledge element and turns research into an understandable narrative. | T1 (`GPT-5.6 Terra`) |
| Research & Writing | `framing.agent.md` | Framing Agent | Periodically summarize active research, work, goals, and next tasks into a compact context frame for other agents. | T3 (`GPT-5.4 mini`) |
| Quality & Architecture | `refactoring.agent.md` | Refactoring Agent | Find and apply behavior-preserving improvements to code, documentation, and existing features, reducing duplication and improving comprehension or performance; repair encountered defects when warranted. | T1 (`GPT-5.6 Terra`) |
| Quality & Architecture | `code-architect.agent.md` | Code Architect Agent | Review and improve project architecture, including language-specific design considerations. | T1 (`GPT-5.6 Terra`) |
| Quality & Architecture | `surface-design.agent.md` | Surface Design Agent | Evaluate and improve UI/UX for novice and power users. | T1 (`GPT-5.6 Terra`) |
| Quality & Architecture | `live-validation.agent.md` | Live Validation Agent | Exercise shipped tools, CLIs, and servers directly and report observed behavior. | T3 (`GPT-5 mini`) |
| Ops & Intake | `installer.agent.md` | Installer Agent | Install and update tools and external skills, record installed versions, support reinstall, and run post-install checks. | T3 (`GPT-5 mini`) |
| Ops & Intake | `bug-report.agent.md` | Bug Report Agent | Document a defect with reproducible evidence, create the bug ticket, and link the defect to its owning specification and component. | T3 (`GPT-5.4 mini`) |
| Ops & Intake | `session-learning.agent.md` | Session Learning Agent | Analyze prior sessions and artifacts, extract learning and improvement opportunities, record feedback, and decide whether each finding becomes a bug or feature ticket. | T1 (`GPT-5.6 Terra`) |
| Ops & Intake | `scoping.agent.md` | Scoping Agent | Estimate work, split work into isolated task blocks and phases, build the ticket hierarchy and dependency graph, and support later decomposition. | T1 (`GPT-5.6 Terra`) |

The model names in the roster are bare frontmatter values. The canonical tier
definitions, vendor-qualified dispatch names, and override rules remain in
`.agents/instructions/orchestration/model-routing.instructions.md`.

# Responsibility Boundaries

- Cleanup Agent safely removes stale or temporary workspace material; Merge
	Agent alone integrates a completed feature branch and enforces bottom-up
	fast-forward and gitlink rules.
- Session Learning Agent analyzes completed or historical session artifacts and
	records durable feedback; Handoff Agent packages the current session's state
	for the immediate next actor and does not perform retrospective analysis.
- Structured Research Agent performs dialectic thesis/antithesis synthesis;
	Research Agent continues to perform bounded first-pass research and evidence
	triage for an implementation slice.
- Writing Agent composes an argument or explanatory narrative; Transcription
	Agent preserves and restructures source transcript intent without becoming
	the narrative author.
- Live Validation Agent observes actual shipped CLIs, servers, and tools;
	Testing Agent owns automated test design, test execution, and validation
	evidence.
- Scoping Agent creates estimates, phases, a ticket hierarchy, and dependency
	graph; Ticket Refinement Agent improves a known ticket's scope, criteria,
	fields, and links without becoming the graph planner.
- Refactoring Agent improves existing behavior while preserving contracts; Code
	Architect Agent evaluates architectural direction before or beyond a local
	refactor; Surface Design Agent owns user-facing usability rather than general
	architecture.
- Installer Agent manages tool and skill lifecycle; Session Bootstrap Agent
	initializes an execution session; neither role owns feature implementation.

# Authoring Contract

Every `.agents/agents/*.agent.md` template must contain YAML frontmatter with
exactly these required fields: `name`, `description`, `tools`, `argument-hint`,
`user-invocable`, and `model`. The `tools` value is a list that may mix coarse
capabilities and scoped MCP grants. The `model` value is a bare model name with
no vendor suffix.

Every template body must provide these sections in the stated order:

1. `## MCP Tool Grant`
2. `## Input Contract`
3. `## Scope`
4. `## Constraints`
5. `## Required Workflow`
6. `## Output Format`

Templates must reference applicable rules under `.agents/instructions/**`
instead of copying those rules inline. A template may state only the local
application of a rule needed to establish the template's responsibility,
input, output, or boundary. The relevant instruction path must be named where
the rule affects a decision.

Each template must set `user-invocable: true` and provide a meaningful
`argument-hint` that lets a human invoke the template directly. Each output
format must be user-friendly, complete, and traceable: decision points name
identifiers, repository-relative file paths, commands, evidence, and blockers
explicitly. Output must avoid repeated prose while restating key task, ticket,
specification, and session anchors often enough for a limited-context reader.

# Guards

No `ValidationSpec` guard exists yet because the current unit authors only the
contract. The implementation tickets must add a file-inspection guard that
verifies the roster and frontmatter, and a content-review guard that verifies
responsibility boundaries and instruction references before this specification
can become verified.

# Positions

- `.agents/agents/session-bootstrap.agent.md` through
	`.agents/agents/scoping.agent.md`: not-implemented; the 15 target templates
	defined by this specification do not yet exist.
- `.agents/agents/orchestrator.agent.md`: partial; the existing orchestrator
	must be rewritten to route the full roster and preserve the red thread.
- `.agents/agents/simplify.agent.md`: partial; the existing Simplify Agent must
	absorb the Instruction Agent rule-steward responsibility.
- `.agents/agents/command.agent.md`: deprecated; implementation must remove
	the file and route residual terminal work to the Implement Agent.

# Governing Rule Requirement

The rule-introduces-spec mechanism owned by
`.spec/specs/51ee3a34-7bcf-4c1e-a9a2-4a6f63cb438b/spec.toml` must introduce this
specification as coming soon until the positions above are implemented and the
implementation guards pass.

# Traceability and Evidence

The existing related ticket paths remain recorded in the manifest because the
current session has no assigned ticket. Implementation must organize the active
work as one epic and one ticket for each of the four thematic batches while
retaining the manifest links:

- `.ticket/tickets/c608f5ac-cb7f-424f-ae99-22e75a9477d7/ticket.toml`
- `.ticket/tickets/3c3b42f3-1412-4c73-a531-4567add92a33/ticket.toml`
- `.ticket/tickets/1c850547-c76a-4d65-83c6-133289552661/ticket.toml`
- `.ticket/tickets/fb241a6c-165f-4a5e-bad7-9ac0ab63348b/ticket.toml`
- `.ticket/tickets/46d423d8-0a7e-4dc8-b701-b5c2768f34f7/ticket.toml`
- `.ticket/tickets/ce9edc5b-cb27-4cb8-8802-68a8714c686c/ticket.toml`
- `.ticket/tickets/ea80712b-3506-4b8f-bb36-fc2618aa7b82/ticket.toml`

Review evidence must include a roster inspection that counts exactly 15 new
files, validates the required frontmatter and section order, confirms the
five recorded decisions, and checks that no template duplicates instruction
text governed by `.agents/instructions/**`.

Related specifications:

- `.spec/specs/ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml` defines scoped
	MCP tool grants.
- `.spec/specs/7c9757a7-739f-4dfe-a4de-26f187f3b5aa/spec.toml` defines the
	default tool suite.
- `.spec/specs/a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml` defines model
	cost routing and template model declarations.
- `.spec/specs/39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml` defines
	orchestrator cost-gate enforcement.
- `.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml` defines the
	iteration loop.

# Acceptance Criteria

1. The implementation creates exactly the 15 new filenames listed in Target
	 Roster, with the stated agent names, batches, sole responsibilities, and
	 assigned model tiers.
2. Each new template has one responsibility and does not take ownership of a
	 boundary assigned to another new or existing template.
3. The implementation deletes `.agents/agents/command.agent.md` and directs
	 ad-hoc terminal work to the Implement Agent.
4. The implementation extends `.agents/agents/simplify.agent.md` as the
	 Instruction Agent rule steward and creates no separate Instruction Agent.
5. The implementation creates no Search Agent and retains `explore.agent.md`
	 as the bounded read-only probing owner.
6. The implementation rewrites `orchestrator.agent.md` to route across the
	 full roster and preserve a clear red thread between tasks, sessions, and
	 goals.
7. Every template has the six required frontmatter fields, with a list-valued
	 `tools` field and a bare, vendor-free `model` value.
8. Every template contains the six required body sections in the required
	 order.
9. Every template is directly human invocable through `user-invocable: true`
	 and a meaningful `argument-hint`.
10. Every template names applicable `.agents/instructions/**` paths rather
		than repeating the corresponding rules, except for the minimal local
		context needed by the template contract.
11. Every template output format identifies relevant ids, repository-relative
		file paths, commands, evidence, decisions, and blockers explicitly while
		avoiding unnecessary repetition.
12. The implementation maintains the six stated responsibility boundaries for
		Cleanup/Merge, Session Learning/Handoff, Structured Research/Research,
		Writing/Transcription, Live Validation/Testing, and Scoping/Ticket
		Refinement.
13. The implementation work is traceable through one epic and one ticket per
		thematic batch on one branch, with roster and authoring-contract review
		evidence attached before the specification moves to review.