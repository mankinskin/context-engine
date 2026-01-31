# MCP Docs Server

A Model Context Protocol (MCP) server for managing structured agent documentation.

## Features

- **Create documents** from type-specific templates with automatic dating
- **List documents** by category with metadata
- **Update metadata** (confidence, tags, summary, status)
- **Search by tag** across all categories
- **Regenerate INDEX** files from document frontmatter
- **Validate** naming conventions and structure

## Document Types

| Type | Directory | Purpose |
|------|-----------|---------|
| `guide` | `guides/` | How-to guides and patterns |
| `plan` | `plans/` | Task plans before implementation |
| `implemented` | `implemented/` | Completed features |
| `bug-report` | `bug-reports/` | Bug analyses |
| `analysis` | `analysis/` | Algorithm comparisons |

## Installation

### Build

```bash
cd context-engine/agents/mcp-docs-server
cargo build --release
```

### VS Code Integration

Add to `.vscode/mcp.json`:

```json
{
  "servers": {
    "docs": {
      "type": "stdio",
      "command": "${workspaceFolder}/context-engine/agents/mcp-docs-server/target/release/mcp-docs-server",
      "env": {
        "AGENTS_DIR": "${workspaceFolder}/context-engine/agents"
      }
    }
  }
}
```

## Tools

### `create_doc`
Create a new document from template.

**Parameters:**
- `doc_type`: "guide", "plan", "implemented", "bug-report", or "analysis"
- `name`: Short name (becomes UPPER_SNAKE_CASE in filename)
- `title`: Human-readable title
- `summary`: One-line summary for INDEX
- `tags`: Array of tags (without #)
- `confidence`: "high", "medium", or "low"
- `status`: (plans only) "design", "in-progress", "completed", "blocked", "superseded"

**Example:**
```json
{
  "doc_type": "guide",
  "name": "error handling",
  "title": "Error Handling Patterns",
  "summary": "Common error handling patterns in context-search",
  "tags": ["error-handling", "patterns"],
  "confidence": "high"
}
```

Creates: `guides/20260131_ERROR_HANDLING.md`

### `list_docs`
List all documents of a specific type.

**Parameters:**
- `doc_type`: Document category

### `update_doc_meta`
Update metadata for an existing document.

**Parameters:**
- `filename`: Document filename
- `confidence`: (optional) New confidence level
- `tags`: (optional) New tags (replaces existing)
- `summary`: (optional) New summary
- `status`: (optional) New status

### `search_docs`
Search documents by tag across all categories.

**Parameters:**
- `tag`: Tag to search for

### `regenerate_index`
Rebuild INDEX.md from document frontmatter.

**Parameters:**
- `doc_type`: Document category

### `validate_docs`
Check all documents for convention compliance.

## Document Structure

All documents use YAML frontmatter:

```markdown
---
confidence: 🟢
tags: `#tag1` `#tag2`
summary: One-line summary
status: 📋  <!-- plans only -->
---

# Title

## Section
...
```

## Templates

Each document type has a tailored template:

- **Guide**: Problem → Solution → Example → Common Mistakes → Related
- **Plan**: Objective → Context → Analysis → Execution Steps → Validation → Risks
- **Implemented**: Summary → Changes → API → Migration → Testing
- **Bug Report**: Symptoms → Reproduction → Root Cause → Location → Fix → Verification
- **Analysis**: Overview → Findings → Comparison → Conclusions → References
