#!/bin/bash
# Post-tool-use hook for documentation validation
# Runs after any tool execution in Copilot CLI or VS Code Copilot Chat

# Read JSON input from stdin
INPUT=$(cat)

# Debug: Log raw input to stderr (shows in VS Code Output panel)
echo "[validate-docs] Raw input (first 500 chars): ${INPUT:0:500}" >&2

# Parse tool name - handle both CLI format (toolName) and VS Code format (tool_name)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // .toolName // "unknown"' 2>/dev/null)
echo "[validate-docs] Tool name: $TOOL_NAME" >&2

# Try to extract file path from various possible locations
FILE_PATH=$(echo "$INPUT" | jq -r '
    .tool_input.filePath // 
    .tool_input.path // 
    .tool_input.files[0] // 
    .toolArgs.filePath // 
    .toolArgs.path //
    .tool_input.replacements[0].filePath //
    "none"
' 2>/dev/null)
echo "[validate-docs] File path: $FILE_PATH" >&2

# Check if the edited file is in the MCP docs server source
if [[ "$FILE_PATH" == *"tools/doc-viewer/backend/src/"* || "$FILE_PATH" == *"tools\\doc-viewer\\backend\\src\\"* ]]; then
    echo "[validate-docs] MATCH: MCP docs server source file" >&2
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "⚠️ Doc viewer backend source modified. Run documentation validation: mcp_docs-server_validate_docs and mcp_docs-server_check_stale_docs"
  }
}
EOF
    exit 0
fi

# Check if agent docs were modified
if [[ "$FILE_PATH" == *"agents/"* && "$FILE_PATH" != *"agents/tmp/"* ]] || \
   [[ "$FILE_PATH" == *"agents\\"* && "$FILE_PATH" != *"agents\\tmp\\"* ]]; then
    echo "[validate-docs] MATCH: Agent docs file" >&2
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "📝 Agent docs modified. Consider updating INDEX.md if adding new files."
  }
}
EOF
    exit 0
fi

echo "[validate-docs] No match - no action needed" >&2
# No message needed - output empty JSON
echo '{}'
exit 0
