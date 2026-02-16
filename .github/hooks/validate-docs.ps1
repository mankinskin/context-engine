# Post-tool-use hook for documentation validation
# Runs after any tool execution in Copilot CLI

# Read JSON input from stdin
$input_json = $input | Out-String

try {
    $data = $input_json | ConvertFrom-Json
    $toolName = $data.toolName
    
    # Check if this was a file edit operation
    if ($toolName -in @("edit", "write", "create")) {
        $filePath = if ($data.toolArgs.filePath) { $data.toolArgs.filePath } else { $data.toolArgs.path }
        
        # Check if the edited file is in the MCP docs server source
        if ($filePath -like "*tools/mcp-docs-server/src/*") {
            Write-Host "⚠️  MCP docs server source modified: $filePath" -ForegroundColor Yellow
            Write-Host "📋 Remember to run documentation validation:" -ForegroundColor Cyan
            Write-Host "   - mcp_docs-server_validate_docs"
            Write-Host "   - mcp_docs-server_check_stale_docs"
            Write-Host ""
        }
        
        # Check if agent docs were modified
        if ($filePath -like "*agents/*" -and $filePath -notlike "*agents/tmp/*") {
            Write-Host "📝 Agent docs modified: $filePath" -ForegroundColor Cyan
            Write-Host "   Consider updating INDEX.md if adding new files"
            Write-Host ""
        }
    }
} catch {
    # Silently fail - hooks shouldn't block execution
}

exit 0
