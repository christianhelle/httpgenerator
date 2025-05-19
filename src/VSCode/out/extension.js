"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const child_process_1 = require("child_process");
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
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
    GENERATE_SUCCESS: 'HTTP files generated successfully in'
};
const COMMANDS = {
    TOOL_LIST: 'dotnet tool list -g',
    TOOL_INSTALL: 'dotnet tool install --global httpgenerator',
    GENERATE: 'httpgenerator "{0}" --output "{1}" --output-type {2}'
};
const FILE_EXTENSIONS = ['.json', '.yaml', '.yml'];
const OUTPUT_TYPES = {
    ONE_FILE: 'OneFile',
    ONE_REQUEST_PER_FILE: 'OneRequestPerFile'
};
function activate(context) {
    console.log(MESSAGES.EXTENSION_ACTIVE);
    // Register commands
    context.subscriptions.push(vscode.commands.registerCommand('http-file-generator.generateSingleFile', generateSingleFile), vscode.commands.registerCommand('http-file-generator.generateMultipleFiles', generateMultipleFiles));
}
function deactivate() {
    // Nothing to clean up
}
/**
 * Checks if the httpgenerator tool is installed
 * @returns Promise resolving to true if tool is installed, false otherwise
 */
async function checkToolInstalled() {
    try {
        const output = (0, child_process_1.execSync)(COMMANDS.TOOL_LIST, { stdio: 'pipe' }).toString();
        return output.includes('httpgenerator');
    }
    catch {
        return false;
    }
}
/**
 * Installs the httpgenerator tool
 * @returns Promise resolving to true if installation succeeded, false otherwise
 */
async function installTool() {
    try {
        const response = await vscode.window.showInformationMessage(MESSAGES.TOOL_NOT_INSTALLED, 'Yes', 'No');
        if (response === 'Yes') {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: MESSAGES.INSTALLING_TOOL,
                cancellable: false
            }, async () => {
                return new Promise((resolve, reject) => {
                    (0, child_process_1.exec)(COMMANDS.TOOL_INSTALL, (error) => {
                        if (error) {
                            const errorMessage = `${MESSAGES.INSTALL_FAILED}: ${error.message}`;
                            vscode.window.showErrorMessage(errorMessage);
                            reject(new Error(errorMessage));
                            return;
                        }
                        vscode.window.showInformationMessage(MESSAGES.INSTALL_SUCCESS);
                        resolve();
                    });
                });
            });
            return true;
        }
        return false;
    }
    catch (error) {
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
async function findOpenApiFiles() {
    const files = await vscode.workspace.findFiles('**/*.{json,yaml,yml}', '**/node_modules/**');
    return files;
}
/**
 * Handles file selection logic when no file is directly selected
 * @returns Promise resolving to selected file path or undefined if canceled
 */
async function handleFileSelection() {
    const openApiFiles = await findOpenApiFiles();
    if (openApiFiles.length === 0) {
        vscode.window.showErrorMessage(MESSAGES.NO_OPENAPI_FILES);
        return undefined;
    }
    const selectedFile = await vscode.window.showQuickPick(openApiFiles.map(file => file.fsPath), { placeHolder: MESSAGES.FILE_PICKER_PLACEHOLDER });
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
function isValidOpenApiFile(filePath) {
    if (!fs.statSync(filePath).isFile()) {
        return false;
    }
    const ext = path.extname(filePath).toLowerCase();
    return FILE_EXTENSIONS.includes(ext);
}
/**
 * Generates HTTP file(s) based on selected OpenAPI specification
 * @param outputType Type of output to generate (OneFile or OneRequestPerFile)
 */
async function generateHttpFile(outputType) {
    const fileUri = vscode.window.activeTextEditor?.document.uri ||
        (vscode.window.activeTextEditor ? undefined : vscode.workspace.workspaceFolders?.[0]?.uri);
    let filePath;
    if (!fileUri) {
        filePath = await handleFileSelection();
        if (!filePath) {
            return;
        }
    }
    else {
        filePath = fileUri.fsPath;
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
async function runGenerator(filePath, outputType) {
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
    // Ensure directory exists
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }
    try {
        await vscode.window.withProgress({
            location: vscode.ProgressLocation.Notification,
            title: MESSAGES.GENERATING,
            cancellable: false
        }, async () => {
            return new Promise((resolve, reject) => {
                // Format command string with parameters
                const command = COMMANDS.GENERATE
                    .replace('{0}', filePath)
                    .replace('{1}', outputDir)
                    .replace('{2}', outputType);
                (0, child_process_1.exec)(command, (error) => {
                    if (error) {
                        const errorMessage = `${MESSAGES.GENERATE_ERROR}: ${error.message}`;
                        vscode.window.showErrorMessage(errorMessage);
                        reject(new Error(errorMessage));
                        return;
                    }
                    vscode.window.showInformationMessage(`${MESSAGES.GENERATE_SUCCESS} ${outputDir}`);
                    resolve();
                });
            });
        });
    }
    catch (error) {
        const errorMessage = error instanceof Error
            ? `${MESSAGES.GENERATE_FAILED}: ${error.message}`
            : `${MESSAGES.GENERATE_FAILED}: ${String(error)}`;
        vscode.window.showErrorMessage(errorMessage);
    }
}
/**
 * Generates a single HTTP file containing all requests
 */
async function generateSingleFile() {
    await generateHttpFile(OUTPUT_TYPES.ONE_FILE);
}
/**
 * Generates multiple HTTP files (one request per file)
 */
async function generateMultipleFiles() {
    await generateHttpFile(OUTPUT_TYPES.ONE_REQUEST_PER_FILE);
}
//# sourceMappingURL=extension.js.map