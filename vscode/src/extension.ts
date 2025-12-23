import * as vscode from "vscode";


export function activate(context: vscode.ExtensionContext) {

    vscode.commands.registerCommand("ohl.runFile", async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        if (editor.document.isDirty) {
            await editor.document.save();
        }

        let terminal = vscode.window.activeTerminal;
        if (!terminal) {
            terminal = vscode.window.createTerminal();
        }

        terminal.show();
        terminal.sendText(`oo run "${editor.document.fileName}"`);
    });

}