import * as vscode from 'vscode';
import { Range, DecorationRenderOptions, TextEditorDecorationType } from 'vscode';
import { Ruzen } from './ruzen';

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	console.log('Congratulations, your extension "ruzen" is now active!');

	const ruzen = new Ruzen();

	let evalSingleCommand = vscode.commands.registerCommand('ruzen.lineEval', () => {
		const activeEditor = vscode.window.activeTextEditor;
		if (activeEditor) {
			let currentLine = activeEditor.selection.active.line;
			const line = activeEditor.document.lineAt(currentLine);
			let range = new Range(line.lineNumber, 0, line.lineNumber, line.text.length);

			ruzen.eval(line.text);

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
