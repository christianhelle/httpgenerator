import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import * as vscode from 'vscode';

let terminal: vscode.Terminal | undefined;

function quotePosix(value: string): string {
    return `'${value.replace(/'/g, `'\\''`)}'`;
}

function quoteWindowsCommand(value: string): string {
    return `"${value.replace(/\\/g, '\\\\').replace(/"/g, '\\"')}"`;
}

function quotePowerShellLiteral(value: string): string {
    return `'${value.replace(/'/g, "''")}'`;
}

function createCommandScript(executablePath: string, args: string[]): string {
    const tempDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'httpgenerator-vscode-'));

    if (process.platform === 'win32') {
        const scriptPath = path.join(tempDirectory, 'run-httpgenerator.ps1');
        const powerShellArgs = args.map(quotePowerShellLiteral).join(', ');
        fs.writeFileSync(
            scriptPath,
            `try { & ${quotePowerShellLiteral(executablePath)} @(${powerShellArgs}) } finally { Remove-Item -LiteralPath $PSCommandPath -Force -ErrorAction SilentlyContinue; Remove-Item -LiteralPath ${quotePowerShellLiteral(tempDirectory)} -Force -Recurse -ErrorAction SilentlyContinue }\n`,
            'utf8'
        );
        return `powershell.exe -NoProfile -ExecutionPolicy Bypass -File ${quoteWindowsCommand(scriptPath)}`;
    }

    const scriptPath = path.join(tempDirectory, 'run-httpgenerator.sh');
    fs.writeFileSync(
        scriptPath,
        `#!/bin/sh\n${quotePosix(executablePath)} ${args.map(quotePosix).join(' ')}\nstatus=$?\nrm -rf ${quotePosix(tempDirectory)}\nexit $status\n`,
        { encoding: 'utf8', mode: 0o700 }
    );
    return `sh ${quotePosix(scriptPath)}`;
}

export function getOrCreateTerminal(name = 'HTTP File Generator'): vscode.Terminal {
    if (terminal && terminal.exitStatus === undefined) {
        return terminal;
    }

    terminal = vscode.window.createTerminal(name);
    return terminal;
}

export function executeInTerminal(executablePath: string, args: string[], terminalName = 'HTTP File Generator'): void {
    const activeTerminal = getOrCreateTerminal(terminalName);
    activeTerminal.show();
    activeTerminal.sendText(createCommandScript(executablePath, args));
}
