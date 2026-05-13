import * as vscode from 'vscode';

let terminal: vscode.Terminal | undefined;

function quoteArgument(value: string): string {
    if (process.platform === 'win32') {
        return `"${value.replace(/\\/g, '\\\\').replace(/"/g, '\\"')}"`;
    }

    return `'${value.replace(/'/g, `'\\''`)}'`;
}

export function getOrCreateTerminal(name = 'HTTP File Generator'): vscode.Terminal {
    if (terminal && terminal.exitStatus === undefined) {
        return terminal;
    }

    terminal = vscode.window.createTerminal(name);
    return terminal;
}

export function executeInTerminal(executablePath: string, args: string[], terminalName?: string): void {
    const command = [executablePath, ...args].map(quoteArgument).join(' ');
    const activeTerminal = getOrCreateTerminal(terminalName);
    activeTerminal.show();
    activeTerminal.sendText(command);
}
