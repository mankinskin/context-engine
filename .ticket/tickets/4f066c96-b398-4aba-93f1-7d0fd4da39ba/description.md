Implement a compact terminal MCP tool that returns short outputs inline and truncates long outputs automatically.

Requirements:
- short outputs return directly
- long outputs return exit status, short summary, and transient file path
- full command stream is persisted to a transient file for targeted follow-up inspection
- supports systematic follow-up via bounded search/read tools instead of replaying the full output into the main token lane
- document expected behavior and failure cases