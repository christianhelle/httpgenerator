using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using Microsoft.VisualStudio.Extensibility.Shell;
using Microsoft.VisualStudio.RpcContracts.ProgressReporting;
using System.Diagnostics;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
public sealed class GenerateHttpCommand(TraceSource traceSource) : Command
{
    public override CommandConfiguration CommandConfiguration => new("%GenerateHttpCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.GenerateFile, IconSettings.IconAndText),
        VisibleWhen = ActivationConstraint.ClientContext(
            ClientContextKey.Shell.ActiveSelectionFileName,
            ".(json|ya?ml)"),
    };

    public override async Task ExecuteCommandAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        using var progress = await Extensibility
            .Shell()
            .StartProgressReportingAsync(
                "Generating .http files",
                new ProgressReporterOptions(true), // true = indeterminate/cancellable
                cancellationToken);

        var selectedPath = await context.GetSelectedPathAsync(cancellationToken);

        if (string.IsNullOrEmpty(selectedPath))
        {
            return;
        }

        var directory = Path.GetDirectoryName(selectedPath) ?? string.Empty;
        var outputFolder = Path.Combine(directory, "HttpFiles");

        try
        {
            if (!Directory.Exists(outputFolder))
            {
                Directory.CreateDirectory(outputFolder);
            }

            progress.Report(new(10, $"Starting code generation"));
            var generateResult = await HttpGeneratorCli.ExecuteAsync(
                selectedPath,
                outputFolder,
                baseUrl: null,
                contentType: "application/json",
                authorizationHeader: null,
                generateMultipleFiles: true,
                progress,
                cancellationToken).ConfigureAwait(false);

            if (generateResult.Success && generateResult.FileCount > 0)
            {
                traceSource.TraceEvent(
                    TraceEventType.Information,
                    0,
                    "Successfully generated {0} file(s) in {1}",
                    generateResult.FileCount,
                    outputFolder);
            }
            else
            {
                traceSource.TraceEvent(
                    TraceEventType.Warning,
                    0,
                    "Generation completed but could not verify result in {0}",
                    outputFolder);
            }
        }
        catch (Exception ex)
        {
            traceSource.TraceEvent(
                TraceEventType.Error,
                0,
                "Error generating .http files: {0}",
                ex.Message);

            var shouldOfferDownload = ex.Message.Contains("httpgenerator") ||
                                      ex.Message.Contains("install") ||
                                      ex.Message.Contains("download");

            if (shouldOfferDownload)
            {
                try
                {
                    var psi = new ProcessStartInfo
                    {
                        FileName = "powershell",
                        Arguments = "-NoProfile -Command \"irm https://christianhelle.com/httpgenerator/install.ps1 | iex\"",
                        UseShellExecute = true,
                    };
                    Process.Start(psi);
                }
                catch
                {
                    // Ignore
                }
            }
        }
    }
}
