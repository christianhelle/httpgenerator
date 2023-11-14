using System.Diagnostics;
using HttpGenerator.Core;
using HttpGenerator.Validation;
using Microsoft.OpenApi.Models;
using Spectre.Console;
using Spectre.Console.Cli;

namespace HttpGenerator;

public class GenerateCommand : AsyncCommand<Settings>
{
    private static readonly string Crlf = Environment.NewLine;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        try
        {
            var stopwatch = Stopwatch.StartNew();
            AnsiConsole.MarkupLine($"[green]HTTP File Generator v{GetType().Assembly.GetName().Version!}[/]");
            AnsiConsole.MarkupLine(
                settings.NoLogging
                    ? "[green]Support key: Unavailable when logging is disabled[/]"
                    : $"[green]Support key: {SupportInformation.GetSupportKey()}[/]");
            
            if (!settings.SkipValidation)
                await ValidateOpenApiSpec(settings);
            
            var result = await HttpFileGenerator.Generate(settings.OpenApiPath!);

            if (!string.IsNullOrWhiteSpace(settings.OutputFolder) && !Directory.Exists(settings.OutputFolder))
                Directory.CreateDirectory(settings.OutputFolder);

            foreach (var file in result.Files)
            {
                var outputFile = Path.Combine(settings.OutputFolder!, file.Filename);
                await File.WriteAllTextAsync(outputFile, file.Content);
            }

            AnsiConsole.MarkupLine($"[green]Files: {result.Files.Count}[/]");
            AnsiConsole.MarkupLine($"[green]Duration: {stopwatch.Elapsed}{Crlf}[/]");
            return 0;
        }
        catch (Exception exception)
        {
            if (exception is not OpenApiValidationException)
            {
                AnsiConsole.MarkupLine($"[red]Error:{Crlf}{exception.Message}[/]");
                AnsiConsole.MarkupLine($"[red]Exception:{Crlf}{exception.GetType()}[/]");
                AnsiConsole.MarkupLine($"[yellow]Stack Trace:{Crlf}{exception.StackTrace}[/]");
            }

            await Analytics.LogError(exception, settings);
            return exception.HResult;
        }
    }

    private static async Task ValidateOpenApiSpec(Settings settings)
    {
        var validationResult = await OpenApiValidator.Validate(settings.OpenApiPath!);
        if (!validationResult.IsValid)
        {
            AnsiConsole.MarkupLine($"[red]{Crlf}OpenAPI validation failed:{Crlf}[/]");

            foreach (var error in validationResult.Diagnostics.Errors)
            {
                TryWriteLine(error, "red", "Error");
            }

            foreach (var warning in validationResult.Diagnostics.Warnings)
            {
                TryWriteLine(warning, "yellow", "Warning");
            }

            validationResult.ThrowIfInvalid();
        }

        AnsiConsole.MarkupLine($"[green]{Crlf}OpenAPI statistics:{Crlf}{validationResult.Statistics}{Crlf}[/]");
    }

    private static void TryWriteLine(
        OpenApiError error,
        string color,
        string label)
    {
        try
        {
            AnsiConsole.MarkupLine($"[{color}]{label}:{Crlf}{error}{Crlf}[/]");
        }
        catch
        {
            // ignored
        }
    }
}