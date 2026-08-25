This is a classic breakdown in state management and context propagation. When subagents "step in the dark" and replicate work, or loop endlessly on failures, it means your orchestrator is treating them like independent chat bots rather than stateless functions in a structured workflow.
To fix this, you need to transition from a loose conversation to a strict, centralized ledger system where the orchestrator rigidly controls the inputs and enforces standardized reporting.
Here is how to solve these three specific failure modes:
## 1. Fix "Stepping in the Dark" with a Centralized Ledger
Subagents should never just return a block of conversational text. They must update a global state graph or centralized ledger before handing control back to the orchestrator.
Force subagents to return a strictly structured JSON payload using schemas (e.g., Pydantic). The orchestrator should refuse to accept an update unless it fills out this specific ledger:

{
  "task_status": "COMPLETED | FAILED | ESCALATED",
  "key_findings": [
    "Found that API version 2 is deprecated.",
    "Retrieved customer ID: 99281"
  ],
  "artifacts_produced": {
    "download_link": "s3://bucket/report.csv"
  },
  "failed_attempts": [
    "Tried searching Endpoint X, returned 404."
  ]
}

Why this works: The orchestrator reads this explicit ledger to update its global memory, meaning it always knows exactly what has been done without parsing vague paragraphs of chat text.
## 2. Stop Duplicate Research with "Delta Context"
Subagents are doing repeat research because you are likely passing them the entire historical conversation, forcing them to guess what is relevant. Instead, implement a Task-Specific Context Compiler.
Before a subagent is spun up, the orchestrator must dynamically build a fresh, minimal prompt containing only three things:

* The Micro-Goal: "Analyze the CSV file located at path/to/file."
* The "What is Already Known" Delta: "Prior research established that: [Insert precise key_findings from previous agents]."
* The Constraints: "Do not search Endpoint X (already attempted and failed)."

Rule of thumb: Never let a subagent read the raw thought logs of a previous subagent. The orchestrator must act as the filter, passing only the condensed facts.
## 3. Kill the Infinite Loop with an "Escalation Budget"
Subagents keep looping because they are trapped in an autonomous Try -> Fail -> Fix loop without a circuit breaker. You must strip them of the authority to self-correct indefinitely.

* Set a max_retries counter (Budget = 1 or 2): If a subagent calls a tool and gets an error, it is allowed one attempt to fix its own parameters.
* Force Escalation: If the second attempt fails, the subagent code must explicitly trigger a FAILED status and return control to the orchestrator alongside the exact error log.
* Orchestrator Routing Rules: When the orchestrator receives a FAILED status, it should execute a deterministic fallback rule (e.g., routing to a different agent, notifying a human, or gracefully failing the overall run) rather than blindly asking the same subagent to "try again."

------------------------------
## Suggested State Flow Architecture

[User Request] 
      │
      ▼
┌──────────────┐      Compiles minimal context      ┌───────────────┐
│ Orchestrator │ ─────────────────────────────────> │  Subagent A   │
│   (Ledger)   │ <───────────────────────────────── │ (Task Exec)   │
└──────────────┘    Returns Schema (Findings/State) └───────────────┘
      │
      ▼ (Updates Ledger)
┌──────────────┐      Compiles Context + A's Findings   ┌───────────────┐
│ Orchestrator │ ─────────────────────────────────────> │  Subagent B   │
│   (Ledger)   │ <───────────────────────────────────── │ (Research Exec)│
└──────────────┘     Fails twice -> Hits Max Retries   └───────────────┘
      │
      ▼ (Detects Failure Budget Exceeded)
[Graceful Exit / Human Checkpoint]

To give you the exact code template to fix this:

* What framework are you currently using to manage this architecture (e.g., LangGraph, CrewAI, AutoGen, or a custom Python loop)?
* Are your agents communicating via natural language text, or are you already using JSON/Function calling for their outputs?


