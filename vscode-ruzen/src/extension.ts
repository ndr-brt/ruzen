import * as vscode from 'vscode';
import { Ruzen } from './ruzen';
import { Editor } from './editor';

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	console.log('Ruzen extension is active');

	const ruzen = new Ruzen();
	const editor = new Editor(vscode.window.activeTextEditor!);

	let evalLine = vscode.commands.registerCommand('ruzen.lineEval', () => {
		let expression = editor.currentLineExpression();
		ruzen.eval(expression.code);
		editor.flash(expression.range);
	});

	let evalBlock = vscode.commands.registerCommand('ruzen.blockEval', () => {
		let expression = editor.currentBlockExpression();
		ruzen.eval(expression.code);
		editor.flash(expression.range);
	});
	
	context.subscriptions.push(evalLine, evalBlock);
}

export function deactivate() {}
