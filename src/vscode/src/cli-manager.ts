import * as vscode from 'vscode';
import * as childProcess from 'child_process';
import * as crypto from 'crypto';
import * as fs from 'fs';
import * as https from 'https';
import * as os from 'os';
import * as path from 'path';
import { promisify } from 'util';

const execFile = promisify(childProcess.execFile);
const GITHUB_REPO = 'christianhelle/httpgenerator';
const BINARY_BASENAME = process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';
const VERSION_FILE = 'cli-version.json';
const CACHE_READY_FILE = '.cache-ready';
const GITHUB_API_TIMEOUT_MS = 15000;
const DOWNLOAD_TIMEOUT_MS = 30000;
const MAX_DOWNLOAD_BYTES = 100 * 1024 * 1024;

interface CachedVersion {
    version: string;
    downloadedAt: string;
}

interface ReleaseAsset {
    name: string;
    browser_download_url: string;
    size?: number;
}

interface GitHubRelease {
    assets: ReleaseAsset[];
}

function getExtensionVersion(context: vscode.ExtensionContext): string {
    const packageVersion = context.extension?.packageJSON?.version;
    return typeof packageVersion === 'string' && packageVersion.length > 0 ? packageVersion : '0.1.0';
}

function getConfigurationPath(): string | undefined {
    const configuredPath = vscode.workspace
        .getConfiguration('http-file-generator')
        .get<string>('executablePath');

    return configuredPath && configuredPath.trim().length > 0 ? configuredPath.trim() : undefined;
}

function cachePath(context: vscode.ExtensionContext, fileName: string): string {
    return path.join(context.globalStorageUri.fsPath, fileName);
}

function cachedBinaryPath(context: vscode.ExtensionContext): string {
    return cachePath(context, BINARY_BASENAME);
}

function normalizeReleaseVersion(version: string): string[] {
    const trimmed = version.trim();
    const withoutV = trimmed.startsWith('v') ? trimmed.slice(1) : trimmed;
    const withV = trimmed.startsWith('v') ? trimmed : `v${trimmed}`;
    return Array.from(new Set([trimmed, withoutV, withV]));
}

async function pathExists(filePath: string): Promise<boolean> {
    try {
        await fs.promises.access(filePath, fs.constants.F_OK);
        return true;
    } catch {
        return false;
    }
}

export async function verifyCLI(filePath: string): Promise<boolean> {
    try {
        const mode = process.platform === 'win32' ? fs.constants.F_OK : fs.constants.X_OK;
        await fs.promises.access(filePath, mode);
        return true;
    } catch {
        return false;
    }
}

async function findOnPath(commandName = BINARY_BASENAME): Promise<string | undefined> {
    const pathValue = process.env.PATH;
    if (!pathValue) {
        return undefined;
    }

    const pathExts = process.platform === 'win32'
        ? (process.env.PATHEXT ?? '.EXE;.CMD;.BAT;.COM').split(';')
        : [''];
    const executableCandidates = process.platform === 'win32' && path.extname(commandName).length === 0
        ? pathExts.map(ext => `${commandName}${ext.toLowerCase()}`)
        : [commandName];

    for (const directory of pathValue.split(path.delimiter)) {
        if (!directory) {
            continue;
        }

        for (const candidate of executableCandidates) {
            const fullPath = path.join(directory, candidate);
            if (await verifyCLI(fullPath)) {
                return fullPath;
            }
        }
    }

    return undefined;
}

export function getPlatformArch(): 'win-x64' | 'linux-x64' | 'darwin-x64' | 'darwin-arm64' {
    if (process.platform === 'win32') {
        return 'win-x64';
    }

    if (process.platform === 'linux' && process.arch === 'x64') {
        return 'linux-x64';
    }

    if (process.platform === 'darwin') {
        if (process.arch === 'arm64') {
            return 'darwin-arm64';
        }

        if (process.arch === 'x64') {
            return 'darwin-x64';
        }
    }

    throw new Error(`Unsupported platform: ${process.platform}-${process.arch}`);
}

export function getArtifactName(version: string): string {
    const platformArch = getPlatformArch();
    if (platformArch === 'win-x64') {
        return `httpgenerator-${version}-win-x64.zip`;
    }

    return `httpgenerator-${version}-${platformArch}.tar.gz`;
}

