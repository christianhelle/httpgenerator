using System.Diagnostics;
using System.Reflection;

namespace HttpGenerator.VSIX;

internal static class CliInstaller
{
    private const string CommandName = "httpgenerator";
    private const string BinaryName = CommandName + ".exe";
    private const string InstallScriptCommand = "irm https://christianhelle.com/httpgenerator/install.ps1 | iex";
    private const string PathOverrideEnvironmentVariable = "HTTPGENERATOR_PATH";

    private static readonly object installLock = new();
    private static Task<string>? installTask;

    public static void EnsureInstalledInBackground()
    {
        _ = Task.Run(async () =>
        {
            try
            {
                await ResolveOrInstallAsync(CancellationToken.None);
            }
            catch
            {
                // The command path reports installation failures when the user invokes the extension.
            }
        });
    }

    public static async Task<string> ResolveOrInstallAsync(CancellationToken cancellationToken)
    {
        var resolvedPath = Resolve();
        if (resolvedPath is not null)
        {
            return resolvedPath;
        }

        Task<string> currentInstallTask;
        lock (installLock)
        {
            installTask ??= InstallAsync(cancellationToken);
            currentInstallTask = installTask;
        }

        try
        {
            return await currentInstallTask;
        }
        catch
        {
            lock (installLock)
            {
                if (installTask == currentInstallTask)
                {
                    installTask = null;
                }
            }

            throw;
        }
    }

    private static async Task<string> InstallAsync(CancellationToken cancellationToken)
    {
        var startInfo = new ProcessStartInfo("powershell.exe")
        {
            Arguments = $"-NoProfile -Command \"{InstallScriptCommand}\"",
            CreateNoWindow = true,
            RedirectStandardError = true,
            RedirectStandardOutput = true,
            UseShellExecute = false,
        };

        using var process = Process.Start(startInfo)
            ?? throw new InvalidOperationException("Failed to start PowerShell to install httpgenerator.");

        var outputTask = process.StandardOutput.ReadToEndAsync();
        var errorTask = process.StandardError.ReadToEndAsync();
        await Task.Run(() => ProcessRunner.WaitForExit(process, cancellationToken), cancellationToken);

        var output = await outputTask;
        var error = await errorTask;
        if (process.ExitCode != 0)
        {
            throw new InvalidOperationException(
                $"httpgenerator installation failed with exit code {process.ExitCode}.{Environment.NewLine}{output}{Environment.NewLine}{error}");
        }

        return Resolve()
            ?? throw new InvalidOperationException("httpgenerator was installed but could not be found.");
    }

    private static string? Resolve()
    {
        return ResolveEnvironmentOverride()
            ?? ResolveDevelopmentExecutable()
            ?? ResolveDefaultInstallExecutable()
            ?? ResolvePathExecutable();
    }

    private static string? ResolveEnvironmentOverride()
    {
        var overridePath = Environment.GetEnvironmentVariable(PathOverrideEnvironmentVariable);
        if (string.IsNullOrWhiteSpace(overridePath))
        {
            return null;
        }

        if (File.Exists(overridePath))
        {
            return overridePath;
        }

        var binaryPath = Path.Combine(overridePath, BinaryName);
        return File.Exists(binaryPath) ? binaryPath : null;
    }

    private static string? ResolveDefaultInstallExecutable()
    {
        var localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
        var userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        var candidates = new[]
        {
            Path.Combine(localAppData, "Programs", "httpgenerator", BinaryName),
            Path.Combine(userProfile, ".local", "bin", BinaryName),
            Path.Combine(userProfile, "bin", BinaryName),
        };

        return candidates.FirstOrDefault(File.Exists);
    }

    private static string? ResolveDevelopmentExecutable()
    {
        var location = Assembly.GetExecutingAssembly().Location;
        var directory = Path.GetDirectoryName(location);
        while (!string.IsNullOrWhiteSpace(directory))
        {
            if (File.Exists(Path.Combine(directory, "Cargo.toml")) &&
                Directory.Exists(Path.Combine(directory, "src", "rust", "cli")))
            {
                var candidates = new[]
                {
                    Path.Combine(directory, "target", "debug", BinaryName),
                    Path.Combine(directory, "target", "release", BinaryName),
                };

                return candidates.FirstOrDefault(File.Exists);
            }

            directory = Path.GetDirectoryName(directory);
        }

        return null;
    }

    private static string? ResolvePathExecutable()
    {
        var path = Environment.GetEnvironmentVariable("PATH");
        if (string.IsNullOrWhiteSpace(path))
        {
            return null;
        }

        foreach (var entry in path.Split(Path.PathSeparator).Where(entry => !string.IsNullOrWhiteSpace(entry)))
        {
            var candidate = Path.Combine(entry.Trim('"'), BinaryName);
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        return null;
    }
}
