# Work Trees, Sessions, and MCP Tool Resolution

The remaining open question is resolved as follows: the only required parameter should be the session ID. From that session ID, the system should derive the workspace, meaning the domain-specific folder that contains the store database in the Memory API. In the current model, the workspace is defined by and derived from the session ID.

That means the session ID points to the work tree, and the work tree is the workspace for the tool. Every tool should resolve its workspace directory through the session's work-tree directory. The session ID is the anchor, and the work tree is what it resolves to.

There is one important constraint: work-tree-based operation assumes a Git repository, but the Memory API now allows a workspace to exist in the directory hierarchy without being bound to a Git repository. That capability should be kept offline for now.

To support this, add an explicit workspace path or relative workspace path in addition to the work tree. By default, this should resolve to the current directory or the work tree, but it can also be used explicitly to address a workspace nested deeper inside a work tree. Coordinating that is the agents' responsibility.

Normally, tool calls should operate automatically in the root directory of the work tree. That resolution must happen through the session ID, so a tool call ultimately receives only the session ID and the work-tree working directory is set automatically during initialization. That work-tree directory then becomes the default workspace used to resolve tool-call targets such as the ticket workspace.

The relative workspace path would provide an optional parameter that agents can pass to tools when they need to address a deeper workspace inside a work tree.

## Updated direction

- The proxy resolves the work tree from the session ID.
- The session ID is the only required anchor for tool calls.
- The work tree becomes the default workspace for tool resolution.
- An explicit workspace path or relative workspace path is optional and can be used to target a nested workspace.
- The Memory API's non-Git workspace capability should remain available, but not become the default behavior for this path.

## Open question

The implementation still needs to define exactly how the optional workspace path is represented in the tool and session APIs, but the resolution model itself is now clear.