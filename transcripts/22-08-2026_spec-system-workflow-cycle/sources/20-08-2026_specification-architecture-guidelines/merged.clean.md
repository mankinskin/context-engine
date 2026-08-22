# Specification Structure and Semantics (Merged)

## Original Guidance

While specifying a new feature, I noticed that we still do not have a clear rule for how specifications should be maintained.

The current problem is that the specifications I see are usually far too broad: they are too large for a single file, and they do not use modular structure or a model of the desired architecture as specification artifacts. That is missing.

A specification should read more like a structured architecture of abstracted components, with cross-dependencies, nesting, and interface contracts. Those contracts should be linked to concrete test cases so they can be verified, and each component should have a short, human-readable description that refers briefly to the component's overall form, the context in which it is used, and its central task.

A good specification should consist of multiple files with references between them that capture semantic relationships. It should also reference specification-external artifacts such as tests, tickets, and documentation so the specification is grounded in the real world. In that way, the specification becomes practically executable.

The specification should collect:

- relationships to external artifacts that provide relevant context;
- relationships to other specifications that define components related to the component in question;
- the expectations that components have of one another, meaning the contracts of their interfaces from each component's perspective; and
- the acceptance criteria used within the specification store.

Acceptance criteria should be short, measurable artifacts or decisions that can be actively validated by one or more tests. A specification can mention or use such an artifact to define which acceptance criteria the component includes.

We need to improve the overall specification structure so that we have clear artifacts for components, acceptance criteria, and the relationships between components.

For relationships between two components, I would model them as a directed edge that binds the contract between the components. Whenever two components are related, we can define a contract for the interface between them.

One component is always the consuming or reading component, and the other is the writing or serving component. The edge references the acceptance criteria that the serving component must satisfy so that the reading or consuming component can consider the contract fulfilled.

For now, we should still allow cycles between components so that we do not become too detailed too early. Two components may also read from and write to each other, or serve and consume each other.

The point is to have the tools needed to model a specification with this semantics. In the end, the specification should model the entire system we are building. That includes components that are manifested in code and can interact by writing to or reading from storage. Those interactions must be modeled as acceptance criteria that are satisfied by the active side and claimed by the passive side.

## Concrete Case Study (folded in)

Consider the example we created recently. A lot of work has gone into it, but the problem is still visible in the specification for the presentation system.

The Presentation System specification (`.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/`) is the concrete case study: it demonstrates the monolithic-document problem described above in a real, already-reviewed specification, and grounds the abstract guidance in an actual artifact that can be examined and later migrated.

## Open Question

Should a component specify only the expectations it has of another component, or should it also define the contract it offers outward to other components?