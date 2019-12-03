import * as vscode from 'vscode';
import { Range, DecorationRenderOptions, TextEditorDecorationType } from 'vscode';
var udp = require('dgram');

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	console.log('Congratulations, your extension "ruzen" is now active!');

	var client = udp.createSocket('udp4');

	let evalSingleCommand = vscode.commands.registerCommand('ruzen.eval', () => {
		const activeEditor = vscode.window.activeTextEditor;
		if (activeEditor) {
			let currentLine = activeEditor.selection.active.line;
			const line = activeEditor.document.lineAt(currentLine);
			let range = new Range(line.lineNumber, 0, line.lineNumber, line.text.length);

			client.send(line.text, 38043, 'localhost', function(error: any) {
				if (error) {
					  console.error(`Error ${error}`);
					client.close();
				} else {
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

export function deactivate() {}
