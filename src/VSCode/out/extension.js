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
function activate(context) {
    console.log('HTTP File Generator for VS Code extension is now active');
    // Register commands
    context.subscriptions.push(vscode.commands.registerCommand('http-file-generator.generateSingleFile', generateSingleFile), vscode.commands.registerCommand('http-file-generator.generateMultipleFiles', generateMultipleFiles));
}
function deactivate() {
    // Nothing to clean up
}
async function checkToolInstalled() {
    try {
        (0, child_process_1.execSync)('dotnet tool list -g', { stdio: 'pipe' }).toString().includes('httpgenerator');
        return true;
    }
    catch (error) {
        return false;
    }
}
async function installTool() {
    try {
        const response = await vscode.window.showInformationMessage('The httpgenerator .NET tool is not installed. Do you want to install it?', 'Yes', 'No');
        if (response === 'Yes') {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Installing httpgenerator...',
                cancellable: false
            }, async (_progress) => {
                return new Promise((resolve, reject) => {
                    (0, child_process_1.exec)('dotnet tool install --global httpgenerator', (error, _stdout, _stderr) => {
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
    }
    catch (error) {
        vscode.window.showErrorMessage(`Error installing httpgenerator: ${error}`);
        return false;
    }
}
async function generateHttpFile(outputType) {
    const fileUri = vscode.window.activeTextEditor?.document.uri ||
        (vscode.window.activeTextEditor ? undefined : vscode.workspace.workspaceFolders?.[0]?.uri);
    if (!fileUri) {
        const openApiFiles = await findOpenApiFiles();
        if (openApiFiles.length === 0) {
            vscode.window.showErrorMessage('No OpenAPI specification files found in workspace');
            return;
        }
        const selectedFile = await vscode.window.showQuickPick(openApiFiles.map(file => file.fsPath), { placeHolder: 'Select an OpenAPI specification file' });
        if (!selectedFile) {
            return;
        }
        await runGenerator(selectedFile, outputType);
    }
    else {
        const filePath = fileUri.fsPath;
        // Check if right-clicked on a file in Explorer
        if (fs.statSync(filePath).isFile()) {
            const ext = path.extname(filePath).toLowerCase();
            if (['.json', '.yaml', '.yml'].includes(ext)) {
                await runGenerator(filePath, outputType);
            }
            else {
                vscode.window.showErrorMessage('Selected file is not a supported OpenAPI specification (JSON or YAML)');
            }
        }
        else {
            vscode.window.showErrorMessage('Please select an OpenAPI specification file (JSON or YAML)');
        }
    }
}
async function findOpenApiFiles() {
    const files = await vscode.workspace.findFiles('**/*.{json,yaml,yml}', '**/node_modules/**');
    return files;
}
async function runGenerator(filePath, outputType) {
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
        }, async (_progress) => {
            return new Promise((resolve, reject) => {
                const command = `httpgenerator "${filePath}" --output "${outputDir}" --output-type ${outputType}`;
                (0, child_process_1.exec)(command, (error, _stdout, _stderr) => {
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
    }
    catch (error) {
        vscode.window.showErrorMessage(`Failed to generate HTTP files: ${error}`);
    }
}
async function generateSingleFile() {
    await generateHttpFile('OneFile');
}
async function generateMultipleFiles() {
    await generateHttpFile('OneRequestPerFile');
}
//# sourceMappingURL=extension.js.map