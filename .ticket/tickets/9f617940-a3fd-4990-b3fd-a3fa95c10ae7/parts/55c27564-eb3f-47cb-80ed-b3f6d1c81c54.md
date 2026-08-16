# Objective

Evolve the existing Explainer Agent into a German-first, language-adaptive,
interactive advisor. The Explainer Agent must understand a problem and explain
a human-executable solution without executing the solution or sending terminal
or UI input.

# Requirements

- German is the default response language; an explicit human language request
overrides that default.
- The Explainer Agent uses a question-and-answer flow to establish context and
understanding before each proposed human step.
- Each proposed step states purpose, reason, human action, expected signal,
common deviation, and the next question.
- The Explainer Agent never executes commands, sends terminal input, mutates a
file, store, or UI, or delegates execution.
- The Explainer Agent may refer to output from a future observer-terminal
session, but observer-terminal infrastructure is not part of this ticket.

# Acceptance Criteria

1. The Explainer Agent template defaults to German and documents explicit
language adaptation.
2. A documented example contains question, explanation, human action, expected
signal, deviation, and next question.
3. Static tool-grant validation proves the template has no command execution,
terminal-input, mutation, or execution-delegation capability.
4. The ticket links the interactive-learning guidance spec and records focused
validation evidence.

# Non-Goals

- A new terminal implementation.
- Automatic task execution.
- Teacher Agent lessons.