function getArtifactCandidates(version: string): string[] {
    const platformArch = getPlatformArch();
    const versions = normalizeReleaseVersion(version);
    const candidates: string[] = [];

    for (const candidateVersion of versions) {
        candidates.push(getArtifactName(candidateVersion));
    }

    if (platformArch === 'win-x64') {
        candidates.push('httpgenerator-win-x64.zip');
    } else {
        candidates.push(`httpgenerator-${platformArch}`);
    }

    return Array.from(new Set(candidates));
}

function requestJson<T>(url: string): Promise<T> {
    return new Promise((resolve, reject) => {
        const request = https.get(
            url,
            {
                headers: {
                    'Accept': 'application/vnd.github+json',
                    'User-Agent': 'http-file-generator-vscode'
                },
                timeout: GITHUB_API_TIMEOUT_MS
            },
            response => {
                if (response.statusCode && response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
                    response.resume();
                    requestJson<T>(response.headers.location).then(resolve, reject);
                    return;
                }

                if (response.statusCode !== 200) {
                    response.resume();
                    reject(new Error(`GitHub Releases returned HTTP ${response.statusCode}`));
                    return;
                }

                const chunks: Buffer[] = [];
                response.on('data', chunk => chunks.push(Buffer.from(chunk)));
                response.on('end', () => {
                    try {
                        resolve(JSON.parse(Buffer.concat(chunks).toString('utf8')) as T);
                    } catch (error) {
                        reject(error);
                    }
                });
            }
        );

        request.on('timeout', () => request.destroy(new Error('GitHub Releases request timed out')));
        request.on('error', reject);
    });
}

function downloadFile(url: string, destination: string, onProgress?: (message: string) => void): Promise<void> {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(destination);
        const request = https.get(
            url,
            {
                headers: { 'User-Agent': 'http-file-generator-vscode' },
                timeout: DOWNLOAD_TIMEOUT_MS
            },
            response => {
                if (response.statusCode && response.statusCode >= 300 && response.statusCode < 400) {
                    const redirectUrl = response.headers.location;
                    response.resume();
                    file.close();
                    if (!redirectUrl) {
                        reject(new Error('Download redirect did not include a location'));
                        return;
                    }

                    fs.promises.unlink(destination).catch(() => undefined).finally(() => {
                        downloadFile(redirectUrl, destination, onProgress).then(resolve, reject);
                    });
                    return;
                }

                if (response.statusCode !== 200) {
                    response.resume();
                    file.close();
                    reject(new Error(`Download returned HTTP ${response.statusCode}`));
                    return;
                }

                const totalBytes = Number(response.headers['content-length'] ?? 0);
                if (totalBytes > MAX_DOWNLOAD_BYTES) {
                    response.resume();
                    file.close();
                    reject(new Error('CLI download is larger than expected'));
                    return;
                }

                let downloadedBytes = 0;

                response.on('data', chunk => {
                    downloadedBytes += Buffer.byteLength(chunk);
                    if (downloadedBytes > MAX_DOWNLOAD_BYTES) {
                        response.destroy(new Error('CLI download is larger than expected'));
                        return;
                    }

                    if (totalBytes > 0) {
                        const percent = Math.round((downloadedBytes / totalBytes) * 100);
                        onProgress?.(`Downloading CLI (${percent}%)`);
                    }
                });

                response.pipe(file);
                file.on('finish', () => file.close(() => resolve()));
            }
        );

        const cleanup = (error: Error) => {
            file.close();
            fs.promises.unlink(destination).catch(() => undefined).finally(() => reject(error));
        };

        request.on('timeout', () => request.destroy(new Error('CLI download timed out')));
        request.on('error', cleanup);
        file.on('error', cleanup);
    });
}

function isArchive(assetName: string): boolean {
    return assetName.endsWith('.zip') || assetName.endsWith('.tar.gz') || assetName.endsWith('.tgz');
}

function normalizeArchiveEntry(entry: string): string {
    return path.posix.normalize(entry.replace(/\\/g, '/'));
}

function isSafeArchiveEntry(entry: string): boolean {
    const normalized = normalizeArchiveEntry(entry);
    return normalized.length > 0
        && normalized !== '..'
        && !normalized.startsWith('../')
        && !path.posix.isAbsolute(normalized)
        && !path.win32.isAbsolute(entry);
}

