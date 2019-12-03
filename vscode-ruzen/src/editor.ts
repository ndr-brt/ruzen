import { TextEditor, Range, Position } from 'vscode';
import vscode = require('vscode');

export class Expression {
    public readonly code: string;
    public readonly range: Range;

    constructor(code: string, range: Range) {
        this.code = code;
        this.range = range;
    }
}

export class Editor {
    editor: TextEditor;
    createTextEditorDecorationType = vscode.window.createTextEditorDecorationType;

    constructor(editor: TextEditor) {
        this.editor = editor;
    }

    public currentLineExpression(): Expression {
        let currentLine = this.editor.selection.active.line;
        let line = this.editor.document.lineAt(currentLine);
        let range = new Range(currentLine, 0, currentLine, line.text.length);
        return new Expression(line.text, range);
    }

    public currentBlockExpression(): Expression {
        let currentLine = this.editor.selection.active.line;

        let startLineIndex = this.blockStartLine(currentLine);
        let endLineIndex = this.blockEndLine(currentLine);
        let text = "";
        for (let i = startLineIndex; i <= endLineIndex; i++) {
            text += this.editor.document.lineAt(i).text + "\n";
        }
        
        let lastLine = this.editor.document.lineAt(endLineIndex);
        let range = new Range(startLineIndex, 0, endLineIndex, lastLine.text.length);

        return new Expression(text, range);
    }

    public flash(range: Range) {
        const flashDecorationType = this.createTextEditorDecorationType({
            backgroundColor: 'rgba(100,250,100,0.3)'
        });

        this.editor.setDecorations(flashDecorationType, [range]);
        
        setTimeout(function () {
            flashDecorationType.dispose();
        }, 250);
    }

    private blockStartLine(currentLine: number): number {
        for (let i = currentLine - 1; i > 0; i--) {
            let line = this.editor.document.lineAt(i);
            if (!line.text.trim()) {
                return i + 1;
            }
        }
        return 0;
    }
    
    private blockEndLine(currentLine: number): number {
        for (let i = currentLine - 1; i < this.editor.document.lineCount; i++) {
            let line = this.editor.document.lineAt(i);
            if (!line.text.trim()) {
                return i - 1;
            }
        }
        return this.editor.document.lineCount - 1;
    }

}