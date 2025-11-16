# PowerShell script to send Ctrl+Alt+C to the active VS Code window
Add-Type @"
    using System;
    using System.Runtime.InteropServices;
    public class WinAPI {
        [DllImport("user32.dll")]
        public static extern IntPtr GetForegroundWindow();
        
        [DllImport("user32.dll")]
        public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder text, int count);
        
        [DllImport("user32.dll")]
        public static extern bool SetForegroundWindow(IntPtr hWnd);
        
        [DllImport("user32.dll")]
        public static extern IntPtr FindWindow(string lpClassName, string lpWindowName);
    }
"@

# Find VS Code window
$processes = Get-Process | Where-Object { $_.ProcessName -eq "Code" }
if ($processes.Count -eq 0) {
    Write-Host "VS Code is not running"
    exit 1
}

$vscodeWindow = $null
foreach ($proc in $processes) {
    if ($proc.MainWindowHandle -ne [IntPtr]::Zero) {
        $vscodeWindow = $proc.MainWindowHandle
        break
    }
}

if ($vscodeWindow -eq $null) {
    Write-Host "Could not find VS Code window"
    exit 1
}

# Focus VS Code window
[WinAPI]::SetForegroundWindow($vscodeWindow) | Out-Null
Start-Sleep -Milliseconds 100

# Send Ctrl+Alt+C
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.SendKeys]::SendWait("^%c")

Write-Host "Sent focus command to VS Code"
