import * as child_process from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as vscode from 'vscode';

const EXECUTABLE_SETTING = 'http-file-generator.executablePath';

function executableFileName(): string {
    return process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';
}

function getConfiguredExecutablePath(): string | undefined {
    const configuredPath = vscode.workspace
        .getConfiguration('http-file-generator')
        .get<string>('executablePath')
        ?.trim();

    return configuredPath ? configuredPath : undefined;
}

function firstExistingPath(candidates: string[]): string | undefined {
    return candidates.find(candidate => fs.existsSync(candidate));
}

function getBundledExecutableCandidates(context: vscode.ExtensionContext): string[] {
    return [path.join(context.extensionPath, 'bin', executableFileName())];
}

function getWorkspaceExecutableCandidates(context: vscode.ExtensionContext): string[] {
    const executable = executableFileName();
    const candidates = new Set<string>();

    if (context.extensionMode === vscode.ExtensionMode.Development) {
        const repoRoot = path.resolve(context.extensionPath, '..', '..');
        candidates.add(path.join(repoRoot, 'target', 'debug', executable));
        candidates.add(path.join(repoRoot, 'target', 'release', executable));
    }

    for (const workspaceFolder of vscode.workspace.workspaceFolders ?? []) {
        candidates.add(path.join(workspaceFolder.uri.fsPath, 'target', 'debug', executable));
        candidates.add(path.join(workspaceFolder.uri.fsPath, 'target', 'release', executable));
    }

    return Array.from(candidates);
}

async function canExecuteCommand(command: string): Promise<boolean> {
    return new Promise(resolve => {
        child_process.execFile(command, ['--version'], { timeout: 5000 }, error => {
            resolve(!error);
        });
    });
}

async function resolveHttpGeneratorExecutable(context: vscode.ExtensionContext): Promise<string | undefined> {
    const configuredPath = getConfiguredExecutablePath();
    if (configuredPath) {
        if (fs.existsSync(configuredPath)) {
            return configuredPath;
        }

        const openSettings = 'Open Settings';
        const choice = await vscode.window.showErrorMessage(
            `The configured HTTP File Generator executable was not found: ${configuredPath}`,
            openSettings
        );

        if (choice === openSettings) {
            await vscode.commands.executeCommand('workbench.action.openSettings', EXECUTABLE_SETTING);
        }

        return undefined;
    }

    const bundledExecutable = firstExistingPath(getBundledExecutableCandidates(context));
    if (bundledExecutable) {
        return bundledExecutable;
    }

    const workspaceExecutable = firstExistingPath(getWorkspaceExecutableCandidates(context));
    if (workspaceExecutable) {
        return workspaceExecutable;
    }

    if (await canExecuteCommand('httpgenerator')) {
        return 'httpgenerator';
    }

    return undefined;
}

function quoteArgument(value: string): string {
    return `"${value.replace(/"/g, '\\"')}"`;
}

function quoteCommand(command: string): string {
    return /^[a-z0-9._-]+$/i.test(command) ? command : quoteArgument(command);
}

function buildHttpGeneratorCommand(
    executablePath: string,
    filePath: string,
    outputFolder: string,
    outputType: string
): string {
    return `${quoteCommand(executablePath)} ${quoteArgument(filePath)} --output ${quoteArgument(outputFolder)} --output-type ${outputType}`;
}

async function promptForOutputFolder(filePath: string): Promise<string | undefined> {
    const inputFileDir = path.dirname(filePath);
    const defaultOutputFolder = path.join(inputFileDir, 'HttpFiles');

    return vscode.window.showInputBox({
        prompt: 'Select output folder',
        value: defaultOutputFolder,
        valueSelection: undefined
    });
}

async function executeHttpGenerator(
    context: vscode.ExtensionContext,
    filePath: string,
    outputType: string
): Promise<void> {
    const executablePath = await resolveHttpGeneratorExecutable(context);
    if (!executablePath) {
        const openSettings = 'Open Settings';
        const choice = await vscode.window.showErrorMessage(
            'Unable to locate the Rust httpgenerator executable. Configure http-file-generator.executablePath, build the workspace binary, or install httpgenerator on PATH.',
            openSettings
        );

        if (choice === openSettings) {
            await vscode.commands.executeCommand('workbench.action.openSettings', EXECUTABLE_SETTING);
        }

        return;
    }

    const outputFolder = await promptForOutputFolder(filePath);
    if (!outputFolder) {
        return;
    }

    const terminal = vscode.window.createTerminal('HTTP File Generator');
    terminal.show();
    terminal.sendText(
        buildHttpGeneratorCommand(executablePath, filePath, outputFolder, outputType)
    );
}

export function activate(context: vscode.ExtensionContext) {
    console.log('HTTP File Generator extension is now active!');

    async function promptForOpenApiFile(): Promise<vscode.Uri | undefined> {
        const openApiFiles = await vscode.workspace.findFiles('**/*.{json,yaml,yml}');

        if (openApiFiles.length === 0) {
            vscode.window.showErrorMessage(
                'No OpenAPI specification files (.json, .yaml, or .yml) found in the workspace.'
            );
            return undefined;
        }

        const fileItems = openApiFiles.map(file => ({
            label: path.basename(file.fsPath),
            description: vscode.workspace.asRelativePath(file),
            uri: file
        }));

        const selectedFile = await vscode.window.showQuickPick(fileItems, {
            placeHolder: 'Select an OpenAPI specification file'
        });

        return selectedFile?.uri;
    }

    const generateSingleHttpFileCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFile',
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return;
                }
            }

            await executeHttpGenerator(context, fileUri.fsPath, 'OneFile');
        }
    );

    const generateMultipleHttpFilesCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFiles',
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return;
                }
            }

            await executeHttpGenerator(context, fileUri.fsPath, 'OneRequestPerFile');
        }
    );

    const generateSingleHttpFileMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFileMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(context, fileUri.fsPath, 'OneFile');
            }
        }
    );

    const generateMultipleHttpFilesMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFilesMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(context, fileUri.fsPath, 'OneRequestPerFile');
            }
        }
    );

    context.subscriptions.push(generateSingleHttpFileCommand);
    context.subscriptions.push(generateMultipleHttpFilesCommand);
    context.subscriptions.push(generateSingleHttpFileMenuCommand);
    context.subscriptions.push(generateMultipleHttpFilesMenuCommand);
}

export function deactivate() {}
