"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const vscode = require("vscode");
const vscode_1 = require("vscode");
var udp = require('dgram');
// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
function activate(context) {
    console.log('Congratulations, your extension "ruzen" is now active!');
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
    context.subscriptions.push(evalSingleCommand);
}
exports.activate = activate;
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map