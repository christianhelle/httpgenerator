using System.Collections.Concurrent;
using System.Diagnostics;

using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Shell;
using Microsoft.VisualStudio.RpcContracts.ProgressReporting;

namespace HttpGenerator.VSIX;

internal sealed class BackgroundGenerationCoordinator(
    TraceSource traceSource,
    HttpGeneratorToolWindowState toolWindowState) : IDisposable
{
    private readonly ConcurrentDictionary<string, CancellationTokenSource> activeJobs = new(StringComparer.OrdinalIgnoreCase);

    public async Task QueueAsync(
        HttpGenerationRequest request,
        VisualStudioExtensibility extensibility,
        CancellationToken cancellationToken)
    {
        var jobCancellation = new CancellationTokenSource();
        if (!activeJobs.TryAdd(request.OpenApiPath, jobCancellation))
        {
            jobCancellation.Dispose();
            traceSource.TraceEvent(
                TraceEventType.Information,
                0,
                "Generation already in progress for {0}",
                request.OpenApiPath);

            toolWindowState.RecordDuplicate(request, activeJobs.Count);
            GenerationNotificationService.ShowDuplicate(extensibility, traceSource, request);
            return;
        }

        toolWindowState.RecordQueued(request, activeJobs.Count);
        _ = RunGenerationAsync(request, extensibility, jobCancellation);
    }

    private async Task RunGenerationAsync(
        HttpGenerationRequest request,
        VisualStudioExtensibility extensibility,
        CancellationTokenSource jobCancellation)
    {
        try
        {
            using var progress = await extensibility
                .Shell()
                .StartProgressReportingAsync(
                    $"Generating .http files for {request.DisplayName}",
                    new ProgressReporterOptions(true),
                    CancellationToken.None);

            using var linkedCancellation = CancellationTokenSource.CreateLinkedTokenSource(
                progress.CancellationToken,
                jobCancellation.Token);

            toolWindowState.RecordStarted(request, activeJobs.Count);

            var result = await HttpGeneratorCli.ExecuteAsync(
                request.OpenApiPath,
                request.OutputFolder,
                request.Settings.BaseUrl,
                request.Settings.ContentType,
                authorizationHeader: null,
                request.Settings.GenerateMultipleFiles,
                progress,
                linkedCancellation.Token).ConfigureAwait(false);

            if (result.Cancelled)
            {
                traceSource.TraceEvent(
                    TraceEventType.Information,
                    0,
                    "Generation cancelled for {0}",
                    request.OpenApiPath);

                toolWindowState.RecordCancelled(request, result, activeJobs.Count - 1);
                return;
            }

            traceSource.TraceEvent(
                TraceEventType.Information,
                0,
                "Successfully generated {0} file(s) for {1}",
                result.FileCount,
                request.OpenApiPath);

            toolWindowState.RecordCompleted(request, result, activeJobs.Count - 1);
            GenerationNotificationService.ShowSuccess(extensibility, traceSource, request, result);
        }
        catch (HttpGeneratorCliException ex)
        {
            traceSource.TraceEvent(
                TraceEventType.Error,
                0,
                "Error generating .http files for {0}: {1}",
                request.OpenApiPath,
                ex.Result.Summary);

            toolWindowState.RecordFailure(request, ex.Result, activeJobs.Count - 1);
            GenerationNotificationService.ShowFailure(extensibility, traceSource, request, ex.Result);
        }
        catch (Exception ex)
        {
            traceSource.TraceEvent(
                TraceEventType.Error,
                0,
                "Error generating .http files for {0}: {1}",
                request.OpenApiPath,
                ex.Message);

            toolWindowState.RecordFailure(request, ex, activeJobs.Count - 1);
            GenerationNotificationService.ShowFailure(extensibility, traceSource, request);
        }
        finally
        {
            if (activeJobs.TryRemove(request.OpenApiPath, out var cancellationSource))
            {
                cancellationSource.Dispose();
            }

            toolWindowState.UpdateActiveJobCount(activeJobs.Count);
        }
    }

    public void Dispose()
    {
        foreach (var cancellationSource in activeJobs.Values)
        {
            try
            {
                cancellationSource.Cancel();
            }
            catch
            {
                // Best effort during extension shutdown.
            }
        }
    }
}
