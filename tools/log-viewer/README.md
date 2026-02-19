# Log Viewer

A modern web-based log viewer for tracing logs from `context-engine` tests, built with Preact + Vite frontend and Axum (Rust) backend. Also includes an MCP server for agent integration.

## Features

### Log Viewing
- **File Browser**: List and navigate log files from `target/test-logs/`
- **Parsed Entries**: Log entries are parsed into structured data
- **Indentation**: Visual hierarchy showing span depth
- **Search**: Regex-based search with match highlighting
- **Filtering**: Filter by log level and event type

### JQ Query Language
- **Full JQ syntax** for filtering logs (powered by jaq)
- Filter by level: `select(.level == "ERROR")`
- Search in messages: `select(.message | contains("panic"))`
- Complex queries: `select(.level == "ERROR" and .span_name == "my_function")`
- Regex matching: `select(.message | test("error|panic"; "i"))`

### MCP Server for Agents
- Run with `--mcp` flag to start MCP server on stdio
- Tools: `list_logs`, `get_log`, `query_logs`, `get_source`, `analyze_log`, `search_all_logs`
- Full JQ query support for powerful log filtering

### Code Integration
- **Source Snippets**: View source code snippets inline with log entries
- **Code Viewer**: Full code viewer tab with syntax highlighting
- **File Navigation**: Click source locations to jump to code

### Visualization Tabs
- **Logs**: Main log viewer with structured entries
- **Flow Graph**: Interactive node graph showing execution flow (Cytoscape.js)
- **Statistics**: Charts showing log level distribution, event types, and timeline (Chart.js)
- **Code**: Full source file viewer with Prism.js syntax highlighting

## Architecture

### Backend (Rust/Axum)
- `src/main.rs` - HTTP server with REST API, or MCP server with `--mcp`
- `src/log_parser.rs` - Parses compact tracing format into structured entries
- `src/query.rs` - JQ query language support via jaq
- `src/mcp_server.rs` - MCP protocol server for agent integration

### Frontend (TypeScript/Preact)
- `frontend/src/` - Modular component structure
- `frontend/src/components/` - Preact components (LogViewer, CodeViewer, FlowGraph, Stats)
- `frontend/src/store/` - Reactive state management with @preact/signals
- `frontend/src/api/` - API client functions

## Usage

### Quick Start (Web UI)

```bash
# Build frontend
cd tools/log-viewer/frontend
npm install
npm run build

# Build and run server
cd ..
cargo run --release
```

Visit http://localhost:3000

### MCP Server Mode (for Agents)

```bash
# Run as MCP server on stdio
cargo run --release -- --mcp
```

The MCP server provides these tools:
- `list_logs` - List available log files
- `get_log` - Read log file with optional JQ filtering
- `query_logs` - Filter logs using JQ expressions
- `get_source` - Get source code snippets
- `analyze_log` - Get statistics and error summary
- `search_all_logs` - Search across all log files

### Development Mode

For frontend development with hot reload:

```bash
# Terminal 1: Start the Rust server
cd tools/log-viewer
cargo run

# Terminal 2: Start Vite dev server
cd tools/log-viewer/frontend
npm run dev
```

The Vite dev server proxies API requests to the Rust backend.

### Configuration

Create a `log-viewer.toml` file (copy from `log-viewer.toml.example`):

```toml
# Directory containing log files
log_dir = "../../target/test-logs"

# Server configuration
[server]
host = "127.0.0.1"
port = 3000

# Logging configuration
[logging]
level = "info"
file_logging = false
```

**Config file search order:**
1. Path in `LOG_VIEWER_CONFIG` environment variable
2. `./log-viewer.toml` (current directory)
3. `./config/log-viewer.toml` (config subdirectory)
4. `~/.config/log-viewer/config.toml` (user config directory)

### Environment Variables

Environment variables override config file values:

- `LOG_DIR` - Override the log directory (default: `target/test-logs/`)
- `WORKSPACE_ROOT` - Override workspace root for source file access
- `LOG_LEVEL` - Override log level (trace, debug, info, warn, error)
- `LOG_FILE` - Enable file logging (set to any value)

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/logs` | GET | List all `.log` files |
| `/api/logs/:name` | GET | Get parsed content of a log |
| `/api/search/:name` | GET | Search within a log file (regex) |
| `/api/query/:name` | GET | Filter logs using JQ expressions |
| `/api/source/*path` | GET | Get source file content |
| `/api/source/*path?line=N&context=M` | GET | Get source snippet around line N |

### Search Parameters (`/api/search/:name`)

- `q` (required): Search query (regex supported)
- `level` (optional): Filter by log level (TRACE, DEBUG, INFO, WARN, ERROR)
- `limit` (optional): Maximum results to return

### JQ Query Parameters (`/api/query/:name`)

- `jq` (required): JQ filter expression
- `limit` (optional): Maximum results to return

**Example JQ queries:**
```bash
# Filter by level
curl 'localhost:3000/api/query/test.log?jq=select(.level=="ERROR")'

# Search in message
curl 'localhost:3000/api/query/test.log?jq=select(.message|contains("panic"))'

# Complex filter
curl 'localhost:3000/api/query/test.log?jq=select(.level=="ERROR" and .span_name=="my_fn")'
```

## Tech Stack

### Frontend
- **Preact** - Lightweight React alternative
- **@preact/signals** - Reactive state management
- **Vite** - Build tool and dev server
- **TypeScript** - Type-safe JavaScript
- **Cytoscape.js** - Interactive graph visualization
- **Chart.js** - Charts and statistics
- **Prism.js** - Syntax highlighting

### Backend
- **Axum** - Web framework
- **Tokio** - Async runtime
- **tower-http** - HTTP middleware (CORS, static files)
- **serde** - Serialization
- **regex** - Pattern matching for log parsing
