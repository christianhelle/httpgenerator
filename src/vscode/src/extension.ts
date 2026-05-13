import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

type OutputType = 'OneFile' | 'OneRequestPerFile';

interface ResolvedExecutable {
    path: string;
    source: string;
}

function quoteForShell(value: string): string {
    return `"${value.replace(/\\/g, '\\\\').replace(/"/g, '\\"')}"`;
}

function isExecutable(candidatePath: string): boolean {
    if (!fs.existsSync(candidatePath)) {
        return false;
    }

    const stats = fs.statSync(candidatePath);
    if (!stats.isFile()) {
        return false;
    }

    if (process.platform === 'win32') {
        return true;
    }

    try {
        fs.accessSync(candidatePath, fs.constants.X_OK);
        return true;
    } catch {
        return false;
    }
}

function getConfiguredExecutablePath(): string | undefined {
    const configured = vscode.workspace.getConfiguration('http-file-generator').get<string>('executablePath');
    if (!configured) {
        return undefined;
    }

    const trimmed = configured.trim();
    return trimmed.length > 0 ? trimmed : undefined;
}

function getBundledBinaryRelativePath(): string | undefined {
    const exeName = process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';
    const platformKey = `${process.platform}-${process.arch}`;

    switch (platformKey) {
        case 'win32-x64':
            return path.join('bin', 'win32-x64', exeName);
        case 'linux-x64':
            return path.join('bin', 'linux-x64', exeName);
        case 'darwin-x64':
            return path.join('bin', 'darwin-x64', exeName);
        case 'darwin-arm64':
            return path.join('bin', 'darwin-arm64', exeName);
        default:
            return undefined;
    }
}

function resolveBundledExecutable(context: vscode.ExtensionContext): string | undefined {
    const relativePath = getBundledBinaryRelativePath();
    if (!relativePath) {
        return undefined;
    }

    const bundledPath = path.join(context.extensionPath, relativePath);
    if (!isExecutable(bundledPath)) {
        return undefined;
    }

    return bundledPath;
}

function getDevelopmentRoots(context: vscode.ExtensionContext): string[] {
    const roots = new Set<string>();

    for (const folder of vscode.workspace.workspaceFolders ?? []) {
        roots.add(folder.uri.fsPath);
    }

    roots.add(path.resolve(context.extensionPath, '..', '..'));
    return Array.from(roots);
}

function resolveDevelopmentExecutable(context: vscode.ExtensionContext): string | undefined {
    const exeName = process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';
    const buildConfigurations = ['debug', 'release'];

    for (const root of getDevelopmentRoots(context)) {
        for (const configuration of buildConfigurations) {
            const candidate = path.join(root, 'target', configuration, exeName);
            if (isExecutable(candidate)) {
                return candidate;
            }
        }
    }

    return undefined;
}

function resolveExecutableOnPath(commandName: string): string | undefined {
    const pathEntries = (process.env.PATH ?? '').split(path.delimiter).filter(Boolean);
    if (pathEntries.length === 0) {
        return undefined;
    }

    const windowsExtensions = process.platform === 'win32'
        ? (process.env.PATHEXT ?? '.EXE;.CMD;.BAT;.COM').split(';').filter(Boolean)
        : [''];

    for (const pathEntry of pathEntries) {
        for (const extension of windowsExtensions) {
            const candidate = process.platform === 'win32'
                ? path.join(pathEntry, `${commandName}${extension.toLowerCase()}`)
                : path.join(pathEntry, commandName);

            if (isExecutable(candidate)) {
                return candidate;
            }
        }
    }

    return undefined;
}

function resolveHttpGeneratorExecutable(context: vscode.ExtensionContext): ResolvedExecutable {
    const configuredPath = getConfiguredExecutablePath();
    if (configuredPath) {
        const configuredBasePath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? context.extensionPath;
        const resolvedConfiguredPath = path.isAbsolute(configuredPath)
            ? configuredPath
            : path.resolve(configuredBasePath, configuredPath);

        if (!isExecutable(resolvedConfiguredPath)) {
            throw new Error(
                `The configured http-file-generator.executablePath does not point to a runnable file: ${resolvedConfiguredPath}`
            );
        }

        return { path: resolvedConfiguredPath, source: 'configured path' };
    }

    const bundledPath = resolveBundledExecutable(context);
    if (bundledPath) {
        return { path: bundledPath, source: 'bundled binary' };
    }

    const developmentPath = resolveDevelopmentExecutable(context);
    if (developmentPath) {
        return { path: developmentPath, source: 'workspace development build' };
    }

    const pathExecutable = resolveExecutableOnPath('httpgenerator');
    if (pathExecutable) {
        return { path: pathExecutable, source: 'PATH' };
    }

    throw new Error(
        'Could not find the Rust httpgenerator executable. Set http-file-generator.executablePath, install the platform-targeted extension package, build the repo CLI (target/debug or target/release), or add httpgenerator to PATH.'
    );
}

async function executeHttpGenerator(context: vscode.ExtensionContext, filePath: string, outputType: OutputType): Promise<void> {
    let executable: ResolvedExecutable;
    try {
        executable = resolveHttpGeneratorExecutable(context);
    } catch (error) {
        vscode.window.showErrorMessage(error instanceof Error ? error.message : 'Unable to resolve httpgenerator executable.');
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

    try {
        const terminal = vscode.window.createTerminal('HTTP File Generator');
        terminal.show();

        const command = `${quoteForShell(executable.path)} ${quoteForShell(filePath)} --output ${quoteForShell(outputFolder)} --output-type ${outputType}`;
        terminal.sendText(command);
    } catch (error) {
        vscode.window.showErrorMessage(
            `Failed to execute httpgenerator from ${executable.source}: ${error instanceof Error ? error.message : String(error)}`
        );
    }
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
