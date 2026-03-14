$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

Write-Host "=== Generating TypeScript types ==="
Set-Location $RootDir

# Clean previous output
Remove-Item -Force packages/context-types/src/generated/*.ts -ErrorAction SilentlyContinue
# Keep .gitkeep
if (-not (Test-Path packages/context-types/src/generated/.gitkeep)) {
    New-Item -ItemType File -Path packages/context-types/src/generated/.gitkeep -Force | Out-Null
}

# Generate types from all crates that have ts-rs derives
Write-Host "Generating context-api types..."
cargo test -p context-api --features ts-gen export_bindings -- --ignored 2>$null
if (-not $?) { Write-Host "  (some context-api tests skipped)" }

Write-Host "Generating context-trace types..."
cargo test -p context-trace export_bindings -- --ignored 2>$null
if (-not $?) { Write-Host "  (some context-trace tests skipped)" }

Write-Host "Generating log-viewer types..."
cargo test -p log-viewer export_bindings -- --ignored 2>$null
if (-not $?) { Write-Host "  (some log-viewer tests skipped)" }

# Count generated files
$count = (Get-ChildItem packages/context-types/src/generated/*.ts -ErrorAction SilentlyContinue | Where-Object { $_.Name -ne '.gitkeep' }).Count
Write-Host "=== Generated $count TypeScript type files ==="

# Build the npm package
Write-Host "Building @context-engine/types..."
Set-Location packages/context-types
npm run build
Write-Host "=== Done ==="
