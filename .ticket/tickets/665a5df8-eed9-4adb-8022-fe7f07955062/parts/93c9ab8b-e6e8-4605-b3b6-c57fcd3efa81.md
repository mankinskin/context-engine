# Kernel neutrality and domain extension-trait policy

Create `.agents/instructions/engine/kernel-layering.instructions.md` to state that `memory-kernel` contains neutral data and neutral contracts only. Domain-specific types, keys, and semantics must not enter `memory-kernel`; domain APIs express their behaviour as extension traits over the neutral data.

Use `TicketManifestExt` in `memory-api/crates/ticket-api` as the reference implementation: ticket-manifest behaviour layers over kernel extra-map keys without turning ticket semantics into kernel semantics. State that when a contract is duplicated in `memory-kernel` and a domain crate, `memory-kernel` owns the definition and the domain crate re-exports or implements the contract. Cite the resolved `InteroperableArtifact` duplication between `memory-kernel` and `test-api`.

The policy is needed because resolving the duplicated `InteroperableArtifact` required manual implementor analysis. An explicit kernel-neutrality rule would have made the ownership decision direct and would prevent future domains from placing domain semantics into the shared base layer.

Placement options and consequences: extending `.agents/instructions/engine/core-crates.instructions.md` would place a `memory-kernel` rule inside an instruction scoped only to context-engine core crates, leaving memory-api domain authors outside the stated audience. Adding the rule to a new `kernel-layering.instructions.md` makes the `memory-kernel` boundary explicit and reusable by every domain extraction; the new instruction is the required target.