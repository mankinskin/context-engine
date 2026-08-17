# Session Capture Hook and MCP Proxy Routing

The session capture hook and the MCP proxy still have serious problems resolving the active worktree for transcript recording and for correct MCP routing. In practice, the current setup does not work.

The proposed fix starts with code quality. The codebase should meet our standards, and those standards should be updated so that future code changes consistently prioritize quality. In particular, code should be structured into small, clearly defined components with unique responsibilities. That applies to modules, functions, classes, and files. The goal is a clear code hierarchy that mirrors the domain structure and makes the system easier to understand, search, and adapt.

The capture hook already shows the problem: some files are too large, contain unrelated functions, and mix responsibilities that should be reusable elsewhere. The immediate next step is to improve our instructions and guidance so that agents automatically trigger a refactoring when they notice an unstructured library, crate, script, or other code artifact, especially Rust code.

After the guidance and the capture hook and MCP proxy have been improved, the next topic is session routing across worktrees. The intended model is:

1. When a new VS Code session starts, the capture hook initializes the session in the main checkout.
2. The session initially lives in the main checkout and is used normally there. There is no redirection to another worktree yet because the session has not been initialized in a worktree.
3. If the agent later decides that a worktree is needed, it should create the worktree and move the session to it.
4. That move should happen by committing the session entry before creating the worktree, so the transfer is a fixed operation. The worktree control tool could likely provide this as a library.
5. At that point, the main checkout still contains the full session history up to the worktree creation point, and the session entry points to the worktree.
6. The capture hook and MCP proxy should then read the session record in the main checkout to resolve the registered worktree and the session entry inside it.
7. From then on, the capture hook and the proxy should write only to the worktree until the main checkout deregisters the worktree from the session.

Deregistration is also required. One case is when a worktree is merged and deleted. The worktree should normally be clean and have no changes relative to `main` before removal or deregistration. The system then needs a single operation that does not trigger additional hooks and that unregisters the session from the worktree in the main checkout. In other words, the worktree reference in the main session entry should be reset so that the hooks and proxies again reference the main checkout session.

This failure is already visible in a previous session. For example, the `ticket-extraction-finish` worktree invoked an MCP tool to modify a ticket, but the proxy routed the call to the main checkout. The change landed in the main checkout instead of the worktree, and the worktree then had changes it should not have had. That strongly suggests that the capture hook did not register the session in the main checkout correctly.

The earlier session context also matters:

- Ticket `ba4aaa9c` now records the submodule approach as the correct and final decision.
- Ticket domain extraction was completed and validated.
- Eight consumers were repointed from legacy in-tree paths to the `ticket` submodule.
- Legacy `ticket-api`, `ticket`, and `ticket-vscode-core` were removed from the workspace.
- Two real regressions were fixed: rule-cli README/spec drift and `memory-matrix`'s hardcoded `cargo run -p ticket-mcp` invocation.
- All affected packages build and test clean, with two unrelated pre-existing failures left untouched.
- The work was committed on branch `agent/153deb7f-5ba7-41c0-8497-a29955e17f43/ticket-extraction-finish`, and the ticket moved to `in-review`.
- Browser or Playwright E2E for `ticket-viewer` was deferred and the branch was not merged to `main`.
- New domain-extraction tickets were created for spec, audit, test, session, feedback, doc, and cycle remediation.

Open question: should browser E2E verification for `ticket-viewer` be dispatched before merging `ba4aaa9c`, or should the merge wait for manual review?

Source languages detected: German and English. The German content was translated into English and cleaned for grammar, structure, and repetition.
