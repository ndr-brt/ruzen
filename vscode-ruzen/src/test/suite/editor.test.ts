import * as assert from 'assert';

import * as TypeMoq from 'typemoq';
import { Range, TextEditor, TextDocument, Selection, Position, TextLine } from 'vscode';
import { Editor } from '../../editor';

suite('Editor test suite', () => {

	test('Single line evaluation', () => {
        let document = createMockDocument(["cursor is here"]);
        let selection = new Selection(new Position(0, 3), new Position(0, 3));
        let editor = new Editor(createMockEditor(document, selection));

        let expression = editor.currentLineExpression();

		assert.equal(expression.code, "cursor is here");
		assert.deepEqual(expression.range, new Range(0, 0, 0, 14));
    });
    
    test('Multi line evaluation from first line', () => {
        let document = createMockDocument(["first line", "second line"]);
        let selection = new Selection(new Position(0, 3), new Position(0, 3));
        let editor = new Editor(createMockEditor(document, selection));

        let expression = editor.currentBlockExpression();

		assert.equal(expression.code, "first line\nsecond line");
		assert.deepEqual(expression.range, new Range(0, 0, 1, 11));
    })
});

class TestTextLine implements TextLine {
    lineNumber: number;
    text: string;
    range: Range;
    rangeIncludingLineBreak: Range;
    firstNonWhitespaceCharacterIndex: number;
    isEmptyOrWhitespace: boolean;

    constructor(lineNumber: number, text: string) {
        this.lineNumber = lineNumber;
        this.text = text;
        this.range = new Range(new Position(0, 0), new Position(0, text.length));
        this.rangeIncludingLineBreak = new Range(new Position(0, 0), new Position(0, text.length + 2));
        this.firstNonWhitespaceCharacterIndex = text.search('[^\s]');
        this.isEmptyOrWhitespace = text.trim().length === 0;
    }
}

function createMockEditor(document: TextDocument, selection: Selection): TextEditor {
    let mockEditor = TypeMoq.Mock.ofType<TextEditor>();
    mockEditor.setup(e => e.document).returns(() => document);
    mockEditor.setup(e => e.selection).returns(() => selection);
    return mockEditor.object;
}

export function createMockDocument(lines: string[]): TextDocument {
    let mockDocument = TypeMoq.Mock.ofType<TextDocument>();
    lines.forEach((line, index) => {
        mockDocument
            .setup(d => d.lineAt(
                TypeMoq.It.is((p: Position) => p.line === index && p.character <= line.length)))
            .returns(() => new TestTextLine(index, line));
        mockDocument.setup(d => d.lineAt(index))
            .returns(() => new TestTextLine(index, line));
    });
    mockDocument.setup(d => d.lineCount).returns(() => lines.length);

    mockDocument
        .setup(d => d.getText(TypeMoq.It.isAny()))
        .returns((r: Range) => {
            let result = "";
            for (let line = r.start.line; line <= r.end.line; line++) {
                if (line === r.start.line) {
                    result += mockDocument.object.lineAt(line).text.substring(r.start.character);
                    result += "\r\n";
                } else if (line < r.end.line) {
                    result += mockDocument.object.lineAt(line);
                    result += "\r\n";
                } else {
                    result += mockDocument.object.lineAt(line).text.substring(0, r.end.character);
                }
            }
            return result;
        });

    return mockDocument.object;
}
