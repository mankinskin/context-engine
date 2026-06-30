# Summary
Create an implementation-ready design for a single resilient path normalization utility kernel in memory-api focused on Unix-style canonical path rendering across Windows/Unix environments, including explicit canonicalization failure surfaces.

# Scope
- Audit all path normalization call sites in memory-api and CLI/MCP consumers.
- Add targeted UNC/verbatim-prefix regression guard tests (initially pending if kernel changes are not yet implemented).
- Consolidate related tickets under this root tracker and map dependencies.
- Produce implementation hand-off design with locked decisions + explicit open questions.

# Related tickets
- 59d96577-09a8-44a7-b0ea-3d51b3a6fb05
- 6e5306fb-c1b3-4aec-991d-fabaf3096e23
- 21e6c015-55c6-4807-8d55-16193ed687ed

# Deliverable
Session ends with unambiguous implementation hand-off plan and decision log.