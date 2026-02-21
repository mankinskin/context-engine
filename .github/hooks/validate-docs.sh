#!/bin/bash
# Post-tool-use hook for documentation validation
# Runs after any tool execution in Copilot CLI or VS Code Copilot Chat

# Path patterns for matching (Unix and Windows style)
DOC_VIEWER_SRC_UNIX="tools/doc-viewer/src/"
DOC_VIEWER_SRC_WIN="tools\\\\doc-viewer\\\\src\\\\"
AGENTS_DIR_UNIX="agents/"
AGENTS_DIR_WIN="agents\\\\"
AGENTS_TMP_UNIX="agents/tmp/"
AGENTS_TMP_WIN="agents\\\\tmp\\\\"

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

# Check if the edited file is in the doc-viewer source
if [[ "$FILE_PATH" == *"$DOC_VIEWER_SRC_UNIX"* || "$FILE_PATH" == *"$DOC_VIEWER_SRC_WIN"* ]]; then
    echo "[validate-docs] MATCH: Doc viewer source file" >&2
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "⚠️ Doc viewer source modified. Run documentation validation: mcp_docs-server_validate_docs and mcp_docs-server_check_stale_docs"
  }
}
EOF
    exit 0
fi

# Check if agent docs were modified
if [[ "$FILE_PATH" == *"$AGENTS_DIR_UNIX"* && "$FILE_PATH" != *"$AGENTS_TMP_UNIX"* ]] || \
   [[ "$FILE_PATH" == *"$AGENTS_DIR_WIN"* && "$FILE_PATH" != *"$AGENTS_TMP_WIN"* ]]; then
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
