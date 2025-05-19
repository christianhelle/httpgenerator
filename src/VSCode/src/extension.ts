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
 * Install the httpgenerator .NET tool
 */
async function installHttpGenerator(): Promise<boolean> {
    const installOption = 'Install';
    const cancelOption = 'Cancel';
    
    const choice = await vscode.window.showInformationMessage(
        'The httpgenerator .NET tool is not installed. Would you like to install it?',
        installOption,
        cancelOption
    );
    
    if (choice !== installOption) {
        return false;
    }

    try {
        const terminal = vscode.window.createTerminal('Install HttpGenerator');
        terminal.show();
        terminal.sendText('dotnet tool install --global httpgenerator');
        
        // Wait for the installation to complete
        await new Promise(resolve => setTimeout(resolve, 5000));
        
        // Check again if installed
        return await isHttpGeneratorInstalled();
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to install httpgenerator: ${error}`);
        return false;
    }
}

/**
 * Execute the httpgenerator tool
 */
async function executeHttpGenerator(filePath: string, outputType: string): Promise<void> {
    const isInstalled = await isHttpGeneratorInstalled();
    
    if (!isInstalled) {
        const installed = await installHttpGenerator();
        if (!installed) {
            vscode.window.showErrorMessage('The httpgenerator tool is required but was not installed.');
            return;
        }
    }
    
    // Get the folder where the file is located to use as output directory
    const workspaceFolder = path.dirname(filePath);
    
    // Create a terminal to execute the command
    const terminal = vscode.window.createTerminal('HTTP File Generator');
    terminal.show();
    
    // Execute the httpgenerator command
    const command = `httpgenerator "${filePath}" --output "${workspaceFolder}" --output-type ${outputType}`;
    terminal.sendText(command);
}

export function activate(context: vscode.ExtensionContext) {
    console.log('HTTP File Generator extension is now active!');

    // Register command to generate a single HTTP file
    let generateSingleHttpFileCommand = vscode.commands.registerCommand(
        'http-file-generator.generateSingleHttpFile', 
        async (fileUri: vscode.Uri) => {
            if (!fileUri) {
                vscode.window.showErrorMessage('Please right-click on an OpenAPI specification file (.json, .yaml, or .yml).');
                return;
            }
            
            await executeHttpGenerator(fileUri.fsPath, 'OneFile');
        }
    );

    // Register command to generate multiple HTTP files
    let generateMultipleHttpFilesCommand = vscode.commands.registerCommand(
        'http-file-generator.generateMultipleHttpFiles', 
        async (fileUri: vscode.Uri) => {
            if (!fileUri) {
                vscode.window.showErrorMessage('Please right-click on an OpenAPI specification file (.json, .yaml, or .yml).');
                return;
            }
            
            await executeHttpGenerator(fileUri.fsPath, 'OneRequestPerFile');
        }
    );

    context.subscriptions.push(generateSingleHttpFileCommand);
    context.subscriptions.push(generateMultipleHttpFilesCommand);
}

export function deactivate() {}
