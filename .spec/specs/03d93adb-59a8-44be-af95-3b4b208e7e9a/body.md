<!-- aligned-structure:v2 -->

# Interactive human learning guidance

## Motivation

The current Explainer Agent is read-only but English-first and not structured
as a guided teaching interaction. Humans need German-first explanations,
questions that establish understanding, and safe human-executable tasks. A
Teacher Agent should turn a problem into a lesson while preserving human
control. A comfortable terminal interface must permit human input and bounded
agent observation without granting the agent command-input authority.

Related implementation tickets:

- [9f617940 Make Explainer Agent German-first and interactive](.ticket/tickets/9f617940-a3fd-4990-b3fd-a3fa95c10ae7/ticket.toml)
- [f3cc69a4 Add Teacher Agent lesson guidance](.ticket/tickets/f3cc69a4-03de-4b45-8b87-a548d5669afe/ticket.toml)
- [ea52bd6f Add human-owned observer terminal sessions](.ticket/tickets/ea52bd6f-aa48-43f5-9228-0bff7190abf8/ticket.toml)

## Dependent expectation

If this specification is implemented, a human can receive a German-first,
language-adaptive explanation or lesson, perform all terminal and UI actions
personally, and receive evidence-backed guidance without any guidance agent
sending input or mutating the human task.

## Contract

### German-first interactive explanation

The Explainer Agent answers in German unless the human explicitly selects
another language. Before each proposed human step, the Explainer Agent uses
questions to establish context or understanding. Each step states purpose,
reason, human action, expected signal, common deviation, and the next question.
The Explainer Agent researches and explains but never executes the task,
sends terminal input, mutates a system, or delegates execution.

### Teacher lessons

The Teacher Agent accepts a problem, system, or learning goal. The Teacher
Agent delegates bounded repository research only to the Explore Agent, then
owns lesson planning, task ordering, explanations, questions, repetition, and
summary. A lesson task names the workspace, human action, expected signal,
verification method, explanation, and next-step choice.

The Teacher Agent may assess observed output and human answers against expected
signals. A lesson task is `completed`, `repeat`, or `open`; the Teacher Agent
does not automatically assign a pass/fail grade to a person.

### Human-owned observer terminal

An observer-terminal session separates input ownership from observation.
Humans enter terminal input. Guidance agents can create or attach to a named
session, read bounded output, and query status, working directory, and exit
state. The guidance-agent API must not expose input sending, arbitrary command
execution, or shell-argument execution within observer-terminal sessions.

The selected UI and transport remain `not-implemented` until the capability
inventory in ticket `0dd23fe6-6892-4d21-9927-4a81584dc77a` establishes the safe
implementation path.

## Guards

The following validation evidence is required before any position becomes
verified:

1. A static Explainer contract check verifies German default, language-adaptive
   wording, question-and-answer structure, and no mutation/input/delegated
   execution grant.
2. A static Teacher contract check verifies the required template structure,
   Explore-only delegation, lesson task schema, and no mutation/input/execution
   grant.
3. An observer-terminal integration check proves human input, bounded
   agent-side output readback, session identity, timeout behavior, and absence
   of every agent input operation.
4. A lesson walkthrough supplies three ordered tasks, an expected-signal check,
   and a `repeat` path without a person-level pass/fail conclusion.

## Positions

| Code reference | Status | Required position |
| --- | --- | --- |
| `.agents/agents/explainer.agent.md` | partial | German-first question-and-answer explanation while retaining a no-execution boundary. |
| `.agents/agents/teacher.agent.md` | not-implemented | User-invocable lesson orchestration with Explore-only research delegation. |
| Human-owned observer terminal service and UI | not-implemented | Human-only input plus bounded agent observation. |
| `compact-terminal-mcp` | implemented but unsuitable | Remains single-shot; it is not an observer-terminal substitute. |

## Non-goals

- Autonomous terminal, UI, file, or store mutation by Explainer or Teacher.
- Agent-side terminal input in a human-owned observer session.
- Automatic person-level grading, certification, or scoring.
- Automatic changes to templates, routing, or permissions based on lesson
  feedback.
- Replacing the existing compact single-shot terminal tool.

## Governing-rule requirement

The rule-introduces-spec mechanism must present this specification as
`partial-with-gaps` until the German Explainer, Teacher Agent, and
observer-terminal positions each have passing guard evidence. The related
per-template tool-grant specification
`ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd` governs the declared agent tool sets.

## Traceability

This specification refines the existing Explainer Agent v1 contract
`0a38b1a5-f752-4368-8ec6-d11e6621177f`. The observer-terminal work follows
the terminal reuse inventory owned by ticket
`0dd23fe6-6892-4d21-9927-4a81584dc77a` before implementation begins.
