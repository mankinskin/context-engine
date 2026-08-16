Search the repository for a plan for a custom chat assistant.

We want to use a Rust library to build:
- a provider client that can authenticate with our provider and obtain supplemental images from it
- a terminal chat interface
- a Dioxus frontend that runs both in the browser and natively

The chat interface should primarily handle user input and output and support:
- sessions
- opening files in an editor
- a file tree
- tracking changes
- drag-and-drop file insertion
- loading a Git repository
- provider authentication
- MCP tool management

Open questions:
- The opening phrase "im Hotel" was unclear; I treated it as noise rather than a separate requirement.
- "Rush Library Rick" appears to be a mis-transcription of a Rust library or similar term.
- The clause about "supplemental images" from the provider was unclear; I preserved it as a likely provider-client capability, but the exact intent may need confirmation.