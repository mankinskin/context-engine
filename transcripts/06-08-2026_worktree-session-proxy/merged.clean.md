# Work Trees, Sessions, and MCP Tool Resolution

The current problem is that we use agents, sessions, and MCP tools from Visual Studio Code through the GitHub Copilot Chat extension, but MCP tool calls start in the main work tree by default. As a result, those calls often apply to the main work tree instead of the work tree for the actual session.

The current setup starts a session in the main work tree and only tells the agent by instruction to work in a work tree. That is not enough, because when the agent makes an MCP tool call, the call still defaults to the main work tree. This causes changes to be missing from the intended work tree and unrelated changes to end up in the main work tree.

There is already a bug ticket with additional information, and there is also a research session describing the intended direction: use a proxy to enforce that a work tree is selected and that the main work tree is not used by default.

## Proposed direction

Use the existing MCP tool proxy to enforce work tree selection. Extend the integration so that a session in the Session API can declare an active work tree that can change over time. At the beginning, the active work tree would be the main work tree.

The proxy should read the active work tree from the session. That means every tool call should automatically carry the session ID, and the proxy should resolve from that session which work tree applies, then resolve the repository path and current working directory from that information. This resolution must happen for all MCP tools.

The proxy is responsible for session resolution: it decides which working directory is used, and that directory is the work tree directory. The resolved value might be a richer object that tracks whether the scope is a repository, a work tree inside a repository, or the main repository, so that it can also block work in specific work trees such as main.

Tool calls should therefore either include an explicit working directory or workspace, or include a session that performs the resolution. The exact implementation may use either approach, or both, depending on the code and dependency constraints. The goal is to integrate the Session API into the ticket API, the feedback API, and the other stores, including the Session API itself.

## Session storage and lifecycle

A decision is needed about where a session should live in which work tree. One acceptable option is to keep the session stored only in the main work tree. In that workflow, the tools run where they were started, the session is written there, the proxy starts there, and the proxy and tools then find the session and execute their logic in the session's active work tree.

The Session API and session-related tools need a clean redirect model. They must understand that sessions are not written into the target work tree but remain in the main work tree, even though the actual work may happen in another work tree later.

This is also relevant for the capture hook that Copilot runs for us and for session capture. Once a work tree exists, it should be possible in principle to switch into the target work tree there as well. Copilot is therefore also a tool that needs the proxy logic or Session API logic and must know which session is active and where events should be stored, or what the working directory should be.

A new hook may be useful: one that runs at the start of a session, before any tool calls, and initializes the session with a work tree mechanically. The capture hook would then write the session into that work tree.

One additional problem is that if a new session creates its work tree automatically from the main branch, any currently unstaged changes in the main work tree could be lost. The work-tree creation flow therefore needs a way to preserve unstaged changes when the new work tree is created. One possible approach is to create a stash first, using `stash push --keep-index` instead of a normal stash, so the main work tree is copied into the new work tree without changing the main work tree and without losing unstaged changes.

That preservation behavior should remain available as an option even if the default flow does not use a stash. It may be useful when an agent notices that changes are missing, because the mechanism would still exist as a fallback. At the same time, the design should be careful about working actively in main, because uncommitted changes there may later still change.

Another issue is what happens when the topic changes during a session or the work tree needs to change. Creating a brand-new work tree for every request would be expensive and would produce too many work trees, so that should not be the default approach. Reusing the same work tree for the session is the simpler default.

If the work tree name or identity needs to change, the system should allow renaming the work tree instead of creating a new one. That could use the existing `worktree move` mechanism, and the associated branch would also need to be renamed. In that model, the session keeps using the same work tree, and the hooks continue to use that same work tree for the whole session.

Between turns, and before a user request is answered or processed, nothing special needs to happen beyond the normal session initialization. Agents can still rename the work tree when they are working on a different topic, and the session can keep moving forward while overriding previous state as needed.

## Recommended behavior

Each session should start automatically in its own work tree. A session should not be able to operate outside a work tree.

All tools run on the same server, so the model is not that each session gets its own server or its own isolated process. Instead, all sessions share the same tools, and the tools must understand work trees.

The system should create a new mechanical hook that automatically creates a work tree. The relevant information is which sessions are linked to which work trees and which work trees are actively in use.

A clear session lifecycle is also needed so sessions and work trees can be finished, merged, or deleted when they are no longer needed. This suggests new Session API infrastructure or new interfaces and calls for querying this information.

A complete overview of sessions would be useful: all active sessions, or even all sessions overall. A list command for sessions could support parameters that control what is shown, such as detail level, whether completed sessions are included, whether only active sessions are shown, and whether last activity is included. Expired sessions could also be considered for validation.

## Planning points

The plan should cover the following:

- Document exactly which sessions are active in the current workspace.
- Show which work tree each session is using.
- Show which ticket track each session is using.
- Show whether each session is active.
- Show the time of the last activity.
- Add a new hook that runs at the start of a session, before the capture hook initializes the session or any other tool calls run.
- Initialize the session with a work tree mechanically and point the session at that work tree.
- Let the capture hook write the session into that work tree.
- Have the proxy, the capture hook, and all tools resolve the work tree through the session ID.
- Require an explicit session ID for every tool call and prohibit a default workspace selection that is not tied to a session ID.
- Preserve unstaged changes when a new session work tree is created from main, either by default or as an explicit option, so information is not lost during the copy.
- Reuse one work tree per session by default instead of creating a new work tree for every request.
- Allow the work tree and its branch to be renamed when the session topic changes, rather than forcing a new work tree each time.

The anchor for every tool call should be the session ID. That session ID should carry the information about the working directory, and the proxy, the capture hook, and all tools should use it.

## Session and workspace resolution

The open question is resolved as follows: the only required parameter should be the session ID. From that session ID, the system should derive the workspace, meaning the domain-specific folder that contains the store database in the Memory API. In the current model, the workspace is defined by and derived from the session ID.

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