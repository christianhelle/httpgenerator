import * as vscode from 'vscode';
import * as path from 'path';
import { executeInTerminal } from './cli-executor';
import { resolveCLIPath, resetCLI } from './cli-manager';
import { showProgress } from './progress';

let currentContext: vscode.ExtensionContext;
let cliPath: string | undefined;
let cliStatusBarItem: vscode.StatusBarItem | undefined;
let cliResolutionPromise: Promise<string | undefined> | undefined;

async function resolveAndEnsureCLI(context: vscode.ExtensionContext): Promise<string | undefined> {
    if (cliResolutionPromise) {
        return cliResolutionPromise;
    }

    cliResolutionPromise = showProgress('HTTP File Generator CLI', progress =>
        resolveCLIPath(context, message => progress.report({ message }))
    )
        .then(resolvedPath => {
            cliPath = resolvedPath;
            if (resolvedPath) {
                showCLIStatusBar(resolvedPath);
            }

            return resolvedPath;
        })
        .catch(async error => {
            const retry = 'Retry';
            const configure = 'Configure executablePath';
            const choice = await vscode.window.showErrorMessage(
                `Unable to locate or download the httpgenerator CLI. ${error instanceof Error ? error.message : String(error)}`,
                retry,
                configure
            );

            if (choice === retry) {
                cliResolutionPromise = undefined;
                return resolveAndEnsureCLI(context);
            }

            if (choice === configure) {
                await vscode.commands.executeCommand(
                    'workbench.action.openSettings',
                    'http-file-generator.executablePath'
                );
            }

            return undefined;
        })
        .finally(() => {
            cliResolutionPromise = undefined;
        });

    return cliResolutionPromise;
}

function showCLIStatusBar(resolvedPath: string): void {
    if (!cliStatusBarItem) {
        cliStatusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
        cliStatusBarItem.command = 'http-file-generator.showCLIPath';
        currentContext.subscriptions.push(cliStatusBarItem);
    }

    cliStatusBarItem.text = '$(check) HTTP Generator CLI';
    cliStatusBarItem.tooltip = `HTTP File Generator CLI: ${resolvedPath}`;
    cliStatusBarItem.show();
}

async function showCLIPath(context: vscode.ExtensionContext): Promise<void> {
    const resolvedPath = cliPath ?? await resolveAndEnsureCLI(context);
    if (resolvedPath) {
        vscode.window.showInformationMessage(`HTTP File Generator CLI: ${resolvedPath}`);
    } else {
        vscode.window.showErrorMessage('HTTP File Generator CLI is not available.');
    }
}

async function resetCachedCLI(context: vscode.ExtensionContext): Promise<void> {
    await resetCLI(context);
    cliPath = undefined;
    cliStatusBarItem?.hide();
    vscode.window.showInformationMessage('HTTP File Generator cached CLI has been reset. It will be downloaded again when needed.');
}

async function executeHttpGenerator(filePath: string, outputType: string): Promise<void> {
    const resolvedPath = await resolveAndEnsureCLI(currentContext);
    if (!resolvedPath) {
        vscode.window.showErrorMessage(
            'Unable to locate or download the httpgenerator CLI. Try "HTTP File Generator: Reset CLI" or set "http-file-generator.executablePath".'
        );
        return;
    }

    const inputFileDir = path.dirname(filePath);
    const defaultOutputFolder = path.join(inputFileDir, 'HttpFiles');

    const outputFolder = await vscode.window.showInputBox({
        prompt: 'Select output folder',
        value: defaultOutputFolder,
        valueSelection: undefined
    });

    if (!outputFolder) {
        return;
    }

    executeInTerminal(resolvedPath, [filePath, '--output', outputFolder, '--output-type', outputType]);
}

export function activate(context: vscode.ExtensionContext) {
    currentContext = context;
    console.log('HTTP File Generator extension is now active!');

    resolveAndEnsureCLI(context).then(() => undefined, () => undefined);

    async function promptForOpenApiFile(): Promise<vscode.Uri | undefined> {
        const openApiFiles = await vscode.workspace.findFiles('**/*.{json,yaml,yml}');

        if (openApiFiles.length === 0) {
            vscode.window.showErrorMessage('No OpenAPI specification files (.json, .yaml, or .yml) found in the workspace.');
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

            await executeHttpGenerator(fileUri.fsPath, 'OneFile');
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

            await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile');
        }
    );

    const generateSingleHttpFileMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFileMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneFile');
            }
        }
    );

    const generateMultipleHttpFilesMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFilesMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile');
            }
        }
    );

    const resetCLICommand = vscode.commands.registerCommand(
        'http-file-generator.resetCLI',
        async () => resetCachedCLI(context)
    );

    const showCLIPathCommand = vscode.commands.registerCommand(
        'http-file-generator.showCLIPath',
        async () => showCLIPath(context)
    );

    context.subscriptions.push(
        generateSingleHttpFileCommand,
        generateMultipleHttpFilesCommand,
        generateSingleHttpFileMenuCommand,
        generateMultipleHttpFilesMenuCommand,
        resetCLICommand,
        showCLIPathCommand
    );
}

export function deactivate() {}
