const vscode = require("vscode");

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
  console.log("Chat Focus extension is now active");

  let disposable = vscode.commands.registerCommand(
    "chat-focus.focusChat",
    async function () {
      try {
        // Try to focus the Copilot Chat view
        // The command ID for focusing Copilot Chat may vary by version
        await vscode.commands.executeCommand(
          "workbench.panel.chat.view.copilot.focus"
        );

        // Scroll to the bottom of the chat
        await vscode.commands.executeCommand(
          "workbench.action.chat.scrollToBottom"
        );
      } catch (error) {
        // Try alternative command IDs
        try {
          await vscode.commands.executeCommand("workbench.action.chat.open");

          // Try to scroll to bottom after opening
          await vscode.commands.executeCommand(
            "workbench.action.chat.scrollToBottom"
          );
        } catch (error2) {
          try {
            await vscode.commands.executeCommand("github.copilot.chat.focus");

            // Try to scroll to bottom
            await vscode.commands.executeCommand(
              "workbench.action.chat.scrollToBottom"
            );
          } catch (error3) {
            vscode.window.showErrorMessage(
              "Could not focus Copilot Chat. The command may have changed in your VS Code version."
            );
            console.error("Failed to focus chat:", error, error2, error3);
          }
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
