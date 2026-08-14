## Gap

`.agents/agents/research.agent.md` is 49 lines. Line 29 explicitly permits questions: `Ask concise follow-up questions only when a focused search still leaves a material ambiguity.` The permission contradicts the shared terminal return contract proposed by C1. The Output Format at line 40 (lines 42-49) lists research question, sources checked, key findings, remaining ambiguity, and a single recommended next action, but does not separate observed fact from inference or distinguish a stale plan from work not yet implemented.

## Session Evidence

A correction pass was required repeatedly because restructuring-session findings conflated inference with verified facts and mistook stale plans for incomplete implementation. Read-only dispatches also ended in questions instead of reports.

## Required Corrected State

Adopt the shared terminal contract and add explicit output fields: `fact | inference | stale-or-pending | evidence`.