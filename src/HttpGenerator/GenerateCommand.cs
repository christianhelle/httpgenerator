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
        if (!settings.NoLogging)
            Analytics.Configure();

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
            
            var generatorSettings = new GeneratorSettings
            {
                AuthorizationHeader = settings.AuthorizationHeader,
                OpenApiPath = settings.OpenApiPath,
                ContentType = settings.ContentType,
                BaseUrl = settings.BaseUrl,
                OutputType = settings.OutputType,
            };

            var result = await HttpFileGenerator.Generate(generatorSettings);
            await Analytics.LogFeatureUsage(settings);

            if (!string.IsNullOrWhiteSpace(settings.OutputFolder) && !Directory.Exists(settings.OutputFolder))
                Directory.CreateDirectory(settings.OutputFolder);

            await Task.WhenAll(
                result.Files.Select(
                    file => File.WriteAllTextAsync(
                        Path.Combine(settings.OutputFolder!, file.Filename),
                        file.Content)));

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