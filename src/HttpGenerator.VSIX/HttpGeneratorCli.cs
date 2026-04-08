using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace HttpGenerator.VSIX;

internal sealed class HttpGeneratorCliOptions
{
    public string OpenApiPath { get; set; } = string.Empty;

    public string OutputFolder { get; set; } = string.Empty;

    public string BaseUrl { get; set; } = string.Empty;

    public string ContentType { get; set; } = string.Empty;

    public string AuthorizationHeader { get; set; } = string.Empty;

    public string? AzureTenantId { get; set; }

    public string AzureScope { get; set; } = string.Empty;

    public string OutputType { get; set; } = "OneRequestPerFile";
}

internal static class HttpGeneratorCli
{
    private const string ExecutableOverrideEnvironmentVariable = "HTTPGENERATOR_PATH";
    private const string ExecutableName = "httpgenerator.exe";

    public static async Task<string?> GenerateAsync(HttpGeneratorCliOptions options)
    {
        var executablePath = ResolveExecutablePath();
        var startInfo = new ProcessStartInfo
        {
            FileName = executablePath,
            Arguments = BuildArguments(options),
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };

        using var process = new Process { StartInfo = startInfo };
        if (!process.Start())
        {
            throw new InvalidOperationException("Failed to start the Rust httpgenerator executable.");
        }

        var stdoutTask = process.StandardOutput.ReadToEndAsync();
        var stderrTask = process.StandardError.ReadToEndAsync();

        await Task.Run(process.WaitForExit);

        var stdout = await stdoutTask;
        var stderr = await stderrTask;

        if (process.ExitCode == 0)
        {
            return string.IsNullOrWhiteSpace(stderr) ? null : stderr.Trim();
        }

        var failureOutput = string.IsNullOrWhiteSpace(stderr) ? stdout : stderr;
        failureOutput = failureOutput.Trim();

        throw new InvalidOperationException(
            string.IsNullOrWhiteSpace(failureOutput)
                ? "Rust httpgenerator generation failed."
                : failureOutput);
    }

    private static string ResolveExecutablePath()
    {
        var configuredPath = Environment.GetEnvironmentVariable(ExecutableOverrideEnvironmentVariable);
        if (!string.IsNullOrWhiteSpace(configuredPath) && File.Exists(configuredPath))
        {
            return configuredPath;
        }

        foreach (var candidate in LocalExecutableCandidates())
        {
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        var pathExecutable = FindOnPath(ExecutableName);
        if (!string.IsNullOrWhiteSpace(pathExecutable))
        {
            return pathExecutable;
        }

        throw new FileNotFoundException(
            "Unable to locate the Rust httpgenerator executable. Set HTTPGENERATOR_PATH, place httpgenerator.exe next to the VSIX assembly, build the workspace target output, or add httpgenerator.exe to PATH.");
    }

    private static IEnumerable<string> LocalExecutableCandidates()
    {
        var baseDirectory = AppDomain.CurrentDomain.BaseDirectory;
        var repoRoot = Path.GetFullPath(Path.Combine(baseDirectory, "..", "..", "..", ".."));

        yield return Path.Combine(baseDirectory, ExecutableName);
        yield return Path.Combine(baseDirectory, "bin", ExecutableName);
        yield return Path.Combine(repoRoot, "target", "debug", ExecutableName);
        yield return Path.Combine(repoRoot, "target", "release", ExecutableName);
    }

    private static string? FindOnPath(string executableName)
    {
        var pathValue = Environment.GetEnvironmentVariable("PATH");
        if (string.IsNullOrWhiteSpace(pathValue))
        {
            return null;
        }

        foreach (var directory in pathValue
                     .Split(Path.PathSeparator)
                     .Select(segment => segment.Trim().Trim('"'))
                     .Where(segment => !string.IsNullOrWhiteSpace(segment)))
        {
            var candidate = Path.Combine(directory, executableName);
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        return null;
    }

    private static string BuildArguments(HttpGeneratorCliOptions options)
    {
        var arguments = new List<string>
        {
            QuoteArgument(options.OpenApiPath),
            "--output",
            QuoteArgument(options.OutputFolder),
            "--output-type",
            options.OutputType,
            "--base-url",
            QuoteArgument(options.BaseUrl),
            "--content-type",
            QuoteArgument(options.ContentType),
        };

        if (!string.IsNullOrWhiteSpace(options.AuthorizationHeader))
        {
            arguments.Add("--authorization-header");
            arguments.Add(QuoteArgument(options.AuthorizationHeader));
        }

        if (!string.IsNullOrWhiteSpace(options.AzureScope))
        {
            arguments.Add("--azure-scope");
            arguments.Add(QuoteArgument(options.AzureScope));
        }

        if (!string.IsNullOrWhiteSpace(options.AzureTenantId))
        {
            arguments.Add("--azure-tenant-id");
            arguments.Add(QuoteArgument(options.AzureTenantId));
        }

        return string.Join(" ", arguments);
    }

    private static string QuoteArgument(string value)
    {
        return $"\"{value.Replace("\"", "\\\"")}\"";
    }
}
