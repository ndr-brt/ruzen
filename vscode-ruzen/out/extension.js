"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const vscode = require("vscode");
const vscode_1 = require("vscode");
var udp = require('dgram');
// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
function activate(context) {
    // Use the console to output diagnostic information (console.log) and errors (console.error)
    // This line of code will only be executed once when your extension is activated
    console.log('Congratulations, your extension "ruzen" is now active!');
    // The command has been defined in the package.json file
    // Now provide the implementation of the command with registerCommand
    // The commandId parameter must match the command field in package.json
    let disposable = vscode.commands.registerCommand('extension.helloWorld', () => {
        // The code you place here will be executed every time your command is executed
        // Display a message box to the user
        vscode.window.showInformationMessage('Hello World!');
    });
    var client = udp.createSocket('udp4');
    let evalSingleCommand = vscode.commands.registerCommand('ruzen.eval', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor) {
            let currentLine = activeEditor.selection.active.line;
            const line = activeEditor.document.lineAt(currentLine);
            let range = new vscode_1.Range(line.lineNumber, 0, line.lineNumber, line.text.length);
            client.send(line.text, 38043, 'localhost', function (error) {
                if (error) {
                    console.error(`Error ${error}`);
                    client.close();
                }
                else {
                    console.log('Data sent !!!');
                }
            });
            let createTextEditorDecorationType = vscode.window.createTextEditorDecorationType;
            const flashDecorationType = createTextEditorDecorationType({
                backgroundColor: 'rgba(100,250,100,0.3)'
            });
            activeEditor.setDecorations(flashDecorationType, [range]);
            setTimeout(function () {
                flashDecorationType.dispose();
            }, 250);
        }
    });
    context.subscriptions.push(disposable, evalSingleCommand);
}
exports.activate = activate;
// this method is called when your extension is deactivated
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map