import * as vscode from 'vscode';
import * as path from 'path';
import * as child_process from 'child_process';
import * as fs from 'fs';

const exeName = process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';

/**
 * Check whether httpgenerator is available on PATH by attempting to run it.
 */
function isOnPath(): boolean {
    try {
        child_process.execFileSync('httpgenerator', ['--version'], { stdio: 'pipe' });
        return true;
    } catch {
        return false;
    }
}

/**
 * Resolve the httpgenerator executable using the documented fallback chain:
 *   1. http-file-generator.executablePath setting (fail fast if path is set but invalid)
 *   2. Bundled binary inside the installed extension (bin/<exeName>)
 *   3. Repo-root development build outputs relative to the extension directory
 *      (extensionPath/../../target/release then .../target/debug)
 *   4. httpgenerator on PATH
 */
async function resolveExecutable(context: vscode.ExtensionContext): Promise<string | null> {
    const config = vscode.workspace.getConfiguration('http-file-generator');
    const configuredPath = config.get<string>('executablePath');

    // 1. Explicit override — fail fast if the path is set but the file does not exist
    if (configuredPath && configuredPath.trim() !== '') {
        if (fs.existsSync(configuredPath)) {
            return configuredPath;
        }
        const choice = await vscode.window.showErrorMessage(
            `http-file-generator.executablePath points to a file that does not exist: "${configuredPath}". ` +
            `Please update the setting or clear it to use automatic resolution.`,
            'Open Settings'
        );
        if (choice === 'Open Settings') {
            vscode.commands.executeCommand('workbench.action.openSettings', 'http-file-generator.executablePath');
        }
        return null;
    }

    // 2. Bundled binary shipped with the extension
    const bundledPath = path.join(context.extensionPath, 'bin', exeName);
    if (fs.existsSync(bundledPath)) {
        return bundledPath;
    }

    // 3. Development build outputs (repo checkout layout: extensionPath = <repo>/src/vscode)
    const repoRoot = path.resolve(context.extensionPath, '..', '..');
    for (const profile of ['release', 'debug']) {
        const devPath = path.join(repoRoot, 'target', profile, exeName);
        if (fs.existsSync(devPath)) {
            return devPath;
        }
    }

    // 4. PATH fallback
    if (isOnPath()) {
        return 'httpgenerator';
    }

    const choice = await vscode.window.showErrorMessage(
        'httpgenerator executable not found. ' +
        'Install it from https://github.com/christianhelle/httpgenerator, ' +
        'or set the http-file-generator.executablePath setting to point to the executable.',
        'Open Settings',
        'Learn More'
    );
    if (choice === 'Open Settings') {
        vscode.commands.executeCommand('workbench.action.openSettings', 'http-file-generator.executablePath');
    } else if (choice === 'Learn More') {
        vscode.env.openExternal(vscode.Uri.parse('https://github.com/christianhelle/httpgenerator'));
    }
    return null;
}

/**
 * Invoke the httpgenerator Rust CLI to generate .http files from an OpenAPI spec.
 */
async function executeHttpGenerator(
    filePath: string,
    outputType: string,
    context: vscode.ExtensionContext
): Promise<void> {
    const executablePath = await resolveExecutable(context);
    if (!executablePath) {
        return;
    }

    const inputFileDir = path.dirname(filePath);
    const defaultOutputFolder = path.join(inputFileDir, 'HttpFiles');

    const outputFolder = await vscode.window.showInputBox({
        prompt: 'Select output folder',
        value: defaultOutputFolder,
    });

    if (!outputFolder) {
        return;
    }

    const terminal = vscode.window.createTerminal('HTTP File Generator');
    terminal.show();

    const command = `"${executablePath}" "${filePath}" --output "${outputFolder}" --output-type ${outputType}`;
    terminal.sendText(command);
}

export function activate(context: vscode.ExtensionContext) {
    console.log('HTTP File Generator extension is now active!');

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

    let generateSingleHttpFileCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFile',
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return;
                }
            }
            await executeHttpGenerator(fileUri.fsPath, 'OneFile', context);
        }
    );

    let generateMultipleHttpFilesCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFiles',
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return;
                }
            }
            await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile', context);
        }
    );

    let generateSingleHttpFileMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFileMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneFile', context);
            }
        }
    );

    let generateMultipleHttpFilesMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFilesMenu',
        async (fileUri: vscode.Uri) => {
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile', context);
            }
        }
    );

    context.subscriptions.push(generateSingleHttpFileCommand);
    context.subscriptions.push(generateMultipleHttpFilesCommand);
    context.subscriptions.push(generateSingleHttpFileMenuCommand);
    context.subscriptions.push(generateMultipleHttpFilesMenuCommand);
}

export function deactivate() {}
