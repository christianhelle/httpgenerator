import * as vscode from 'vscode';
import { execSync, spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

// Constants for messages and command parameters
const MESSAGES = {
    EXTENSION_ACTIVE: 'HTTP File Generator for VS Code extension is now active',
    TOOL_NOT_INSTALLED: 'The httpgenerator .NET tool is not installed. Do you want to install it?',
    INSTALLING_TOOL: 'Installing httpgenerator...',
    INSTALL_FAILED: 'Failed to install httpgenerator',
    INSTALL_SUCCESS: 'httpgenerator tool installed successfully!',
    INSTALL_ERROR: 'Error installing httpgenerator',
    NO_OPENAPI_FILES: 'No OpenAPI specification files found in workspace',
    FILE_PICKER_PLACEHOLDER: 'Select an OpenAPI specification file',
    UNSUPPORTED_FILE: 'Selected file is not a supported OpenAPI specification (JSON or YAML)',
    SELECT_FILE: 'Please select an OpenAPI specification file (JSON or YAML)',
    TOOL_REQUIRED: 'The httpgenerator tool is required but not installed',
    OUTPUT_PROMPT: 'Output directory for HTTP files',
    GENERATING: 'Generating HTTP files...',
    GENERATE_ERROR: 'Error generating HTTP files',
    GENERATE_FAILED: 'Failed to generate HTTP files',
    GENERATE_SUCCESS: 'HTTP files generated successfully in',
    PATH_OUTSIDE_WORKSPACE: 'Path must be within the workspace boundaries'
};

const COMMANDS = {
    TOOL_LIST: 'dotnet tool list -g',
    TOOL_INSTALL: 'dotnet tool install --global httpgenerator',
    GENERATE_CMD: 'httpgenerator'
};

const FILE_EXTENSIONS = ['.json', '.yaml', '.yml'];
const OUTPUT_TYPES = {
    ONE_FILE: 'OneFile',
    ONE_REQUEST_PER_FILE: 'OneRequestPerFile'
};

export function activate(context: vscode.ExtensionContext): void {
    console.log(MESSAGES.EXTENSION_ACTIVE);

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('http-file-generator.generateSingleFile', generateSingleFile),
        vscode.commands.registerCommand('http-file-generator.generateMultipleFiles', generateMultipleFiles)
    );
}

export function deactivate(): void {
    // Nothing to clean up
}

/**
 * Checks if the httpgenerator tool is installed
 * @returns Promise resolving to true if tool is installed, false otherwise
 */
async function checkToolInstalled(): Promise<boolean> {
    try {
        const output = execSync(COMMANDS.TOOL_LIST, { stdio: 'pipe' }).toString();
        return output.includes('httpgenerator');
    } catch {
        return false;
    }
}

/**
 * Installs the httpgenerator tool
 * @returns Promise resolving to true if installation succeeded, false otherwise
 */
async function installTool(): Promise<boolean> {
    try {
        const response = await vscode.window.showInformationMessage(
            MESSAGES.TOOL_NOT_INSTALLED,
            'Yes', 'No'
        );

        if (response === 'Yes') {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: MESSAGES.INSTALLING_TOOL,
                cancellable: false
            }, async () => {
                return new Promise<void>((resolve, reject) => {
                    // Use spawn instead of exec for better security
                    const args = COMMANDS.TOOL_INSTALL.split(' ');
                    const cmd = args.shift() || 'dotnet';
                    
                    const process = spawn(cmd, args);
                    
                    let errorOutput = '';
                    process.stderr.on('data', (data: Buffer) => {
                        errorOutput += data.toString();
                    });
                    
                    process.on('close', (code: number | null) => {
                        if (code !== 0) {
                            const errorMessage = `${MESSAGES.INSTALL_FAILED}: ${errorOutput || 'Process exited with code ' + code}`;
                            vscode.window.showErrorMessage(errorMessage);
                            reject(new Error(errorMessage));
                            return;
                        }
                        
                        vscode.window.showInformationMessage(MESSAGES.INSTALL_SUCCESS);
                        resolve();
                    });
                    
                    process.on('error', (error: Error) => {
                        const errorMessage = `${MESSAGES.INSTALL_FAILED}: ${error.message}`;
                        vscode.window.showErrorMessage(errorMessage);
                        reject(new Error(errorMessage));
                    });
                });
            });
            return true;
        }
        return false;
    } catch (error) {
        const errorMessage = error instanceof Error 
            ? `${MESSAGES.INSTALL_ERROR}: ${error.message}`
            : `${MESSAGES.INSTALL_ERROR}: ${String(error)}`;
        vscode.window.showErrorMessage(errorMessage);
        return false;
    }
}

