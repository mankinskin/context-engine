# Guidance and Specification Simplification Mission

We want to start a mission to massively optimize and simplify the repository's guidance corpus because we are seeing symptoms of confusion, task forgetting, and over-focusing on unimportant details.

The next session should be able to operate freely. We have already disabled loading these context files so it can work without being constrained by the current corpus. Its job is to shorten large parts of the rules, rewrite them in a tighter form, and completely overhaul the specification layer.

## Core principles

- Prefer minimal but correct over detailed but incorrect.
- Cut at least half of the existing text.
- Keep only the core that is actually useful and important.
- If something cannot be expressed precisely, omit it.
- Everything that remains must stay 100% correct.
- Do not over-specify.

## Guidance corpus

Start with a top-down inventory of all files.

- List the files first.
- Decide from the title and purpose whether each document is worth keeping.
- Keep only the important documents.
- Most guidance files will probably remain, but they should be reduced in size.
- Search parent templates for links into stores.
- Collect those links, follow them, and move the relevant content into guidance files where appropriate.
- Remove or rewrite the old links so the corpus becomes self-contained.
- The final guidance corpus should not depend on the content of any store.
- Guidance files should link only to other guidance files, not to transient artifacts.
- The files should still work even if the stores are completely empty.

## Agent templates and instructions

The structure of agent templates should be revised more aggressively around workflow.

- Describe the workflow in clear steps.
- Explain what the agent should do in each step.
- Be more explicit about allowed actions and execution order.
- Say less about scope limits and prohibitions.
- Link to available tools and other stable guidance where needed.

Instruction files should be used as narrowly scoped task guides.

- Their frontmatter descriptions should clearly state what each instruction file does.
- They should stay short.
- Each instruction file should describe one closed work step, not an entire class of work steps.
- If a task needs more coverage, combine several small instruction files instead of one large one.

## Specifications

The specification layer needs a major redesign.

- Delete specifications we no longer need.
- Massively shorten the specifications we keep.
- Make the specification format much more compact and more precise.
- A spec should be short, abstract, and exact.
- It should describe only what we want, not what we do not want.
- It must remain 100% correct.
- If something is uncertain or hard to express, leave it out.
- Only record what we are completely sure about with respect to the product's core functionality.
- Do not use specs to define agent guidance or instructions.
- Remove links to specs, tickets, or other hard facts from agent rules, instructions, and prompts.
- Agent files should be free text and should only link to other guidance files.

The spec structure should also change.

- Separate acceptance criteria from the main prose.
- Separate links to other hard facts from the descriptive content.
- Store acceptance criteria in a structured list format instead of free text.
- Keep free text only for the informal description of the object.
- If needed, provide a full Markdown rendering with metadata as a derived overview, but focus first on extracting metadata into structured files.

## Measurement and migration

Measure progress from the start and measure success at the end.

- Record file sizes at the beginning.
- Track total byte size.
- Track how the distribution shifts toward fewer large files.
- Compare the final state against the reduction target.
- Add migration tooling to move the repository to the new specification schema.
- Adapt the specification tools to support the new structure.

Once the corpus has been moved, do another pass over it and shorten it further with a stronger focus on productivity and focus.