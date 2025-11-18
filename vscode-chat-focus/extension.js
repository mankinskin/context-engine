const vscode = require("vscode");

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
  console.log("Chat Focus extension is now active");

  let disposable = vscode.commands.registerCommand(
    "chat-focus.focusChat",
    async function () {
      let focused = false;

      try {
        // Focus the Copilot Chat view
        await vscode.commands.executeCommand(
          "workbench.panel.chat.view.copilot.focus"
        );
        focused = true;
      } catch (error) {
        try {
          await vscode.commands.executeCommand("workbench.action.chat.open");
          focused = true;
        } catch (error2) {
          vscode.window.showErrorMessage(
            "Could not focus Copilot Chat. The command may have changed in your VS Code version."
          );
          console.error("Failed to focus chat:", error, error2);
        }
      }

      // If we successfully focused the chat, scroll to bottom
      if (focused) {
        await new Promise((resolve) => setTimeout(resolve, 200));

        // Move focus from input to chat list using Ctrl+Up equivalent
        try {
          await vscode.commands.executeCommand("list.focusUp");
          await new Promise((resolve) => setTimeout(resolve, 100));

          // Jump to last message
          await vscode.commands.executeCommand("list.focusLast");
          await new Promise((resolve) => setTimeout(resolve, 50));

          // Scroll down within the last message to see the bottom
          for (let i = 0; i < 100; i++) {
            await vscode.commands.executeCommand("list.scrollDown");
          }
        } catch (error) {
          console.log("Could not scroll to bottom:", error);
        }
      }
    }
  );
  context.subscriptions.push(disposable);
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
};
