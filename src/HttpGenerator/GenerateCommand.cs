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
            DisplayHeader(settings);

            if (!settings.SkipValidation) await ValidateOpenApiSpec(settings);

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
                SkipHeaders = settings.SkipHeaders,
            };
            GeneratorResult result = await HttpFileGenerator.Generate(generatorSettings);
            await Analytics.LogFeatureUsage(settings);
            await WriteFiles(settings, result);

            DisplaySuccess(stopwatch.Elapsed);
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
                $"This tool uses Microsoft.OpenApi libraries for parsing and validation.{Crlf}{Crlf}[/]");
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
        DisplayFileWritingHeader(result.Files.Count);

        if (!string.IsNullOrWhiteSpace(settings.OutputFolder) && !Directory.Exists(settings.OutputFolder))
            Directory.CreateDirectory(settings.OutputFolder);

        var timeout = Task.Delay(TimeSpan.FromSeconds(settings.Timeout));
        var writeFiles = Task.WhenAll(
            result.Files.Select(file => File.WriteAllTextAsync(
                Path.Combine(settings.OutputFolder, file.Filename),
                file.Content)));

        if (timeout == await Task.WhenAny(timeout, writeFiles))
        {
            AnsiConsole.MarkupLine($"{Crlf}[red]❌ Operation timed out[/]");
        }
        else
        {
            DisplayFilesWritten(result.Files, settings.OutputFolder);
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
            AnsiConsole.MarkupLine($"[cyan]🔐 Acquiring authorization header from Azure Entra ID...[/]");
            using var listener = AzureEventSourceListener.CreateConsoleLogger();
            var token = await AzureEntraID
                .TryGetAccessTokenAsync(
                    settings.AzureTenantId!,
                    settings.AzureScope!,
                    CancellationToken.None);

            if (!string.IsNullOrWhiteSpace(token))
            {
                settings.AuthorizationHeader = $"Bearer {token}";
                AnsiConsole.MarkupLine($"[green]✅ Successfully acquired access token[/]{Crlf}");
            }
        }
        catch (Exception exception)
        {
            AnsiConsole.MarkupLine($"{Crlf}[red]Error:{Crlf}{exception.Message}[/]");
        }
    }

    private static async Task ValidateOpenApiSpec(Settings settings)
    {
        AnsiConsole.MarkupLine("[cyan]🔍 Validating OpenAPI specification...[/]");
        var validationResult = await OpenApiValidator.Validate(settings.OpenApiPath!);
        WriteValidationResults(validationResult);
        validationResult.ThrowIfInvalid();

        DisplayOpenApiStatistics(validationResult.Statistics);
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

    [ExcludeFromCodeCoverage]
    private static void DisplayHeader(Settings settings)
    {
        var version = typeof(GenerateCommand).Assembly.GetName().Version!;

        // Create a panel with the application header
        var panel = new Panel(new Markup($"[bold blue]🚀 HTTP File Generator[/] [dim]v{version}[/]"))
        {
            Border = BoxBorder.Rounded,
            BorderStyle = new Style(Color.Blue),
            Padding = new Padding(1, 0, 1, 0)
        };

        AnsiConsole.Write(panel);

        // Support key information
        var supportKey = settings.NoLogging
            ? "[yellow]⚠️  Unavailable when logging is disabled[/]"
            : $"[green]🔑 {SupportInformation.GetSupportKey()}[/]";

        AnsiConsole.MarkupLine($"Support key: {supportKey}");
        AnsiConsole.WriteLine();
    }

    [ExcludeFromCodeCoverage]
    private static void DisplayOpenApiStatistics(OpenApiStats statistics)
    {
        AnsiConsole.MarkupLine("[green]✅ OpenAPI specification validated successfully[/]");

        var table = new Table()
        {
            Border = TableBorder.Rounded,
            BorderStyle = new Style(Color.Green)
        };

        table.AddColumn(new TableColumn("[bold]📊 OpenAPI Statistics[/]").LeftAligned());
        table.AddColumn(new TableColumn("[bold]Count[/]").LeftAligned());

        table.AddRow("📝 Path Items", $"[cyan]{statistics.PathItemCount}[/]");
        table.AddRow("⚡ Operations", $"[cyan]{statistics.OperationCount}[/]");
        table.AddRow("📝 Parameters", $"[cyan]{statistics.ParameterCount}[/]");
        table.AddRow("📤 Request Bodies", $"[cyan]{statistics.RequestBodyCount}[/]");
        table.AddRow("📥 Responses", $"[cyan]{statistics.ResponseCount}[/]");
        table.AddRow("🔗 Links", $"[cyan]{statistics.LinkCount}[/]");
        table.AddRow("📞 Callbacks", $"[cyan]{statistics.CallbackCount}[/]");
        table.AddRow("📋 Schemas", $"[cyan]{statistics.SchemaCount}[/]");

        AnsiConsole.Write(table);
        AnsiConsole.WriteLine();
    }

    [ExcludeFromCodeCoverage]
    private static void DisplayFileWritingHeader(int fileCount)
    {
        var rule = new Rule($"[yellow]📁 Writing {fileCount} file(s)[/]")
        {
            Style = Style.Parse("yellow"),
            Justification = Justify.Left
        };
        AnsiConsole.Write(rule);
    }

    [ExcludeFromCodeCoverage]
    private static void DisplayFilesWritten(IReadOnlyCollection<HttpFile> files, string outputFolder)
    {
        AnsiConsole.MarkupLine("[green]✅ Files written successfully:[/]");

        foreach (var file in files)
        {
            var fullPath = Path.Combine(outputFolder, file.Filename);
            AnsiConsole.MarkupLine($"   [dim]📄[/] [link]{fullPath}[/]");
        }

        AnsiConsole.WriteLine();
    }

    [ExcludeFromCodeCoverage]
    private static void DisplaySuccess(TimeSpan duration)
    {
        var successPanel = new Panel(new Markup($"[bold green]🎉 Generation completed successfully![/]"))
        {
            Border = BoxBorder.Rounded,
            BorderStyle = new Style(Color.Green),
            Padding = new Padding(1, 0, 1, 0)
        };

        AnsiConsole.Write(successPanel);
        AnsiConsole.MarkupLine($"[dim]⏱️  Duration: {duration:mm\\:ss\\.fff}[/]");
        AnsiConsole.WriteLine();
    }
}
