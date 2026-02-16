#!/bin/bash
# Post-tool-use hook for documentation validation
# Runs after any tool execution in Copilot CLI

# Read JSON input from stdin
INPUT=$(cat)

# Parse tool name (requires jq)
TOOL_NAME=$(echo "$INPUT" | jq -r '.toolName // empty' 2>/dev/null)

# Check if this was a file edit operation
if [[ "$TOOL_NAME" == "edit" || "$TOOL_NAME" == "write" || "$TOOL_NAME" == "create" ]]; then
    # Get the file path from tool args
    FILE_PATH=$(echo "$INPUT" | jq -r '.toolArgs.filePath // .toolArgs.path // empty' 2>/dev/null)
    
    # Check if the edited file is in the MCP docs server source
    if [[ "$FILE_PATH" == *"tools/mcp-docs-server/src/"* ]]; then
        echo "⚠️  MCP docs server source modified: $FILE_PATH" >&2
        echo "📋 Remember to run documentation validation:" >&2
        echo "   - mcp_docs-server_validate_docs" >&2
        echo "   - mcp_docs-server_check_stale_docs" >&2
        echo "" >&2
    fi
    
    # Check if agent docs were modified
    if [[ "$FILE_PATH" == *"agents/"* && "$FILE_PATH" != *"agents/tmp/"* ]]; then
        echo "📝 Agent docs modified: $FILE_PATH" >&2
        echo "   Consider updating INDEX.md if adding new files" >&2
        echo "" >&2
    fi
fi

# Always succeed - hooks shouldn't block execution
exit 0
