# Agent Builder MVP

We have already started building our own locally running agent builder. It is authenticated with the Copilot API, and it uses the Rig framework.

The first milestone should focus on a single request flow, before any complex session handling:

- Accept an externally attached file from the user.
- Load an agent template from a configurable path.
- Complete the model prompt from that template.
- Use the first tools needed to read files.
- Optionally integrate one simple MCP tool as a proof of concept, such as the ticket tool or a spec-store tool.

Initial constraints:

- The agent runs only in the CLI.
- The agent responds only to the request.
- Complex sessions are out of scope for now.

Validation should be end to end and executable:

- Add a new fixture for the scenario.
- Put agent templates in an agent-templates directory inside the fixture.
- Add a configuration file for the agent builder in the fixture.
- Add a sample input file with information about a fictional person.
- Use the attached file to identify the person.
- Use the MCP-backed store to provide the age in prose, if that path is part of the scenario.
- Ask the agent: "How old is the person?"
- The agent template should require the model to return only a JSON object.
- The test should verify that the agent can produce the correct answer from the prompt template, the attached file, and the MCP tool interaction.

The goal is to prove the whole path once with a realistic fixture, not to build the full session system yet.

If the framework usage is unclear during implementation, inspect the Rig repository or its documentation to understand how the system is meant to be used.