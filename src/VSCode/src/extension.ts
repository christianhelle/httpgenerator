import * as vscode from 'vscode';
import { execSync, exec } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

export function activate(context: vscode.ExtensionContext) {
    console.log('HTTP File Generator for VS Code extension is now active');

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('http-file-generator.generateSingleFile', generateSingleFile),
        vscode.commands.registerCommand('http-file-generator.generateMultipleFiles', generateMultipleFiles)
    );
}

export function deactivate() {}

async function checkToolInstalled(): Promise<boolean> {
    try {
        execSync('dotnet tool list -g', { stdio: 'pipe' }).toString().includes('httpgenerator');
        return true;
    } catch (error) {
        return false;
    }
}

async function installTool(): Promise<boolean> {
    try {
        const response = await vscode.window.showInformationMessage(
            'The httpgenerator .NET tool is not installed. Do you want to install it?',
            'Yes', 'No'
        );

        if (response === 'Yes') {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Installing httpgenerator...',
                cancellable: false
            }, async (progress) => {
                return new Promise<void>((resolve, reject) => {
                    exec('dotnet tool install --global httpgenerator', (error, stdout, stderr) => {
                        if (error) {
                            vscode.window.showErrorMessage(`Failed to install httpgenerator: ${error.message}`);
                            reject(error);
                            return;
                        }
                        vscode.window.showInformationMessage('httpgenerator tool installed successfully!');
                        resolve();
                    });
                });
            });
            return true;
        }
        return false;
    } catch (error) {
        vscode.window.showErrorMessage(`Error installing httpgenerator: ${error}`);
        return false;
    }
}

async function generateHttpFile(outputType: string) {
    const fileUri = vscode.window.activeTextEditor?.document.uri || 
                   (vscode.window.activeTextEditor ? undefined : vscode.workspace.workspaceFolders?.[0]?.uri);
    
    if (!fileUri) {
        const openApiFiles = await findOpenApiFiles();
        if (openApiFiles.length === 0) {
            vscode.window.showErrorMessage('No OpenAPI specification files found in workspace');
            return;
        }
        
        const selectedFile = await vscode.window.showQuickPick(
            openApiFiles.map(file => file.fsPath), 
            { placeHolder: 'Select an OpenAPI specification file' }
        );
        
        if (!selectedFile) {
            return;
        }
        
        await runGenerator(selectedFile, outputType);
    } else {
        const filePath = fileUri.fsPath;
        
        // Check if right-clicked on a file in Explorer
        if (fs.statSync(filePath).isFile()) {
            const ext = path.extname(filePath).toLowerCase();
            if (['.json', '.yaml', '.yml'].includes(ext)) {
                await runGenerator(filePath, outputType);
            } else {
                vscode.window.showErrorMessage('Selected file is not a supported OpenAPI specification (JSON or YAML)');
            }
        } else {
            vscode.window.showErrorMessage('Please select an OpenAPI specification file (JSON or YAML)');
        }
    }
}

async function findOpenApiFiles(): Promise<vscode.Uri[]> {
    const files = await vscode.workspace.findFiles('**/*.{json,yaml,yml}', '**/node_modules/**');
    return files;
}

async function runGenerator(filePath: string, outputType: string) {
    // Check if tool is installed
    if (!(await checkToolInstalled())) {
        const installed = await installTool();
        if (!installed) {
            vscode.window.showErrorMessage('The httpgenerator tool is required but not installed');
            return;
        }
    }

    // Get output directory
    let defaultOutput = path.dirname(filePath);
    defaultOutput = path.join(defaultOutput, 'HttpFiles');
    
    const outputDir = await vscode.window.showInputBox({
        prompt: 'Output directory for HTTP files',
        value: defaultOutput
    });
    
    if (!outputDir) {
        return; // User cancelled
    }
    
    // Ensure directory exists
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    
    try {
        await vscode.window.withProgress({
            location: vscode.ProgressLocation.Notification,
            title: 'Generating HTTP files...',
            cancellable: false
        }, async (progress) => {
            return new Promise<void>((resolve, reject) => {
                const command = `httpgenerator "${filePath}" --output "${outputDir}" --output-type ${outputType}`;
                
                exec(command, (error, stdout, stderr) => {
                    if (error) {
                        vscode.window.showErrorMessage(`Error generating HTTP files: ${error.message}`);
                        reject(error);
                        return;
                    }
                    
                    vscode.window.showInformationMessage(`HTTP files generated successfully in ${outputDir}`);
                    resolve();
                });
            });
        });
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to generate HTTP files: ${error}`);
    }
}

async function generateSingleFile() {
    await generateHttpFile('OneFile');
}

async function generateMultipleFiles() {
    await generateHttpFile('OneRequestPerFile');
}