using System.Text;

namespace HttpGenerator.VSIX;

internal sealed class GenerateResult
{
    public bool Success { get; init; }
    public bool Cancelled { get; init; }
    public int ExitCode { get; init; }
    public int FileCount => Files.Count;
    public string OutputFolder { get; init; } = string.Empty;
    public string ExecutablePath { get; init; } = string.Empty;
    public IReadOnlyList<string> Files { get; init; } = Array.Empty<string>();
    public string Summary { get; init; } = string.Empty;
    public string Details { get; init; } = string.Empty;
    public string StandardOutput { get; init; } = string.Empty;
    public string StandardError { get; init; } = string.Empty;
}

internal static class CliOutputParser
{
    private const string OutputSectionMarker = "Files written successfully:";

    public static GenerateResult CreateSuccess(
        string stdout,
        string stderr,
        string outputFolder,
        string executablePath)
    {
        var files = ParseFiles(stdout);
        var summary = files.Count > 0
            ? $"Generated {files.Count} .http file(s)."
            : "Generation completed. Output parsing was inconclusive.";

        return new GenerateResult
        {
            Success = true,
            ExitCode = 0,
            OutputFolder = outputFolder,
            ExecutablePath = executablePath,
            Files = files,
            Summary = summary,
            Details = BuildDetails(
                summary,
                executablePath,
                outputFolder,
                stdout,
                stderr,
                files),
            StandardOutput = stdout,
            StandardError = stderr,
        };
    }

    public static GenerateResult CreateFailure(
        int exitCode,
        string stdout,
        string stderr,
        string outputFolder,
        string executablePath)
    {
        var summary = $"httpgenerator exited with code {exitCode}.";

        return new GenerateResult
        {
            Success = false,
            ExitCode = exitCode,
            OutputFolder = outputFolder,
            ExecutablePath = executablePath,
            Files = ParseFiles(stdout),
            Summary = summary,
            Details = BuildDetails(
                summary,
                executablePath,
                outputFolder,
                stdout,
                stderr),
            StandardOutput = stdout,
            StandardError = stderr,
        };
    }

    public static GenerateResult CreateCancelled(
        string stdout,
        string stderr,
        string outputFolder,
        string executablePath)
    {
        const string summary = "Generation was cancelled.";

        return new GenerateResult
        {
            Success = false,
            Cancelled = true,
            ExitCode = -1,
            OutputFolder = outputFolder,
            ExecutablePath = executablePath,
            Files = ParseFiles(stdout),
            Summary = summary,
            Details = BuildDetails(
                summary,
                executablePath,
                outputFolder,
                stdout,
                stderr),
            StandardOutput = stdout,
            StandardError = stderr,
        };
    }

    private static IReadOnlyList<string> ParseFiles(string stdout)
    {
        var lines = stdout.Replace("\r\n", "\n", StringComparison.Ordinal).Split('\n');
        var fileLines = new List<string>();
        var inOutputSection = false;

        foreach (var rawLine in lines)
        {
            var line = rawLine.Trim();

            if (line.Contains(OutputSectionMarker, StringComparison.Ordinal))
            {
                inOutputSection = true;
                continue;
            }

            if (!inOutputSection)
            {
                continue;
            }

            if (string.IsNullOrWhiteSpace(line)
                || line.StartsWith("Generation completed successfully!", StringComparison.Ordinal)
                || line.StartsWith("Duration:", StringComparison.Ordinal)
                || line.StartsWith("---", StringComparison.Ordinal)
                || line.StartsWith("╭", StringComparison.Ordinal)
                || line.StartsWith("╰", StringComparison.Ordinal))
            {
                inOutputSection = false;
                continue;
            }

            fileLines.Add(line);
        }

        return fileLines;
    }

    private static string BuildDetails(
        string summary,
        string executablePath,
        string outputFolder,
        string stdout,
        string stderr,
        IReadOnlyList<string>? files = null)
    {
        var builder = new StringBuilder();
        builder.AppendLine(summary);
        builder.AppendLine();
        builder.AppendLine($"Executable: {executablePath}");
        builder.AppendLine($"Output folder: {outputFolder}");

        if (files is { Count: > 0 })
        {
            builder.AppendLine();
            builder.AppendLine("Generated files:");

            foreach (var file in files)
            {
                builder.AppendLine(file);
            }
        }

        if (!string.IsNullOrWhiteSpace(stderr))
        {
            builder.AppendLine();
            builder.AppendLine("stderr:");
            builder.AppendLine(stderr.Trim());
        }

        if (!string.IsNullOrWhiteSpace(stdout))
        {
            builder.AppendLine();
            builder.AppendLine("stdout:");
            builder.AppendLine(stdout.Trim());
        }

        return builder.ToString().TrimEnd();
    }
}
