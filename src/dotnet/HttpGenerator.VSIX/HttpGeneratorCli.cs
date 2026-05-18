using System.Diagnostics;

using Microsoft.VisualStudio.Extensibility.Shell;

namespace HttpGenerator.VSIX;

internal static class HttpGeneratorCli
{
    private const string HttpGeneratorExecutableName = "httpgenerator.exe";
    private const string HttpGeneratorPathEnvironmentVariable = "HTTPGENERATOR_PATH";

    public static async Task<GenerateResult> ExecuteAsync(
        string openApiPath,
        string outputFolder,
        string? baseUrl,
        string contentType,
        string? authorizationHeader,
        bool generateMultipleFiles,
        ProgressReporter progress,
        CancellationToken cancellationToken)
    {
        progress.Report(new(15, "Resolving httpgenerator.exe..."));

        var executablePath = ResolveExecutablePath();
        progress.Report(new(30, $"Using httpgenerator: {executablePath}"));

        Directory.CreateDirectory(outputFolder);

        var arguments = CliArgumentBuilder.BuildArguments(
            openApiPath,
            outputFolder,
            baseUrl,
            contentType,
            authorizationHeader,
            generateMultipleFiles);

        progress.Report(new(60, "Generating .http files..."));

        var processResult = await RunProcessAsync(
            executablePath,
            arguments,
            workingDirectory: Path.GetDirectoryName(openApiPath) ?? Environment.CurrentDirectory,
            cancellationToken).ConfigureAwait(false);

        if (processResult.Cancelled)
        {
            progress.Report(new(100, "Generation cancelled."));
            return CliOutputParser.CreateCancelled(
                processResult.StandardOutput,
                processResult.StandardError,
                outputFolder,
                executablePath);
        }

        if (processResult.ExitCode != 0)
        {
            var failure = CliOutputParser.CreateFailure(
                processResult.ExitCode,
                processResult.StandardOutput,
                processResult.StandardError,
                outputFolder,
                executablePath);

            throw new HttpGeneratorCliException(failure);
        }

        var parsedResult = CliOutputParser.CreateSuccess(
            processResult.StandardOutput,
            processResult.StandardError,
            outputFolder,
            executablePath);

        progress.Report(new(100, parsedResult.Summary));
        return parsedResult;
    }

    private static string ResolveExecutablePath()
    {
        var configuredPath = Environment.GetEnvironmentVariable(HttpGeneratorPathEnvironmentVariable);
        if (!string.IsNullOrWhiteSpace(configuredPath))
        {
            var explicitPath = NormalizeConfiguredPath(configuredPath);

            if (!File.Exists(explicitPath))
            {
                throw new FileNotFoundException(
                    $"The {HttpGeneratorPathEnvironmentVariable} setting points to '{explicitPath}', but httpgenerator.exe was not found.");
            }

            return explicitPath;
        }

        foreach (var candidate in GetBundledExecutableCandidates())
        {
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        var repositoryRoot = TryFindRepositoryRoot(AppContext.BaseDirectory);
        if (repositoryRoot is not null)
        {
            foreach (var candidate in GetRepositoryExecutableCandidates(repositoryRoot))
            {
                if (File.Exists(candidate))
                {
                    return candidate;
                }
            }
        }

        var pathCandidate = ResolveFromPath();
        if (!string.IsNullOrWhiteSpace(pathCandidate))
        {
            return pathCandidate;
        }

        throw new FileNotFoundException(
            "Could not locate httpgenerator.exe. Checked HTTPGENERATOR_PATH, bundled VSIX payload, repository target\\debug and target\\release outputs, and PATH.");
    }

    private static IEnumerable<string> GetBundledExecutableCandidates()
    {
        var baseDirectory = AppContext.BaseDirectory;

        yield return Path.Combine(baseDirectory, HttpGeneratorExecutableName);
        yield return Path.Combine(baseDirectory, "bin", HttpGeneratorExecutableName);
    }

    private static IEnumerable<string> GetRepositoryExecutableCandidates(string repositoryRoot)
    {
        yield return Path.Combine(repositoryRoot, "target", "debug", HttpGeneratorExecutableName);
        yield return Path.Combine(repositoryRoot, "target", "release", HttpGeneratorExecutableName);
    }

    private static string? ResolveFromPath()
    {
        var pathValue = Environment.GetEnvironmentVariable("PATH");
        if (string.IsNullOrWhiteSpace(pathValue))
        {
            return null;
        }

        foreach (var segment in pathValue.Split(Path.PathSeparator, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
        {
            var candidate = Path.Combine(segment.Trim('"'), HttpGeneratorExecutableName);
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        return null;
    }

    private static string NormalizeConfiguredPath(string configuredPath)
    {
        var expanded = Environment.ExpandEnvironmentVariables(configuredPath.Trim().Trim('"'));

        if (Directory.Exists(expanded))
        {
            return Path.Combine(expanded, HttpGeneratorExecutableName);
        }

        return Path.GetFullPath(expanded);
    }

    private static string? TryFindRepositoryRoot(string startDirectory)
    {
        var current = new DirectoryInfo(startDirectory);

        while (current is not null)
        {
            var cargoToml = Path.Combine(current.FullName, "Cargo.toml");
            var vsixProject = Path.Combine(current.FullName, "src", "dotnet", "HttpGenerator.VSIX", "HttpGenerator.VSIX.csproj");

            if (File.Exists(cargoToml) && File.Exists(vsixProject))
            {
                return current.FullName;
            }

            current = current.Parent;
        }

        return null;
    }

    private static async Task<ProcessExecutionResult> RunProcessAsync(
        string fileName,
        IReadOnlyList<string> arguments,
        string workingDirectory,
        CancellationToken cancellationToken)
    {
        var startInfo = new ProcessStartInfo
        {
            FileName = fileName,
            WorkingDirectory = workingDirectory,
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };

        foreach (var argument in arguments)
        {
            startInfo.ArgumentList.Add(argument);
        }

        using var process = Process.Start(startInfo)
            ?? throw new InvalidOperationException($"Failed to start '{fileName}'.");

        var stdoutTask = process.StandardOutput.ReadToEndAsync();
        var stderrTask = process.StandardError.ReadToEndAsync();

        var cancelled = false;
        using var registration = cancellationToken.Register(() =>
        {
            try
            {
                if (process.HasExited)
                {
                    return;
                }

                cancelled = true;
                process.Kill(entireProcessTree: true);
            }
            catch
            {
                // Best-effort cancellation cleanup.
            }
        });

        await process.WaitForExitAsync(CancellationToken.None).ConfigureAwait(false);
        var stdout = await stdoutTask.ConfigureAwait(false);
        var stderr = await stderrTask.ConfigureAwait(false);

        return new ProcessExecutionResult(
            process.ExitCode,
            stdout,
            stderr,
            cancelled);
    }

    private readonly record struct ProcessExecutionResult(
        int ExitCode,
        string StandardOutput,
        string StandardError,
        bool Cancelled);
}
