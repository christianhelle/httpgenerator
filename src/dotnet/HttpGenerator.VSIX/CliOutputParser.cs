using System;
using System.Collections.Generic;
using System.IO;

namespace HttpGenerator.VSIX;

internal class GenerateResult
{
    public bool Success { get; }
    public int FileCount { get; }
    public IReadOnlyList<string> Files { get; }

    private GenerateResult(bool success, int fileCount, IReadOnlyList<string> files)
    {
        Success = success;
        FileCount = fileCount;
        Files = files;
    }

    public static GenerateResult SuccessResult(int fileCount, IReadOnlyList<string> files)
    {
        return new GenerateResult(true, fileCount, files);
    }

    public static GenerateResult UnknownResult()
    {
        return new GenerateResult(false, 0, Array.Empty<string>());
    }
}

internal static class CliOutputParser
{
    private const string OutputSectionMarker = "Files written successfully:";

    public static GenerateResult ParseOutput(string stdout, string outputFolder)
    {
        var lines = stdout.Split(new[] { '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries);
        var fileLines = new List<string>();
        var inOutputSection = false;

        for (var i = 0; i < lines.Length; i++)
        {
            var line = lines[i].Trim();

            if (line.Contains(OutputSectionMarker))
            {
                inOutputSection = true;
                continue;
            }

            if (inOutputSection)
            {
                if (string.IsNullOrWhiteSpace(line) || line.StartsWith("---") || line.StartsWith("╭") || line.StartsWith("╰"))
                {
                    inOutputSection = false;
                    continue;
                }

                var trimmed = line.TrimStart().TrimEnd();
                if (!string.IsNullOrWhiteSpace(trimmed))
                {
                    fileLines.Add(trimmed);
                }
            }
        }

        if (fileLines.Count > 0)
        {
            return GenerateResult.SuccessResult(fileLines.Count, fileLines);
        }

        return GenerateResult.UnknownResult();
    }

    public static string ParseErrorOutput(string stderr)
    {
        return string.IsNullOrWhiteSpace(stderr)
            ? "No error details available."
            : stderr.Trim();
    }
}
