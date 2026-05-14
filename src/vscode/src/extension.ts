import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import * as vscode from 'vscode';

const CONFIGURATION_SECTION = 'http-file-generator';
const EXECUTABLE_SETTING = 'executablePath';
const CLI_COMMAND = 'httpgenerator';
const EXECUTABLE_NAME = process.platform === 'win32' ? `${CLI_COMMAND}.exe` : CLI_COMMAND;

type ResolvedExecutable = {
    path: string;
    source: string;
};

function expandHomeDirectory(value: string): string {
    if (value === '~') {
        return os.homedir();
    }

    if (value.startsWith(`~${path.sep}`) || value.startsWith('~/') || value.startsWith('~\\')) {
        return path.join(os.homedir(), value.slice(2));
    }

    return value;
}

function getConfiguredExecutablePath(context: vscode.ExtensionContext): string | undefined {
    const configuredPath = vscode.workspace
        .getConfiguration(CONFIGURATION_SECTION)
        .get<string>(EXECUTABLE_SETTING)
        ?.trim();

    if (!configuredPath) {
        return undefined;
    }

    const expandedPath = expandHomeDirectory(configuredPath);
    if (path.isAbsolute(expandedPath)) {
        return path.normalize(expandedPath);
    }

    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    const basePath = workspaceRoot ?? context.extensionPath;
    return path.normalize(path.resolve(basePath, expandedPath));
}

function isExecutableFile(filePath: string): boolean {
    try {
        const accessMode = process.platform === 'win32'
            ? fs.constants.F_OK
            : fs.constants.F_OK | fs.constants.X_OK;

        fs.accessSync(filePath, accessMode);
        return fs.statSync(filePath).isFile();
    } catch {
        return false;
    }
}

function getCurrentPlatformTarget(): string | undefined {
    switch (process.platform) {
        case 'win32':
            switch (process.arch) {
                case 'x64':
                    return 'win32-x64';
                case 'arm64':
                    return 'win32-arm64';
                case 'ia32':
                    return 'win32-ia32';
                default:
                    return undefined;
            }
        case 'linux':
            switch (process.arch) {
                case 'x64':
                    return 'linux-x64';
                case 'arm64':
                    return 'linux-arm64';
                case 'arm':
                    return 'linux-armhf';
                default:
                    return undefined;
            }
        case 'darwin':
            switch (process.arch) {
                case 'x64':
                    return 'darwin-x64';
                case 'arm64':
                    return 'darwin-arm64';
                default:
                    return undefined;
            }
        default:
            return undefined;
    }
}

function resolveBundledExecutable(context: vscode.ExtensionContext): ResolvedExecutable | undefined {
    const platformTarget = getCurrentPlatformTarget();
    const candidates = platformTarget
        ? [
            path.join(context.extensionPath, 'bin', platformTarget, EXECUTABLE_NAME),
            path.join(context.extensionPath, 'bin', EXECUTABLE_NAME)
        ]
        : [path.join(context.extensionPath, 'bin', EXECUTABLE_NAME)];

    for (const candidate of candidates) {
        if (isExecutableFile(candidate)) {
            return {
                path: candidate,
                source: 'the bundled extension binary'
            };
        }
    }

    return undefined;
}

function findRepoRoot(startPath: string): string | undefined {
    let currentPath = startPath;

    while (true) {
        const cargoTomlPath = path.join(currentPath, 'Cargo.toml');
        const rustCliPath = path.join(currentPath, 'src', 'rust', 'cli');
        if (fs.existsSync(cargoTomlPath) && fs.existsSync(rustCliPath)) {
            return currentPath;
        }

        const parentPath = path.dirname(currentPath);
        if (parentPath === currentPath) {
            return undefined;
        }

        currentPath = parentPath;
    }
}

function resolveDevelopmentExecutable(context: vscode.ExtensionContext): ResolvedExecutable | undefined {
    const repoRoot = findRepoRoot(context.extensionPath);
    if (!repoRoot) {
        return undefined;
    }

    const candidates = [
        path.join(repoRoot, 'target', 'debug', EXECUTABLE_NAME),
        path.join(repoRoot, 'target', 'release', EXECUTABLE_NAME)
    ];

    for (const candidate of candidates) {
        if (isExecutableFile(candidate)) {
            return {
                path: candidate,
                source: 'the repo-root target output'
            };
        }
    }

    return undefined;
}

function resolvePathExecutable(): ResolvedExecutable | undefined {
    const pathEntries = (process.env.PATH ?? '')
        .split(path.delimiter)
        .map(entry => entry.trim())
        .filter(Boolean);

    const commandCandidates = process.platform === 'win32' && path.extname(CLI_COMMAND) === ''
        ? (process.env.PATHEXT ?? '.COM;.EXE;.BAT;.CMD')
            .split(';')
            .filter(Boolean)
            .map(extension => `${CLI_COMMAND}${extension}`)
        : [CLI_COMMAND];

    for (const entry of pathEntries) {
        for (const commandCandidate of commandCandidates) {
            const candidate = path.join(entry, commandCandidate);
            if (isExecutableFile(candidate)) {
                return {
                    path: candidate,
                    source: 'PATH'
                };
            }
        }
    }

    return undefined;
}

function resolveHttpGeneratorExecutable(context: vscode.ExtensionContext): ResolvedExecutable {
    const configuredExecutablePath = getConfiguredExecutablePath(context);
    if (configuredExecutablePath) {
        if (!isExecutableFile(configuredExecutablePath)) {
            throw new Error(
                `The configured http-file-generator.executablePath points to "${configuredExecutablePath}", but no executable was found there. ` +
                'Update the setting or clear it to use the bundled CLI.'
            );
        }

        return {
            path: configuredExecutablePath,
            source: 'http-file-generator.executablePath'
        };
    }

    const bundledExecutable = resolveBundledExecutable(context);
    if (bundledExecutable) {
        return bundledExecutable;
    }

    const developmentExecutable = resolveDevelopmentExecutable(context);
    if (developmentExecutable) {
        return developmentExecutable;
    }

    const pathExecutable = resolvePathExecutable();
    if (pathExecutable) {
        return pathExecutable;
    }

    throw new Error(
        'Unable to locate the httpgenerator executable. Set http-file-generator.executablePath, reinstall the extension to restore the bundled CLI, ' +
        'build the Rust CLI into repo-root target\\debug or target\\release during development, or add httpgenerator to PATH.'
    );
}

function quoteArgument(value: string): string {
    return `"${value.replace(/"/g, '\\"')}"`;
}

function createHttpGeneratorCommand(executablePath: string, filePath: string, outputFolder: string, outputType: string): string {
    return [
        quoteArgument(executablePath),
        quoteArgument(filePath),
        '--output',
        quoteArgument(outputFolder),
        '--output-type',
        outputType
    ].join(' ');
}

/**
 * Execute the httpgenerator tool
 */
async function executeHttpGenerator(context: vscode.ExtensionContext, filePath: string, outputType: string): Promise<void> {
    let executable: ResolvedExecutable;

    try {
        executable = resolveHttpGeneratorExecutable(context);
    } catch (error) {
        const message = error instanceof Error ? error.message : `Failed to resolve httpgenerator: ${error}`;
        vscode.window.showErrorMessage(message);
        return;
    }

    console.log(`Resolved httpgenerator from ${executable.source}: ${executable.path}`);

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

    const terminal = vscode.window.createTerminal('HTTP File Generator');
    terminal.show();
    terminal.sendText(createHttpGeneratorCommand(executable.path, filePath, outputFolder, outputType));
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
