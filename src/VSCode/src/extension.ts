import * as vscode from 'vscode';
import * as path from 'path';
import * as child_process from 'child_process';
import * as fs from 'fs';

/**
 * Check if the .NET tool httpgenerator is installed
 */
async function isHttpGeneratorInstalled(): Promise<boolean> {
    return new Promise<boolean>((resolve) => {
        child_process.exec('httpgenerator --version', (error) => {
            resolve(!error);
        });
    });
}

/**
 * Get the version of the installed httpgenerator tool by checking global .NET tools
 */
async function getInstalledHttpGeneratorVersion(): Promise<string | undefined> {
    return new Promise((resolve) => {
        child_process.exec('dotnet tool list -g', (error, stdout) => {
            if (error) {
                resolve(undefined);
            } else {
                // Look for httpgenerator in the installed tools list
                const toolRegex = /httpgenerator\s+(\d+\.\d+\.\d+)/;
                const match = stdout.match(toolRegex);
                resolve(match ? match[1] : undefined);
            }
        });
    });
}

/**
 * Get the version of the VS Code extension (from package.json)
 */
function getExtensionVersion(): string {
    const packageJsonPath = path.join(__dirname, '..', 'package.json');
    try {
        const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
        return packageJson.version;
    } catch {
        return '0.1.0'; // fallback
    }
}

/**
 * Check if the extension is running in debug mode
 */
function isDebugMode(): boolean {
    return process.env.VSCODE_DEBUG_MODE === 'true' || process.env.NODE_ENV === 'development';
}

/**
 * Install or update the httpgenerator .NET tool to match the extension version
 */
async function installHttpGenerator(forceReinstall = false): Promise<boolean> {
    // Check if installation is actually needed (unless forceReinstall is true)
    if (!forceReinstall) {
        const extensionVersion = getExtensionVersion();
        const debug = isDebugMode();
        const installedVersion = await getInstalledHttpGeneratorVersion();
        
        // Determine if installation is needed
        let needsInstall = false;
        if (!installedVersion) {
            needsInstall = true;
        } else if (debug || extensionVersion === '0.1.0') {
            // In debug or 0.1.0, don't update if already installed
            needsInstall = false;
        } else if (installedVersion !== extensionVersion) {
            needsInstall = true;
        }
        
        if (!needsInstall) {
            // No installation needed
            return true;
        }
    }
    
    // Only show prompt if installation is actually needed
    const installOption = 'Install';
    const cancelOption = 'Cancel';

    const choice = await vscode.window.showInformationMessage(
        'The httpgenerator .NET tool is not installed or the version does not match. Would you like to install/update it?',
        installOption,
        cancelOption
    );

    if (choice !== installOption) {
        return false;
    }

    try {
        const extensionVersion = getExtensionVersion();
        const debug = isDebugMode();
        let installCmd = '';
        if (debug || extensionVersion === '0.1.0') {
            installCmd = 'dotnet tool install --global httpgenerator --prerelease';
        } else {
            installCmd = `dotnet tool install --global httpgenerator --version ${extensionVersion}`;
        }
        // Always try to uninstall first to ensure correct version
        const terminal = vscode.window.createTerminal('Install HttpGenerator');
        terminal.show();
        terminal.sendText('dotnet tool uninstall --global httpgenerator');
        terminal.sendText(installCmd);
        // Wait for the installation to complete
        await new Promise(resolve => setTimeout(resolve, 7000));
        // Check again if installed and version matches
        const installedVersion = await getInstalledHttpGeneratorVersion();
        if (debug || extensionVersion === '0.1.0') {
            return !!installedVersion;
        }
        return installedVersion === extensionVersion;
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to install httpgenerator: ${error}`);
        return false;
    }
}

/**
 * Execute the httpgenerator tool
 */
async function executeHttpGenerator(filePath: string, outputType: string): Promise<void> {
    // Attempt installation if needed, this will only prompt if actually necessary
    const installed = await installHttpGenerator();
    if (!installed) {
        vscode.window.showErrorMessage('The httpgenerator tool is required but was not installed.');
        return;
    }

    // Create default output folder suggestion (HttpFiles subfolder)
    const inputFileDir = path.dirname(filePath);
    const defaultOutputFolder = path.join(inputFileDir, 'HttpFiles');

    // Prompt for output folder with default suggestion
    const outputFolder = await vscode.window.showInputBox({
        prompt: 'Select output folder',
        value: defaultOutputFolder,
        valueSelection: undefined
    });
    
    // User cancelled the operation
    if (!outputFolder) {
        return;
    }    // Create a terminal to execute the command
    const terminal = vscode.window.createTerminal('HTTP File Generator');
    terminal.show();

    // Execute the httpgenerator command
    const command = `httpgenerator "${filePath}" --output "${outputFolder}" --output-type ${outputType}`;
    terminal.sendText(command);
}

export function activate(context: vscode.ExtensionContext) {
    console.log('HTTP File Generator extension is now active!');

    // Helper function to prompt for file selection
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

    // Register command to generate a single HTTP file
    let generateSingleHttpFileCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFile', 
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                // No file provided, prompt user to select one
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return; // User cancelled or no files found
                }
            }
            
            await executeHttpGenerator(fileUri.fsPath, 'OneFile');
        }
    );

    // Register command to generate multiple HTTP files
    let generateMultipleHttpFilesCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFiles', 
        async (fileUri?: vscode.Uri) => {
            if (!fileUri) {
                // No file provided, prompt user to select one
                fileUri = await promptForOpenApiFile();
                if (!fileUri) {
                    return; // User cancelled or no files found
                }
            }
            
            await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile');
        }
    );
    
    // Register context menu commands (these will call the main commands)
    let generateSingleHttpFileMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFileMenu',
        async (fileUri: vscode.Uri) => {
            // Context menu always has a file URI
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneFile');
            }
        }
    );
    
    let generateMultipleHttpFilesMenuCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFilesMenu',
        async (fileUri: vscode.Uri) => {
            // Context menu always has a file URI
            if (fileUri) {
                await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile');
            }
        }
    );

    context.subscriptions.push(generateSingleHttpFileCommand);
    context.subscriptions.push(generateMultipleHttpFilesCommand);
    context.subscriptions.push(generateSingleHttpFileMenuCommand);
    context.subscriptions.push(generateMultipleHttpFilesMenuCommand);
}

export function deactivate() {}
