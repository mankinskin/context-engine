# Plan: Repository as a Self-Improving Agent Environment

## Goal

The entire repository must act as an environment in which the AI model (behind the LLM API) can operate autonomously. Within this environment the agent should be able to:

- Discover all available tools on its own.
- Find the next tasks that need to be done, using those tools.
- Build its own workflow, which it then follows.

This process can potentially be unbounded: the agent runs, updates its own workflow, or spawns sub-agents that either build their own workflows or are handed a workflow to execute, and that eventually finish.

The agent should also use the tools to interact with the user and proactively gather feedback — interview answers, reviews, decisions, ratings, and similar signals. It uses that feedback to keep improving its own workflows, the tools, and the environment, so it becomes progressively more successful.

The complete repository should provide everything needed to build this self-improving system from zero with an AI agent/model. This means the repository needs:

- A root entry point.
- Inside it, the individual tools and the guidance files — skills or pre-built workflows.
- A target project that is separate from all of this, or that can even be one of the tools itself.

## Current Situation vs. General Case

Right now we are working on the very tool that we are using: in our case the tool being improved is the tool itself, so the tasks refer to the tool itself. However, we design for the **general case**, where our tools — or our system as a whole — work on *some* tool. It does not matter what that tool is; we optimize our system so it can solve all kinds of tasks.

We want to clearly distinguish the environment of our tools themselves — individual, standalone systems, possibly with dependencies on other systems we manage, but each with a clear responsibility. Each tool's responsibility must be generally applicable and independent of our specific use case.

## Example Mapping onto the Current Repository

For our concrete example:

- The **context stack** is the example environment we want to improve. It is our application / **target environment**.
- Everything else around it — that is, everything except the context stack — is the **tooling** we want to improve, which is currently still in development and planning. This whole repository exists to develop that tooling.

Within this, we can separate:

- **Tool-generated artifacts** — the data our tools use and produce, stored for this repository. This is everything associated with the tools *except* their installed instance and program code — only what they generate.
- **Tool source code** — the code of the individual tools themselves.

## Restructuring into a General Framework

We want to convert the current repository into a more clearly *instantiated example* of our general framework, and to establish that framework in a separate repository that contains only the part concerning the tools themselves.

- Everything concerning our workflow tools moves into a new repository — we could call it **workflow-tools**.
- This workflow-tools repository becomes a **dependency** of the context-engine repository. The context-engine repository only *uses* these tools and keeps its own generated artifacts in its own repository.
- Everything that has nothing to do with our goal moves into the workflow-tools repository, so that context-engine becomes a genuinely instantiated example of an application (a target environment) that uses our workflow tools.

Later, the workflow tools can be removed entirely from the context-engine repository as a submodule and specified only as a dependency that must be installed. To use the workflow tools — or to use the workflow artifacts — in the context-engine repository, the workflow tools must be installed.

## Self-Referential Artifacts and Nested Stores

Inside the workflow-tools repository we then keep the artifacts that we apply self-referentially to the workflow tools themselves — for example, tickets for the ticket system itself. If we improve the ticket system, we still create tickets for that work; those tickets are artifacts and would appear in the ticket-specific workflow tool.

The intent is to nest and separate the current system further using the workflow tools, and to further exploit the feature that our Memory API stores can reference each other across different workspace levels and across different stores. This lets us:

- Localize tasks more strongly and separate them more cleanly.
- Crystallize this hierarchy explicitly.

## Structure of the Workflow-Tools Repository

We will create the workflow-tools repository containing the workflow tools. In it, each individual workflow tool is stored as an isolated tool, including:

- All of its transports (all of its instantiated realizations).
- Its artifacts specific to that tool.
- Its source code.

Shared libraries — or tools that themselves use other tools — would also get their own repository and would be referenced as a dependency by other tools.

The workflow-tools repository as a whole then has its own layer of artifacts — tickets, specifications — that affect several or all tools. In addition, each individual tool has its own artifacts that affect only that tool.

## Guidance Files (Skills / Workflows)

The guidance files introduce an additional area that is not source code but rather a procedural schema — the workflow that uses the tools. We must be able to reuse this, because we want to use it in three places:

- In the context-engine repository, to use the workflow tools.
- In the workflow-tools repository itself, to work on the workflow tools or the guidance itself (the skills or the workflow).
- In the individual workflow tools, to work on them directly.

Therefore, an agent that starts at any of these places must be pointed to these guidance files. Typically this happens via the entry point (the `AGENTS.md`), but also via the agents folder and the instructions it contains.

However, we will probably offer this guidance primarily via a **skill**. We would offer our workflow tools through a skill that users can install, which effectively takes over the entry point. We install that skill:

- At the workflow-tools root.
- At each individual workflow tool, so an agent can optionally enter there as well.

## Skill Scope Requirements

This workflow-tools skill (the "workflow skill") must be applicable both at the root and at a more deeply nested repository, and it must not conflict or be read twice. We therefore need a clear plan for the scope of our skills and guidance files.

We can assume a single installation of our skill that:

- Knows all the tools and can retrieve and install them.
- Can use the artifacts.
- Can consciously ignore other skill installations in nested repositories.
- Can possibly even uninstall itself.

The end state: a user only has to download our single skill from a site like skills.sh, and it would immediately download and install all repositories/tools and use them correctly.
