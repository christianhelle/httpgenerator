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
    PATH_OUTSIDE_WORKSPACE: 'Path must be within the workspace boundaries',
    PROCESS_ERROR: 'Process execution error',
    DIRECTORY_CREATE_ERROR: 'Could not create output directory'
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

/**
 * Interface for error handling options
 */
interface ErrorHandlingOptions {
    errorPrefix: string;
    error: unknown;
    returnValue?: any;
}

/**
 * Utility for handling errors consistently
 * @param options Options for error handling
 * @returns The provided return value or undefined
 */
function handleError<T>(options: ErrorHandlingOptions): T | undefined {
    const errorMessage = options.error instanceof Error 
        ? `${options.errorPrefix}: ${options.error.message}`
        : `${options.errorPrefix}: ${String(options.error)}`;
    vscode.window.showErrorMessage(errorMessage);
    return options.returnValue;
}

/**
 * Interface for validation results
 */
interface ValidationResult {
    isValid: boolean;
    errorMessage?: string;
}

/**
 * Validates that a path is within the workspace boundaries to prevent path traversal
 * @param inputPath Path to validate
 * @returns Validation result object
 */
function validatePath(inputPath: string): ValidationResult {
    // Get all workspace folders
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders || workspaceFolders.length === 0) {
        return { isValid: false, errorMessage: MESSAGES.PATH_OUTSIDE_WORKSPACE };
    }
    
    // Normalize the input path to resolve any '..' or '.' segments
    const normalizedPath = path.normalize(inputPath);
    
    // Check if the normalized path is within any workspace folder
    const isWithinWorkspace = workspaceFolders.some((folder: vscode.WorkspaceFolder) => {
        const workspacePath = folder.uri.fsPath;
        return normalizedPath.startsWith(workspacePath);
    });

    return { 
        isValid: isWithinWorkspace,
        errorMessage: isWithinWorkspace ? undefined : MESSAGES.PATH_OUTSIDE_WORKSPACE
    };
}

/**
 * Validates if a file is a supported OpenAPI file type
 * @param filePath Path to the file
 * @returns Validation result object
 */
function validateFileType(filePath: string): ValidationResult {
    try {
        if (!fs.statSync(filePath).isFile()) {
            return { isValid: false, errorMessage: MESSAGES.UNSUPPORTED_FILE };
        }
        
        const ext = path.extname(filePath).toLowerCase();
        const isValid = FILE_EXTENSIONS.includes(ext);
        
        return { 
            isValid: isValid,
            errorMessage: isValid ? undefined : MESSAGES.UNSUPPORTED_FILE
        };
    } catch {
        return { isValid: false, errorMessage: MESSAGES.UNSUPPORTED_FILE };
    }
}

/**
 * Ensures that an output directory exists
 * @param dirPath Path to the directory
 * @returns Validation result object
 */
function ensureOutputDirectory(dirPath: string): ValidationResult {
    try {
        if (!fs.existsSync(dirPath)) {
            fs.mkdirSync(dirPath, { recursive: true });
        }
        return { isValid: true };
    } catch (error) {
        return { 
            isValid: false,
            errorMessage: `${MESSAGES.DIRECTORY_CREATE_ERROR}: ${error instanceof Error ? error.message : String(error)}` 
        };
    }
}

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
 * Interface for process execution options
 */
interface ProcessExecutionOptions {
    command: string;
    args: string[];
    progressTitle: string;
    errorPrefix: string;
    successMessage?: string;
}

/**
 * Execute a process with progress indication
 * @param options Options for process execution
 * @returns Promise resolving when the process completes successfully
 */
async function executeProcessWithProgress(options: ProcessExecutionOptions): Promise<void> {
    return vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: options.progressTitle,
        cancellable: false
    }, async () => {
        return new Promise<void>((resolve, reject) => {
            const process = spawn(options.command, options.args);
            
            let errorOutput = '';
            process.stderr.on('data', (data: Buffer) => {
                errorOutput += data.toString();
            });
            
            process.on('close', (code: number | null) => {
                if (code !== 0) {
                    const errorMessage = `${options.errorPrefix}: ${errorOutput || 'Process exited with code ' + code}`;
                    vscode.window.showErrorMessage(errorMessage);
                    reject(new Error(errorMessage));
                    return;
                }
                
                if (options.successMessage) {
                    vscode.window.showInformationMessage(options.successMessage);
                }
                resolve();
            });
            
            process.on('error', (error: Error) => {
                const errorMessage = `${options.errorPrefix}: ${error.message}`;
                vscode.window.showErrorMessage(errorMessage);
                reject(new Error(errorMessage));
            });
        });
    });
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
            // Use spawn instead of exec for better security
            const args = COMMANDS.TOOL_INSTALL.split(' ');
            const cmd = args.shift() || 'dotnet';
            
            await executeProcessWithProgress({
                command: cmd,
                args: args,
                progressTitle: MESSAGES.INSTALLING_TOOL,
                errorPrefix: MESSAGES.INSTALL_FAILED,
                successMessage: MESSAGES.INSTALL_SUCCESS
            });
            
            return true;
        }
        return false;
    } catch (error) {
        return handleError({ 
            errorPrefix: MESSAGES.INSTALL_ERROR, 
            error: error,
            returnValue: false
        }) ?? false;
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
    const safeFiles = openApiFiles.filter(file => validatePath(file.fsPath).isValid);
    
    if (safeFiles.length === 0) {
        vscode.window.showErrorMessage(MESSAGES.NO_OPENAPI_FILES);
        return undefined;
    }
    
    return vscode.window.showQuickPick(
        safeFiles.map(file => file.fsPath), 
        { placeHolder: MESSAGES.FILE_PICKER_PLACEHOLDER }
    );
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
        const pathValidation = validatePath(filePath);
        if (!pathValidation.isValid) {
            vscode.window.showErrorMessage(pathValidation.errorMessage!);
            return;
        }
        
        const fileTypeValidation = validateFileType(filePath);
        if (!fileTypeValidation.isValid) {
            vscode.window.showErrorMessage(fileTypeValidation.errorMessage!);
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
    const pathValidation = validatePath(filePath);
    if (!pathValidation.isValid) {
        vscode.window.showErrorMessage(pathValidation.errorMessage!);
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
    const outputPathValidation = validatePath(outputDir);
    if (!outputPathValidation.isValid) {
        vscode.window.showErrorMessage(outputPathValidation.errorMessage!);
        return;
    }
    
    // Ensure directory exists
    const dirValidation = ensureOutputDirectory(outputDir);
    if (!dirValidation.isValid) {
        vscode.window.showErrorMessage(dirValidation.errorMessage!);
        return;
    }
    
    try {
        const args = [
            filePath,
            '--output', outputDir,
            '--output-type', outputType
        ];
        
        await executeProcessWithProgress({
            command: COMMANDS.GENERATE_CMD,
            args: args,
            progressTitle: MESSAGES.GENERATING,
            errorPrefix: MESSAGES.GENERATE_ERROR,
            successMessage: `${MESSAGES.GENERATE_SUCCESS} ${outputDir}`
        });
    } catch (error) {
        handleError({ 
            errorPrefix: MESSAGES.GENERATE_FAILED, 
            error: error 
        });
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