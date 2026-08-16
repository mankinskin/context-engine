# Objective

Create a user-invocable Teacher Agent that turns a problem, system, or learning
goal into a structured lesson of human-executable tasks. The Teacher Agent
orchestrates limited Explore Agent research, explanations, questions, task
ordering, repetition, and verification feedback.

# Requirements

- The Teacher Agent delegates repository research only to the Explore Agent.
- The Teacher Agent owns lesson planning, task synthesis, question flow,
explanations, repetition, and summary.
- The Teacher Agent gives humans terminal or UI tasks but never executes a
command, sends terminal input, or mutates files, stores, or UI state.
- A task declares workspace, human action, expected signal, verification
method, explanation, and next-step choice.
- Progress is `completed`, `repeat`, or `open`; the Teacher Agent does not
automatically grade people as pass or fail.
- The Teacher Agent may use observed human output as evidence and asks the
human when its verification is inconclusive.

# Acceptance Criteria

1. A Teacher Agent template has the required frontmatter and authoring
structure.
2. The template permits only the Explore Agent as an internal delegation target.
3. A lesson example contains at least three ordered human tasks and handles a
repeat path without grading the human.
4. Static validation proves no mutation, command execution, terminal input, or
execution-delegation capability is granted.
5. The ticket links the interactive-learning guidance spec and records focused
validation evidence.

# Non-Goals

- Implementing a terminal observer.
- Autonomous remediation or task execution.
- Automatic template updates from lesson feedback.
