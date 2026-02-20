#!/bin/bash
# Stop hook - validates documentation before agent session ends
# Blocks completion if MCP docs server was modified without doc validation

# Read JSON input from stdin
INPUT=$(cat)

# Debug output to stderr
echo "[validate-docs-stop] Checking for uncommitted MCP docs server changes..." >&2

# Check if this is already a continuation (prevent infinite loop)
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false' 2>/dev/null)
echo "[validate-docs-stop] stop_hook_active: $STOP_HOOK_ACTIVE" >&2
if [[ "$STOP_HOOK_ACTIVE" == "true" ]]; then
    echo "[validate-docs-stop] Already continued once, allowing completion" >&2
    echo '{}'
    exit 0
fi

# Get transcript path to check what was modified
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '.transcript_path // empty' 2>/dev/null)
echo "[validate-docs-stop] Transcript path: $TRANSCRIPT_PATH" >&2

# Check if doc-viewer backend files were modified in this session
MCP_MODIFIED=false
if [[ -n "$TRANSCRIPT_PATH" && -f "$TRANSCRIPT_PATH" ]]; then
    if grep -q "tools/doc-viewer/backend/src/" "$TRANSCRIPT_PATH" 2>/dev/null; then
        echo "[validate-docs-stop] Found doc-viewer backend changes in transcript" >&2
        MCP_MODIFIED=true
    fi
fi

# Also check git for uncommitted changes to doc-viewer backend
if git diff --name-only 2>/dev/null | grep -q "tools/doc-viewer/backend/src/"; then
    echo "[validate-docs-stop] Found uncommitted doc-viewer backend changes in git" >&2
    MCP_MODIFIED=true
fi

echo "[validate-docs-stop] MCP modified: $MCP_MODIFIED" >&2

if [[ "$MCP_MODIFIED" == "true" ]]; then
    echo "[validate-docs-stop] BLOCKING - require validation" >&2
    # Block completion and ask agent to run validation
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "decision": "block",
    "reason": "Doc viewer backend source files were modified. Please run mcp_docs-server_validate_docs and mcp_docs-server_check_stale_docs to verify documentation is up to date before completing."
  }
}
EOF
    exit 0
fi

echo "[validate-docs-stop] No MCP changes, allowing completion" >&2
# No MCP changes, allow completion
echo '{}'
exit 0
