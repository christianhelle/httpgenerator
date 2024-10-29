using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using Azure.Core.Diagnostics;
using HttpGenerator.Core;
using HttpGenerator.Validation;
using Microsoft.OpenApi.Readers.Exceptions;
using Spectre.Console;
using Spectre.Console.Cli;

namespace HttpGenerator;

public class GenerateCommand : AsyncCommand<Settings>
{
    private static readonly string Crlf = Environment.NewLine;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        Analytics.Configure(settings);

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

            await AcquireAzureEntraIdToken(settings);

            var generatorSettings = new GeneratorSettings
            {
                AuthorizationHeader = settings.AuthorizationHeader,
                AuthorizationHeaderVariableName = settings.AuthorizationHeaderVariableName,
                AuthorizationHeaderFromEnvironmentVariable = settings.AuthorizationHeaderFromEnvironmentVariable,
                OpenApiPath = settings.OpenApiPath,
                ContentType = settings.ContentType,
                BaseUrl = settings.BaseUrl,
                OutputType = settings.OutputType,
                Timeout = settings.Timeout,
                GenerateIntelliJTests = settings.GenerateIntelliJTests,
                CustomHeaders = settings.CustomHeaders,
            };

            var result = await HttpFileGenerator.Generate(generatorSettings);
            await Analytics.LogFeatureUsage(settings);
            await WriteFiles(settings, result);

            AnsiConsole.MarkupLine($"[green]Duration: {stopwatch.Elapsed}{Crlf}[/]");
            return 0;
        }
        catch (OpenApiUnsupportedSpecVersionException exception)
        {
            AnsiConsole.MarkupLine($"{Crlf}[red]Error:{Crlf}{exception.Message}[/]");
            AnsiConsole.MarkupLine(
                $"{Crlf}[yellow]Tips:{Crlf}" +
                $"Consider using the --skip-validation argument.{Crlf}" +
                $"In some cases, the features that are specific to the " +
                $"unsupported versions of OpenAPI specifications aren't really used.{Crlf}" +
                $"This tool uses NSwag libraries to parse the OpenAPI document and " +
                $"Microsoft.OpenApi libraries for validation.{Crlf}{Crlf}[/]");
            return exception.HResult;
        }
        catch (Exception exception)
        {
            if (exception is not OpenApiValidationException)
            {
                TryWriteLine(exception.Message, "red", "Error");
                TryWriteLine(exception.GetType(), "red", "Exception");

                if (exception.StackTrace != null)
                    TryWriteLine(exception.StackTrace, "yellow", "Stack Trace");

                if (exception.InnerException != null)
                {
                    TryWriteLine(exception.InnerException.Message, "red", "Inner Error");
                    TryWriteLine(exception.InnerException.GetType(), "red", "Inner Exception");
                    if (exception.InnerException.StackTrace != null)
                        TryWriteLine(exception.InnerException.StackTrace, "yellow", "Inner Stack Trace");
                }
            }

            await Analytics.LogError(exception, settings);
            return exception.HResult;
        }
    }

    [ExcludeFromCodeCoverage]
    private static async Task WriteFiles(Settings settings, GeneratorResult result)
    {
        AnsiConsole.MarkupLine($"[green]Writing {result.Files.Count} file(s)[/]");

        if (!string.IsNullOrWhiteSpace(settings.OutputFolder) && !Directory.Exists(settings.OutputFolder))
            Directory.CreateDirectory(settings.OutputFolder);
            
        var timeout = Task.Delay(TimeSpan.FromSeconds(settings.Timeout));
        var writeFiles = Task.WhenAll(
            result.Files.Select(
                file => File.WriteAllTextAsync(
                    Path.Combine(settings.OutputFolder, file.Filename),
                    file.Content)));

        if (timeout == await Task.WhenAny(timeout, writeFiles))
        {
            AnsiConsole.MarkupLine($"[red]Operation timed out :([/]");
        }
    }

    private static async Task AcquireAzureEntraIdToken(Settings settings)
    {
        if (!string.IsNullOrWhiteSpace(settings.AuthorizationHeader) ||
            (string.IsNullOrWhiteSpace(settings.AzureScope) &&
             string.IsNullOrWhiteSpace(settings.AzureTenantId)))
        {
            return;
        }

        try
        {
            AnsiConsole.MarkupLine($"[green]Acquiring authorization header from Azure Entra ID[/]{Crlf}");
            using var listener = AzureEventSourceListener.CreateConsoleLogger();
            var token = await AzureEntraID
                .TryGetAccessTokenAsync(
                    settings.AzureTenantId!,
                    settings.AzureScope!,
                    CancellationToken.None);

            if (!string.IsNullOrWhiteSpace(token))
            {
                settings.AuthorizationHeader = $"Bearer {token}";
                AnsiConsole.MarkupLine($"{Crlf}[green]Successfully acquired access token[/]{Crlf}");
            }
        }
        catch (Exception exception)
        {
            AnsiConsole.MarkupLine($"{Crlf}[red]Error:{Crlf}{exception.Message}[/]");
        }
    }

    private static async Task ValidateOpenApiSpec(Settings settings)
    {
        var validationResult = await OpenApiValidator.Validate(settings.OpenApiPath!);
        WriteValidationResults(validationResult);
        validationResult.ThrowIfInvalid();

        AnsiConsole.MarkupLine($"[green]{Crlf}OpenAPI statistics:{Crlf}{validationResult.Statistics}{Crlf}[/]");
    }

    [ExcludeFromCodeCoverage]
    private static void WriteValidationResults(OpenApiValidationResult validationResult)
    {
        if (validationResult.IsValid)
            return;

        AnsiConsole.MarkupLine($"[red]{Crlf}OpenAPI validation failed:{Crlf}[/]");

        foreach (var error in validationResult.Diagnostics.Errors)
        {
            TryWriteLine(error, "red", "Error");
        }

        foreach (var warning in validationResult.Diagnostics.Warnings)
        {
            TryWriteLine(warning, "yellow", "Warning");
        }
    }

    private static void TryWriteLine(
        object error,
        string color,
        string label)
    {
        try
        {
            AnsiConsole.MarkupLine($"[{color}]{label}:{Crlf}{error}{Crlf}[/]");
        }
        catch
        {
            var originalColor = Console.ForegroundColor;
            Console.ForegroundColor = color switch
            {
                "red" => ConsoleColor.Red,
                "yellow" => ConsoleColor.Yellow,
                _ => originalColor
            };

            Console.WriteLine($"{label}:{Crlf}{error}{Crlf}");

            Console.ForegroundColor = originalColor;
        }
    }
}
