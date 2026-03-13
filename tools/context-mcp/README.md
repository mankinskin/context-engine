# context-mcp

MCP server for the context-engine hypergraph workspace system.

Exposes the entire `context-api` command surface as a single MCP `execute` tool. An MCP client (e.g. an AI agent) sends a `Command` JSON object and receives a `CommandResult` JSON object.

## Architecture

```text
┌─────────────┐
│  MCP Client │  (AI agent, Copilot, Claude Desktop, etc.)
└──────┬──────┘
       │ JSON-RPC over stdio
┌──────┴──────┐
│ context-mcp │  ← this binary
└──────┬──────┘
       │
┌──────┴──────┐
│ context-api │  (WorkspaceManager + execute())
└─────────────┘
```

## Usage

### With Copilot CLI

```bash
copilot --additional-mcp-config @.github/context-mcp-config.json
```

### Standalone

```bash
cargo run -p context-mcp
```

The server communicates over stdio (stdin/stdout) using the MCP JSON-RPC protocol. All diagnostic output goes to stderr.

## Tool: execute

A single tool that accepts any context-engine command.

### Workflow Example

1. **Create:** `{"command": "create_workspace", "name": "demo"}`
2. **Add atoms:** `{"command": "add_atoms", "workspace": "demo", "chars": ["a","b","c","d","e"]}`
3. **Insert:** `{"command": "insert_sequence", "workspace": "demo", "text": "abcde"}`
4. **Search:** `{"command": "search_sequence", "workspace": "demo", "text": "abc"}`
5. **Read:** `{"command": "read_as_text", "workspace": "demo", "index": 5}`
6. **Save:** `{"command": "save_workspace", "name": "demo"}`

### Available Commands

| Category | Commands |
|----------|----------|
| **Workspace** | `create_workspace`, `open_workspace`, `close_workspace`, `save_workspace`, `list_workspaces`, `delete_workspace` |
| **Atoms** | `add_atom`, `add_atoms`, `get_atom`, `list_atoms` |
| **Patterns** | `add_simple_pattern`, `get_vertex`, `list_vertices` |
| **Search** | `search_pattern`, `search_sequence` |
| **Insert** | `insert_first_match`, `insert_sequence`, `insert_sequences` |
| **Read** | `read_pattern`, `read_as_text` |
| **Debug** | `get_snapshot`, `get_statistics`, `validate_graph`, `show_graph`, `show_vertex` |

### Response Format

Successful commands return:
```json
{
  "success": true,
  "result": { ... }
}
```

Failed commands return:
```json
{
  "success": false,
  "error": "error description"
}
```

API-level errors are returned as successful MCP responses (not MCP-level errors) so the agent can read and react to error messages.

### Example Commands

**Create a workspace:**
```json
{"command": "create_workspace", "name": "my-graph"}
```

**Add atoms:**
```json
{"command": "add_atoms", "workspace": "my-graph", "chars": ["a", "b", "c"]}
```

**Insert a sequence:**
```json
{"command": "insert_sequence", "workspace": "my-graph", "text": "abc"}
```

**Search:**
```json
{"command": "search_sequence", "workspace": "my-graph", "text": "ab"}
```

**Read as text:**
```json
{"command": "read_as_text", "workspace": "my-graph", "index": 3}
```

**Save:**
```json
{"command": "save_workspace", "name": "my-graph"}
```

See the tool's JSON schema for the complete list of commands and their parameters.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | — | Tracing filter (e.g. `context_mcp=debug`) |

## Testing

```bash
cargo test -p context-mcp
```
