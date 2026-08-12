# Agent Template Redesign

We need to improve our agent templates, remove agents that are used too broadly, and replace them with specialized agents that fit our typical work better.

## Remove and replace

- Remove the Command Agent. It is too general, gets used too often, and provides too little value.
- Replace it with specialized agents that are better matched to the tasks we typically need.

## Proposed agents

- Session Bootstrapping Agent: initializes a new session end to end, including the worktree and all required context. It should understand how sessions and worktrees work.
- Merge Agent: merges worktrees or branches.
- Refactoring Agent: finds opportunities to improve code, documentation, or existing features without changing behavior. It should preserve behavior as much as possible, but it may still fix bugs. Its main goals are to improve understanding, reduce duplication, and improve performance.
- Learning Agent: generates learnings from previous sessions, finds errors or improvement opportunities, and helps improve the overall process. It should understand how a session is structured, how to search it, and how to identify what still needs improvement. This could run at the end of a session to review the current session and potentially create follow-up tickets.
- Bug Report Agent: documents a problem, creates a bug ticket, and links it to the right place.
- Instruction Agent: improves instruction wording, creates new agents and instructions, and acts like a shift lead. It should be strong at referencing rules, formulating rules, and removing contradictions or repetition.
- Creative or technical writing Agent: writes clear, precise, high-quality text that is easy to understand and can carry a specific argument or knowledge element.
- Structured Research Agent: works dialectically by building a thesis, expanding and completing it, testing it against an antithesis, and then synthesizing both into a more complete result.
- Scoping Agent: estimates tasks and tickets and splits them into isolated task blocks, larger phases, and a full hierarchy so no single agent has to do too much at once or face too many open questions.
- Cleanup Agent: keeps the local workspace free of temporary files and duplication, and can also run audits and health checks to ensure the system stays clean.
- Explainer Agent: simplifies complex concepts and turns research results into an understandable narrative.
- Session Analysis Agent: analyzes sessions, including multiple sessions together, and evaluates session artifacts.
- Surface Design Agent: evaluates and improves the user interface and user experience for novice users and power users alike.
- Live Validation Agent: tests the tools directly.
- Installer Agent: installs and updates tools, records which versions are installed, and supports reinstalling and testing.
- Code Architect Agent: reviews the entire project from an architecture perspective and improves it, including language-specific considerations.
- Workflow Agent: turns scoped tasks into workflows, assigns tickets and dependencies, and produces a directed graph of the work. It should also support later refinement by splitting tasks into smaller scopes.
- Feedback Agent: collects improvement suggestions and findings from a session, records them, and decides whether they should become bug tickets or feature tickets.
- Skills Management Agent: manages external skills, including updating, downloading, installing, and testing them.
- Online Research Agent: searches for information online and summarizes or evaluates it.
- Removal Agent: deletes unwanted items carefully, with clear instructions, backups, worktrees, and Git support. It must be clear whether something should be removed or recreated.
- Search Agent: performs precise searches for a specific goal, such as finding all occurrences of something or locating a specific path. It should stay small and focused.
- Framing Agent: regularly summarizes the current research, active work, goals, and next tasks into a clear context frame so other agents keep the thread. The transcript's final name here was unclear, so I preserved the role rather than guessing the label.

## Orchestration and rules

- The Orchestrator Agent should be adapted to integrate all of these agents into one workflow.
- Avoid repeating instructions. Prefer reusable instruction files and shared references.
- Every agent should have one clear responsibility and a clean boundary from the others.
- Every agent should also be directly usable by a human.
- Agent output should be user-friendly, complete, and traceable. IDs, file paths, and other important anchors should be named clearly at important decision points.
- Agents should not repeat themselves unnecessarily, but they should restate the key anchors often enough that a limited-context reader can still follow the thread.
- The orchestrator should preserve a clear red thread across tasks, sessions, and goals.
