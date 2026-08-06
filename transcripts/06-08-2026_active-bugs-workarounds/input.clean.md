# Active Bugs and Workarounds

We need a system that makes active bugs visible to agents and provides them with workarounds.

This should be embedded in our guidance as an essential step before using tools.

We should also be able to find active bug tickets for specific tools so we can be selective about what we use.

The system should support incremental inspection of bug tickets:

- Check whether there are any bug tickets for a given tool.
- See how many there are.
- Read the simplest short description of each bug.
- Understand what each bug ticket contains.

Bug tickets should be able to provide a workaround. When we create bug tickets, we should also check whether a workaround exists and, more generally, which bug tickets have workarounds.

That suggests a health check: whenever we touch bug tickets, we should do a health check, look at the overall state, and, when in doubt, improve relevant tickets and quickly fix obvious simple problems.

The core feature here is practical workarounds and an explicit, searchable bug board or guidance for how to search it.

Example: there is currently a bug ticket for worktrees, and that prevents us from reconciling worktrees correctly with the MCP tools. For example, we cannot create tickets in the correct worktree when using the MCP tools. The workaround is to use the CLI tools. Until that bug is fixed, agents should not use the MCP tools to work in a worktree.

Agents need to know this, and it needs to be relatively high in their context so these mistakes happen less often.