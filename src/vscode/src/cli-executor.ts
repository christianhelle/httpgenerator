import * as vscode from 'vscode';

let terminal: vscode.Terminal | undefined;

export function getOrCreateTerminal(name = 'HTTP File Generator'): vscode.Terminal {
    if (terminal && terminal.exitStatus === undefined) {
        return terminal;
    }

    terminal = vscode.window.createTerminal(name);
    return terminal;
}

export function executeInTerminal(executablePath: string, args: string[], terminalName = 'HTTP File Generator'): void {
    terminal = vscode.window.createTerminal({
        name: terminalName,
        shellPath: executablePath,
        shellArgs: args
    });
    terminal.show();
}
