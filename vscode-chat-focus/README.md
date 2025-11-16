# Chat Focus Extension

Simple VS Code extension to provide a command for focusing GitHub Copilot Chat.

## Installation

1. Install dependencies (if needed): `npm install`
2. Open this folder in VS Code
3. Press F5 to run the extension in a new Extension Development Host window
4. Or package and install: `npm run package` then install the `.vsix` file

## Usage

### From Command Palette
1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
2. Type "Focus Copilot Chat"
3. Press Enter

### From Terminal
Once installed, you can trigger it from scripts or terminal using:
```bash
code --command chat-focus.focusChat
```

Note: The `--command` flag may not be available in all VS Code versions. In that case, you'll need to use keyboard shortcuts or the command palette.

## Adding to Your Workflow

You can append this command to terminal commands in scripts:
```bash
cargo test && code --command chat-focus.focusChat
```

Or create a shell function:
```bash
function focus_chat() {
    code --command chat-focus.focusChat 2>/dev/null || true
}

# Then use it:
cargo test; focus_chat
```
