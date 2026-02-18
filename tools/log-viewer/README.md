# Log Viewer

A modern web-based log viewer for tracing logs from `context-engine` tests, built with Preact + Vite frontend and Axum (Rust) backend.

## Features

### Log Viewing
- **File Browser**: List and navigate log files from `target/test-logs/`
- **Parsed Entries**: Log entries are parsed into structured data
- **Indentation**: Visual hierarchy showing span depth
- **Search**: Regex-based search with match highlighting
- **Filtering**: Filter by log level and event type

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
- `src/main.rs` - HTTP server with REST API
- `src/log_parser.rs` - Parses compact tracing format into structured entries

### Frontend (TypeScript/Preact)
- `frontend/src/` - Modular component structure
- `frontend/src/components/` - Preact components (LogViewer, CodeViewer, FlowGraph, Stats)
- `frontend/src/store/` - Reactive state management with @preact/signals
- `frontend/src/api/` - API client functions

## Usage

### Quick Start

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

### Environment Variables

- `LOG_DIR` - Override the log directory (default: `target/test-logs/`)
- `WORKSPACE_ROOT` - Override workspace root for source file access

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/logs` | GET | List all `.log` files |
| `/api/logs/:name` | GET | Get parsed content of a log |
| `/api/logs/:name/search` | GET | Search within a log file |
| `/api/source/*path` | GET | Get source file content |
| `/api/source/*path?line=N&context=M` | GET | Get source snippet around line N |

### Search Parameters

- `q` (required): Search query (regex supported)
- `level` (optional): Filter by log level (TRACE, DEBUG, INFO, WARN, ERROR)
- `limit` (optional): Maximum results to return

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