async function extractTarGz(archivePath: string, destinationDirectory: string): Promise<string> {
    const { stdout } = await execFile('tar', ['-tzf', archivePath]);
    const entries = stdout.split(/\r?\n/).filter(entry => entry.length > 0);
    if (entries.some(entry => !isSafeArchiveEntry(entry))) {
        throw new Error('Downloaded archive contains unsafe paths');
    }

    const binaryEntry = entries.find(entry => path.posix.basename(normalizeArchiveEntry(entry)) === BINARY_BASENAME);
    if (!binaryEntry) {
        throw new Error(`Downloaded archive does not contain ${BINARY_BASENAME}`);
    }

    await execFile('tar', ['-xzf', archivePath, '-C', destinationDirectory, binaryEntry]);
    return path.join(destinationDirectory, binaryEntry);
}

async function extractZip(archivePath: string, destinationDirectory: string): Promise<string> {
    if (process.platform === 'win32') {
        await execFile('powershell.exe', [
            '-NoProfile',
            '-NonInteractive',
            '-Command',
            'Expand-Archive -LiteralPath $args[0] -DestinationPath $args[1] -Force',
            archivePath,
            destinationDirectory
        ]);
    } else {
        await execFile('unzip', ['-q', archivePath, '-d', destinationDirectory]);
    }

    const binaryName = process.platform === 'win32' ? 'httpgenerator.exe' : 'httpgenerator';
    const matches = await findFilesByName(destinationDirectory, binaryName);
    if (matches.length === 0) {
        throw new Error(`Downloaded archive does not contain ${binaryName}`);
    }

    return matches[0];
}

async function findFilesByName(directory: string, fileName: string): Promise<string[]> {
    const matches: string[] = [];
    const entries = await fs.promises.readdir(directory, { withFileTypes: true });

    for (const entry of entries) {
        const entryPath = path.join(directory, entry.name);
        if (entry.isDirectory()) {
            matches.push(...await findFilesByName(entryPath, fileName));
        } else if (entry.isFile() && entry.name === fileName) {
            matches.push(entryPath);
        }
    }

    return matches;
}

async function prepareDownloadedBinary(downloadPath: string, assetName: string, targetPath: string): Promise<void> {
    if (!isArchive(assetName)) {
        await fs.promises.copyFile(downloadPath, targetPath);
    } else {
        const extractDirectory = await fs.promises.mkdtemp(path.join(os.tmpdir(), 'httpgenerator-extract-'));
        try {
            const extractedBinary = assetName.endsWith('.zip')
                ? await extractZip(downloadPath, extractDirectory)
                : await extractTarGz(downloadPath, extractDirectory);
            await fs.promises.copyFile(extractedBinary, targetPath);
        } finally {
            await fs.promises.rm(extractDirectory, { force: true, recursive: true });
        }
    }

    const stats = await fs.promises.stat(targetPath);
    if (stats.size > MAX_DOWNLOAD_BYTES) {
        throw new Error('Extracted CLI is larger than expected');
    }

    if (process.platform !== 'win32') {
        await fs.promises.chmod(targetPath, 0o755);
    }
}

async function getReleaseAsset(version: string): Promise<{ version: string; asset: ReleaseAsset }> {
    let lastError: unknown;
    for (const candidateVersion of normalizeReleaseVersion(version)) {
        try {
            const release = await requestJson<GitHubRelease>(`https://api.github.com/repos/${GITHUB_REPO}/releases/tags/${candidateVersion}`);
            const candidates = getArtifactCandidates(candidateVersion);
            const asset = release.assets.find(item => candidates.includes(item.name));
            if (!asset) {
                throw new Error(`No ${getPlatformArch()} asset found in release ${candidateVersion}`);
            }

            if (asset.size && asset.size > MAX_DOWNLOAD_BYTES) {
                throw new Error(`Release asset ${asset.name} is larger than expected`);
            }

            return { version: candidateVersion, asset };
        } catch (error) {
            lastError = error;
        }
    }

    throw lastError instanceof Error ? lastError : new Error('Failed to locate a matching GitHub Release asset');
}

