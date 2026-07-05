# Goal
Clear immediate quality-signal blockers: compiler warning, test execution failure, and coverage failure.

# Planning Scope
This category is intentionally small and should complete quickly after ticket_graph.
Batch sequence:
1. compiler_warning
2. test_execution
3. coverage

# Implementation Strategy
- Resolve warning source first to stabilize compilation output.
- Reproduce and fix failing test command with focused scope.
- Restore coverage gate by either improving tests or narrowing false-positive instrumentation.

# Validation Plan
- Compile affected crate set with warnings visible.
- Run the failing test target and adjacent suites.
- Run coverage command path used by audit and confirm threshold pass.
- Re-run audit summary by category and confirm all three signals are zero.

# Done Criteria
- compiler_warning, test_execution, and coverage categories each report zero findings.
- Any temporary allow or ignore is removed or documented with follow-up ticket.