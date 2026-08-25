# Repository Direction

- We already have tickets for the restructuring work, and the relevant pieces have been moved into Workflow Tools. Everything is working for now.
- The next step is to review where we are in the plan, what the tickets say, and what the next steps should be based on that plan.
- Long term, the repository should contain almost nothing beyond the context stack.
- Workflow Tools should still be installable as a dependency, either through a script or via Cargo imports, while the source code itself lives in a separate repository.
- We also want a minimal repository as a test fixture: a small example application, a few tickets, a few specifications, and a simple demo of the tools on a very small project.
- The target state is a minimal, green, working setup with very little content, but not an empty repository.
- More extensive fixtures can exist alongside that minimal one.