/**
 * Finds potential OpenAPI specification files in the workspace
 * @returns Promise resolving to an array of file URIs
 */
async function findOpenApiFiles(): Promise<vscode.Uri[]> {
    const files = await vscode.workspace.findFiles(
        '**/*.{json,yaml,yml}', 
        '**/node_modules/**'
    );
    return files;
}

/**
 * Validates that a path is within the workspace boundaries to prevent path traversal
 * @param inputPath Path to validate
 * @returns true if the path is within the workspace, false otherwise
 */
function isPathWithinWorkspace(inputPath: string): boolean {
    // Get all workspace folders
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders || workspaceFolders.length === 0) {
        return false;
    }
    
    // Normalize the input path to resolve any '..' or '.' segments
    const normalizedPath = path.normalize(inputPath);
    
    // Check if the normalized path is within any workspace folder
    return workspaceFolders.some((folder: vscode.WorkspaceFolder) => {
        const workspacePath = folder.uri.fsPath;
        return normalizedPath.startsWith(workspacePath);
    });
}

/**
 * Handles file selection logic when no file is directly selected
 * @returns Promise resolving to selected file path or undefined if canceled
 */
async function handleFileSelection(): Promise<string | undefined> {
    const openApiFiles = await findOpenApiFiles();
    
    if (openApiFiles.length === 0) {
        vscode.window.showErrorMessage(MESSAGES.NO_OPENAPI_FILES);
        return undefined;
    }
    
    // Filter files to ensure they're within workspace boundaries
    const safeFiles = openApiFiles.filter(file => isPathWithinWorkspace(file.fsPath));
    
    if (safeFiles.length === 0) {
        vscode.window.showErrorMessage(MESSAGES.NO_OPENAPI_FILES);
        return undefined;
    }
    
    const selectedFile = await vscode.window.showQuickPick(
        safeFiles.map(file => file.fsPath), 
        { placeHolder: MESSAGES.FILE_PICKER_PLACEHOLDER }
    );
    
    if (!selectedFile) {
        return undefined;
    }
    
    return selectedFile;
}

/**
 * Validates if a file is a supported OpenAPI file type
 * @param filePath Path to the file
 * @returns true if the file has a supported extension, false otherwise
 */
function isValidOpenApiFile(filePath: string): boolean {
    try {
        if (!fs.statSync(filePath).isFile()) {
            return false;
        }
        
        const ext = path.extname(filePath).toLowerCase();
        return FILE_EXTENSIONS.includes(ext);
    } catch {
        return false;
    }
}

/**
 * Generates HTTP file(s) based on selected OpenAPI specification
 * @param outputType Type of output to generate (OneFile or OneRequestPerFile)
 */
async function generateHttpFile(outputType: string): Promise<void> {
    const fileUri = vscode.window.activeTextEditor?.document.uri || 
                   (vscode.window.activeTextEditor ? undefined : vscode.workspace.workspaceFolders?.[0]?.uri);
    
    let filePath: string | undefined;
    
    if (!fileUri) {
        filePath = await handleFileSelection();
        if (!filePath) {
            return;
        }
    } else {
        filePath = fileUri.fsPath;
        
        // Validate path is within workspace boundaries
        if (!isPathWithinWorkspace(filePath)) {
            vscode.window.showErrorMessage(MESSAGES.PATH_OUTSIDE_WORKSPACE);
            return;
        }
        
        if (!isValidOpenApiFile(filePath)) {
            vscode.window.showErrorMessage(MESSAGES.UNSUPPORTED_FILE);
            return;
        }
    }
    
    await runGenerator(filePath, outputType);
}

