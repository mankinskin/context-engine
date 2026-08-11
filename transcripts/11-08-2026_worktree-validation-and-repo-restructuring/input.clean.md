# Next Session Focus

We need to make progress and clarify today's tasks. The current focus is not yet fully clear, so I need help identifying the best next steps.

## Current assessment

We are close to finishing the worktree tooling implementation, but several worktrees are still open and the instructions for agents are still unstable. We need to evaluate what still absolutely has to be done so we can work with worktrees safely.

The goal is for every session to receive its own worktree automatically through hooks, commit its work quickly in that worktree, and merge cleanly into the main branches for all submodules. That should isolate changes between sessions. The setup is complicated, but it is mechanically achievable, so we should make it reliable.

## Required behavior

- Each session must get its own worktree automatically via hooks, as originally defined.
- Agents must declare their session ID and worktree for traceability at the start and at the end of a session.
- Wherever possible, the worktree and branch should be named to match the task.
- The worktree tooling has to be stable enough that we can trust it for isolated session work.

## What needs to be validated

We need a way to test this end to end. Ideally, we should be able to start a complete session, possibly through the Copilot CLI tool, observe the behavior, and validate how well the tooling and hooks work.

If needed, we can also inspect a session recording or transcript to see what the hooks did. The capture hook may need to record more detail so that this is easier to verify.

A verification loop is essential. Agents must be able to prove that a change actually fixes the problem, because it is common to think a problem is solved while a small missing detail still breaks the workflow. Agents should verify their work and confirm that the assigned task is really resolved.

## Broader roadmap

The long-term goal is to make this project easier for outsiders to understand. That includes generating a presentation for the individual tools, ideally in a way that can adapt automatically when the project changes.

To get there, we first want to restructure the Git repositories, split the code more cleanly across repositories, add more documentation in README files, and localize more of the memory store to the individual tools. That likely means more submodules and more nested workspaces and stores. There are tickets for that work, and it becomes the next major step once worktree integration is proven safe and supports the workflow.

## Working style

- Gather the relevant tickets and planning artifacts into one place.
- Keep an eye out for anything that goes wrong while doing that.
- Treat tooling problems as actionable, not as noise.
- If something fails or behaves unexpectedly, immediately create a bug ticket, update the instructions, or stop and ask the user.
- For very small issues, fix the problem directly in the session after creating the ticket if needed.
- In particular, agent guidance should be improved quickly when it is clearly wrong or too weak.

## Preferred next step

The next session should collect the relevant tickets and plans, produce an execution plan, and start by validating the complete worktree integration in an isolated session and worktree.

If that validation succeeds, we can move on to the repository restructuring work.