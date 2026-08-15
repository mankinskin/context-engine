# Verification-First Repository Review

This repository needs a review. It contains the building blocks for general workflows, but two problems are probably the main counterforces to the success of projects like this: verification and cost.

## Verification First

Verification must come first for every request. The request and the solution should be analyzed through the lens of how the solution will be verified. In other words, the solution should be developed backward from the specific verification criteria.

The best verification is deterministic and locally evaluable, but it can also require human or LLM judgment. What matters is that verification closes the self-improvement loop. Every autonomous system takes an input and produces an output, so if the system is going to improve itself, it needs a strategy for verifying its outputs or determining their quality.

We have implemented parts of this already, but verification is not yet as central to our workflow approach or workflow system as it should be.

We need a methodical way to:

- define deterministic criteria from a request,
- derive nondeterministic requirements, criteria, or acceptance conditions when needed,
- find a solution that satisfies all criteria,
- expand, refine, or correct the criteria we have recorded,
- account for changes and flakiness, meaning time-varying results, during verification.

We also need a centralized approach that classifies our tools and our product-specific artifacts. The separation should be clear:

- which tools are our general tools that we offer as a product for other projects,
- which project is the customer project,
- which artifacts on the customer side must be managed by us.

Our product is also structured in many ways like a customer project, and we use our own product to improve itself. That means we are not only optimizing work in our repository; we are primarily optimizing the improvement of a general repository and then applying that back to our own repository. We also use the product in different repositories where the product is deployed.

## Verification, Testing, and Local Quality Checks

This is ultimately about validation and verification of requests. We should use the Test API, the Log API, and the Audit API much more heavily.

We should also use GitHub to run cheap checks repeatedly and to detect missing executions or missing artifacts. We need deterministic linting tools, or tools that operate locally on the machine, so they can surface as many problems as possible in a deterministic way.

A strong form of type checking across the architecture would help. The system should be able to surface problems such as:

- a ticket has too few acceptance criteria,
- a ticket is closed without validation artifacts,
- a ticket is about to be closed even though it has no validation artifacts,
- a test needs review and someone must decide whether the test is good or not.

We should be able to catch or enforce more quality locally, and that should make solution verification more efficient.

## Cost

The second major problem is cost. The current product story is still expensive.

The success story is two-sided. On one side, we have many useful and well-formed tools that support a complete workflow for large software projects. They let us pursue very large goals and very small goals at the same time, connect them to each other, and automate decisions about the next work steps.

On the other side, there are still many sharp edges, large gaps, and non-optimal structures or architecture, and those issues drive cost very high.

The simplest example is our MCP tools. They are useful, but the interface is not well optimized, so the tools are harder to use than they should be. That increases cost and also lowers quality because we do not use agent context efficiently.

These two issues are tightly linked. Efficient verification is the central concern, and the development loop has to close around it. We need to plan and use verification as early as possible on the way to the goal. Verification is the final stop: at the end, all tests and metrics must be green, and we must be able to trust that the tests actually measure what we want.

## Session Plan

This should be treated as a one-shot session.

For now, do not use the tools directly. Start by creating a folder and several Markdown files that describe the thesis, the solution, an index or overview, and a few larger sections. The goal is to present the argument, support it, and outline the solution.

Use the process itself. Think first, and feel free to change the plan until the first edits have been made. After the first edits are made, finish the task without interruption or an interview.

The verification question is the key one: how can you verify that the task has been fulfilled, and how can you make sure that all required points were covered?

## Open Questions

- The exact file names and section boundaries are not specified; only the need for a folder, an index, and several Markdown sections is explicit.