export function getCachedCLIVersion(context: vscode.ExtensionContext): string | undefined {
    try {
        const versionFile = cachePath(context, VERSION_FILE);
        const contents = fs.readFileSync(versionFile, 'utf8');
        const cached = JSON.parse(contents) as CachedVersion;
        return typeof cached.version === 'string' ? cached.version : undefined;
    } catch {
        return undefined;
    }
}

export async function setCachedCLIVersion(context: vscode.ExtensionContext, version: string): Promise<void> {
    await fs.promises.mkdir(context.globalStorageUri.fsPath, { recursive: true });
    const cached: CachedVersion = {
        version,
        downloadedAt: new Date().toISOString()
    };
    await fs.promises.writeFile(cachePath(context, VERSION_FILE), `${JSON.stringify(cached, null, 2)}\n`, 'utf8');
}

export async function downloadCLI(
    context: vscode.ExtensionContext,
    version: string,
    onProgress?: (message: string) => void
): Promise<vscode.Uri> {
    await fs.promises.mkdir(context.globalStorageUri.fsPath, { recursive: true });
    onProgress?.('Locating CLI release asset');
    const { version: releaseVersion, asset } = await getReleaseAsset(version);
    const tempDirectory = await fs.promises.mkdtemp(path.join(os.tmpdir(), 'httpgenerator-download-'));
    const downloadPath = path.join(tempDirectory, asset.name);
    const targetPath = cachedBinaryPath(context);
    const tempTargetDirectory = await fs.promises.mkdtemp(path.join(context.globalStorageUri.fsPath, '.download-'));
    const tempTargetPath = path.join(tempTargetDirectory, `${BINARY_BASENAME}.${crypto.randomUUID()}.tmp`);

    try {
        onProgress?.('Downloading CLI');
        await downloadFile(asset.browser_download_url, downloadPath, onProgress);
        onProgress?.('Installing CLI');
        await prepareDownloadedBinary(downloadPath, asset.name, tempTargetPath);
        if (!await verifyCLI(tempTargetPath)) {
            throw new Error('Downloaded CLI is not executable');
        }

        await fs.promises.rm(targetPath, { force: true }).catch(() => undefined);
        await fs.promises.rename(tempTargetPath, targetPath);
        await setCachedCLIVersion(context, releaseVersion.replace(/^v/, ''));
        await fs.promises.writeFile(cachePath(context, CACHE_READY_FILE), new Date().toISOString(), 'utf8');
        return vscode.Uri.file(targetPath);
    } finally {
        await fs.promises.rm(tempDirectory, { force: true, recursive: true });
        await fs.promises.rm(tempTargetDirectory, { force: true, recursive: true }).catch(() => undefined);
    }
}

async function getCachedCLIPath(context: vscode.ExtensionContext, expectedVersion: string): Promise<string | undefined> {
    const binaryPath = cachedBinaryPath(context);
    const readyPath = cachePath(context, CACHE_READY_FILE);
    const cachedVersion = getCachedCLIVersion(context);

    if (cachedVersion === expectedVersion && await pathExists(readyPath) && await verifyCLI(binaryPath)) {
        return binaryPath;
    }

    return undefined;
}

export async function resolveCLIPath(
    context: vscode.ExtensionContext,
    onProgress?: (message: string) => void
): Promise<string | undefined> {
    const configuredPath = getConfigurationPath();
    if (configuredPath && await verifyCLI(configuredPath)) {
        return configuredPath;
    }

    const pathBinary = await findOnPath('httpgenerator');
    if (pathBinary) {
        return pathBinary;
    }

    const expectedVersion = getExtensionVersion(context);
    const cachedPath = await getCachedCLIPath(context, expectedVersion);
    if (cachedPath) {
        return cachedPath;
    }

    const downloadedUri = await downloadCLI(context, expectedVersion, onProgress);
    return downloadedUri.fsPath;
}

export async function resetCLI(context: vscode.ExtensionContext): Promise<boolean> {
    const files = [
        cachedBinaryPath(context),
        cachePath(context, VERSION_FILE),
        cachePath(context, CACHE_READY_FILE)
    ];

    await Promise.all(files.map(file => fs.promises.rm(file, { force: true }).catch(() => undefined)));
    return true;
}
