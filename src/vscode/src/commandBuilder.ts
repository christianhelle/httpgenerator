export type ShellKind = 'powershell' | 'cmd' | 'posix';

function quoteArgument(value: string, shellKind: ShellKind): string {
    switch (shellKind) {
        case 'powershell':
            return `'${value.replace(/'/g, "''")}'`;
        case 'cmd':
            return `"${value.replace(/"/g, '\\"')}"`;
        case 'posix':
        default:
            return `'${value.replace(/'/g, `'"'"'`)}'`;
    }
}

export function createHttpGeneratorCommandForShell(
    executablePath: string,
    filePath: string,
    outputFolder: string,
    outputType: string,
    shellKind: ShellKind
): string {
    const quotedExecutable = quoteArgument(executablePath, shellKind);
    const quotedInputFile = quoteArgument(filePath, shellKind);
    const quotedOutputFolder = quoteArgument(outputFolder, shellKind);
    const quotedOutputType = quoteArgument(outputType, shellKind);
    const args = `${quotedInputFile} --output ${quotedOutputFolder} --output-type ${quotedOutputType}`;

    // PowerShell requires the call operator to invoke quoted executable paths.
    if (shellKind === 'powershell') {
        return `& ${quotedExecutable} ${args}`;
    }

    return `${quotedExecutable} ${args}`;
}
