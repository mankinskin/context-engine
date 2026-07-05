Roadmap tracker for the 2026-07-05 full audit pass across context-engine and subrepositories.

Audit baseline:
- total findings: 551
- categories: ticket_graph=258, file_length=182, static_complexity=108, compiler_warning=1, test_execution=1, coverage=1
- source artifact: target/tmp/audit-full-2026-07-05.json

Execution order for category resolution:
1) ticket_graph
2) stability-signals (compiler_warning, test_execution, coverage)
3) static_complexity
4) file_length

This tracker depends on category tickets and each category depends on batch tickets.
