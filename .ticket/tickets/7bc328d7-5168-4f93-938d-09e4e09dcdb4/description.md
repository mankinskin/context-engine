# Repository Guidance and Agent-Template Learnings

This epic collects every documentation and agent-template correction identified by a post-hoc analysis of the workflow-tools restructuring session.

## Workstreams

A. Stale or broken references where documentation and tooling still describe the pre-restructuring architecture. Some of these findings are real build/install breakage rather than prose-only drift.

B. Durable technical policies the restructuring session established but which are documented nowhere: cross-repository git-URL dependencies, memory-kernel neutrality, extraction preconditions, and test-triage and assertion rules.

C. Agent-template and orchestration contract gaps, each backed by a measured failure in the observed restructuring session.

The highest-value single item is the shared sub-agent terminal return contract: three separate dispatches in the session were wasted because agents asked a question instead of delivering their assigned result.

Priority order: fix actively misleading and broken items first; then record the policies that unblock the ten remaining tool extractions; then close the agent-template and orchestration contract gaps.