/**
 * Runs the httpgenerator tool with the provided parameters
 * @param filePath Path to the OpenAPI specification file
 * @param outputType Type of output to generate
 */
async function runGenerator(filePath: string, outputType: string): Promise<void> {
    // Validate paths to prevent path traversal
    if (!isPathWithinWorkspace(filePath)) {
        vscode.window.showErrorMessage(MESSAGES.PATH_OUTSIDE_WORKSPACE);
        return;
    }

    // Check if tool is installed
    if (!(await checkToolInstalled())) {
        const installed = await installTool();
        if (!installed) {
            vscode.window.showErrorMessage(MESSAGES.TOOL_REQUIRED);
            return;
        }
    }

    // Get output directory
    let defaultOutput = path.dirname(filePath);
    defaultOutput = path.join(defaultOutput, 'HttpFiles');
    
    const outputDir = await vscode.window.showInputBox({
        prompt: MESSAGES.OUTPUT_PROMPT,
        value: defaultOutput
    });
    
    if (!outputDir) {
        return; // User cancelled
    }
    
    // Validate output directory path to prevent path traversal
    if (!isPathWithinWorkspace(outputDir)) {
        vscode.window.showErrorMessage(MESSAGES.PATH_OUTSIDE_WORKSPACE);
        return;
    }
    
    // Ensure directory exists
    try {
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir, { recursive: true });
        }
    } catch (error) {
        const errorMessage = error instanceof Error 
            ? `${MESSAGES.GENERATE_ERROR}: ${error.message}`
            : `${MESSAGES.GENERATE_ERROR}: ${String(error)}`;
        vscode.window.showErrorMessage(errorMessage);
        return;
    }
    
    try {
        await vscode.window.withProgress({
            location: vscode.ProgressLocation.Notification,
            title: MESSAGES.GENERATING,
            cancellable: false
        }, async () => {
            return new Promise<void>((resolve, reject) => {
                // Use spawn instead of exec for better security
                // Pass arguments as an array instead of a command string
                const args = [
                    filePath,
                    '--output', outputDir,
                    '--output-type', outputType
                ];
                
                const process = spawn(COMMANDS.GENERATE_CMD, args);
                
                let errorOutput = '';
                process.stderr.on('data', (data: Buffer) => {
                    errorOutput += data.toString();
                });
                
                process.on('close', (code: number | null) => {
                    if (code !== 0) {
                        const errorMessage = `${MESSAGES.GENERATE_ERROR}: ${errorOutput || 'Process exited with code ' + code}`;
                        vscode.window.showErrorMessage(errorMessage);
                        reject(new Error(errorMessage));
                        return;
                    }
                    
                    vscode.window.showInformationMessage(`${MESSAGES.GENERATE_SUCCESS} ${outputDir}`);
                    resolve();
                });
                
                process.on('error', (error: Error) => {
                    const errorMessage = `${MESSAGES.GENERATE_ERROR}: ${error.message}`;
                    vscode.window.showErrorMessage(errorMessage);
                    reject(new Error(errorMessage));
                });
            });
        });
    } catch (error) {
        const errorMessage = error instanceof Error 
            ? `${MESSAGES.GENERATE_FAILED}: ${error.message}`
            : `${MESSAGES.GENERATE_FAILED}: ${String(error)}`;
        vscode.window.showErrorMessage(errorMessage);
    }
}

/**
 * Generates a single HTTP file containing all requests
 */
async function generateSingleFile(): Promise<void> {
    await generateHttpFile(OUTPUT_TYPES.ONE_FILE);
}

/**
 * Generates multiple HTTP files (one request per file)
 */
async function generateMultipleFiles(): Promise<void> {
    await generateHttpFile(OUTPUT_TYPES.ONE_REQUEST_PER_FILE);
}