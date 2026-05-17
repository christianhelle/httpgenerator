using System.Diagnostics;
using System.IO;
using System.Threading;

namespace HttpGenerator.VSIX;

internal static class HttpGeneratorCli
{
    private const string DefaultPinnedVersion = "1.1.0";
    private const string OutputSectionMarker = "Files written successfully:";

    public static async Task<GenerateResult> ExecuteAsync(
        string openApiPath,
        string outputFolder,
        string? baseUrl,
        string contentType,
        string? authorizationHeader,
        bool generateMultipleFiles,
        IProgress<string>? progress,
        CancellationToken cancellationToken)
    {
        var pinnedVersion = DefaultPinnedVersion;
        var executablePath = ResolveExecutable(pinnedVersion);

        if (executablePath == null)
        {
            var tempDir = Path.Combine(Path.GetTempPath(), "httpgenerator-install-" + Guid.NewGuid().ToString("N"));
            Directory.CreateDirectory(tempDir);

            var installScriptPath = Path.Combine(tempDir, "install.ps1");
            var installScript = await GetEmbeddedResourceAsync("install.ps1", cancellationToken);

            await File.WriteAllTextAsync(installScriptPath, installScript, cancellationToken);

            var installDir = GetInstallDirectory(pinnedVersion);
            Directory.CreateDirectory(installDir);

            var psi = new ProcessStartInfo
            {
                FileName = "powershell",
                Arguments = $"-NoProfile -ExecutionPolicy Bypass -File \"{installScriptPath}\" -Version {pinnedVersion}",
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                CreateNoWindow = true,
            };

            using var installProcess = Process.Start(psi) ?? throw new InvalidOperationException("Failed to start PowerShell.");

            var installOutput = await installProcess.StandardOutput.ReadToEndAsync().ConfigureAwait(false);
            var installError = await installProcess.StandardError.ReadToEndAsync().ConfigureAwait(false);

            await installProcess.WaitForExitAsync(cancellationToken).ConfigureAwait(false);

            if (installProcess.ExitCode != 0)
            {
                throw new InvalidOperationException(
                    $"Failed to install httpgenerator CLI.\n\n{installError}");
            }

            executablePath = Path.Combine(installDir, "httpgenerator.exe");

            if (!File.Exists(executablePath))
            {
                throw new InvalidOperationException(
                    $"The installed binary was not found at {executablePath}.");
            }

            var success = await VerifyExecutableAsync(executablePath, pinnedVersion, cancellationToken).ConfigureAwait(false);
            if (!success)
            {
                throw new InvalidOperationException(
                    "The installed httpgenerator binary failed version verification. Please try downloading the latest version manually from https://github.com/christianhelle/httpgenerator/releases.");
            }
        }
        else
        {
            progress?.Report($"Using cached httpgenerator: {executablePath}");
        }

        var args = CliArgumentBuilder.BuildArguments(
            openApiPath,
            outputFolder,
            baseUrl,
            contentType,
            authorizationHeader,
            generateMultipleFiles);

        progress?.Report("Generating .http files...");

        var psi2 = new ProcessStartInfo
        {
            FileName = executablePath,
            Arguments = args,
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };

        using var process = Process.Start(psi2) ?? throw new InvalidOperationException("Failed to start httpgenerator.");

        var stdout = await process.StandardOutput.ReadToEndAsync().ConfigureAwait(false);
        var stderr = await process.StandardError.ReadToEndAsync().ConfigureAwait(false);

        await process.WaitForExitAsync(cancellationToken).ConfigureAwait(false);

        if (process.ExitCode != 0)
        {
            var message = $"httpgenerator exited with code {process.ExitCode}.\n\n{stderr}";
            throw new InvalidOperationException(message);
        }

        var result = CliOutputParser.ParseOutput(stdout, outputFolder);

        if (result.Success)
        {
            progress?.Report($"Successfully generated {result.FileCount} file(s).");
        }
        else
        {
            progress?.Report("Generation completed but could not parse output. Files may have been generated in the output folder.");
        }

        return result;
    }

    private static string? ResolveExecutable(string version)
    {
        var installDir = GetInstallDirectory(version);
        var executablePath = Path.Combine(installDir, "httpgenerator.exe");

        if (File.Exists(executablePath))
        {
            return executablePath;
        }

        return null;
    }

    private static string GetInstallDirectory(string version)
    {
        var appData = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        return Path.Combine(appData, ".local", "bin");
    }

    private static async Task<bool> VerifyExecutableAsync(string path, string expectedVersion, CancellationToken cancellationToken)
    {
        try
        {
            var psi = new ProcessStartInfo
            {
                FileName = path,
                Arguments = "--version",
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                CreateNoWindow = true,
            };

            using var process = Process.Start(psi) ?? throw new InvalidOperationException("Failed to start httpgenerator.");

            var output = await process.StandardOutput.ReadToEndAsync().ConfigureAwait(false);
            await process.WaitForExitAsync(cancellationToken).ConfigureAwait(false);

            if (process.ExitCode == 0 && output.Contains(expectedVersion))
            {
                return true;
            }

            return false;
        }
        catch
        {
            return false;
        }
    }

    private static async Task<string> GetEmbeddedResourceAsync(string resourceName, CancellationToken cancellationToken)
    {
        var assembly = typeof(HttpGeneratorCli).Assembly;
        var resourceNames = assembly.GetManifestResourceNames();

        var matchingResource = resourceNames.FirstOrDefault(r => r.EndsWith(resourceName));

        if (matchingResource == null)
        {
            throw new FileNotFoundException($"Embedded resource '{resourceName}' not found.");
        }

        using var stream = assembly.GetManifestResourceStream(matchingResource)
            ?? throw new FileNotFoundException($"Embedded resource '{resourceName}' not found.");

        using var reader = new StreamReader(stream);
        return await reader.ReadToEndAsync().ConfigureAwait(false);
    }
}
