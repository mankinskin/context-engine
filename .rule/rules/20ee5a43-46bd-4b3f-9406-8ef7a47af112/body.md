### Discovery Before Creating

Always search for an existing spec before creating a new one. Duplicate specs weaken the repository contract.

Prefer updating a matching spec when:
- the behavior belongs to the same component and scope
- the existing spec can absorb the acceptance criteria without becoming unfocused
- the requested change is a refinement rather than a new contract slice

Create a new spec when:
- the requested behavior is a distinct contract slice
- the existing spec would become too broad or mix unrelated concerns
- the new work needs its own acceptance criteria and evidence trail