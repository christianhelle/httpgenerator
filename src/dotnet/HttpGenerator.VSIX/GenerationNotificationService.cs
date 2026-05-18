using System.Diagnostics;

using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Shell;

namespace HttpGenerator.VSIX;

internal static class GenerationNotificationService
{
    public static void ShowDuplicate(
        VisualStudioExtensibility extensibility,
        TraceSource traceSource,
        HttpGenerationRequest request)
    {
        Show(
            extensibility,
            traceSource,
            $"Generation is already in progress for {request.DisplayName}.",
            PromptOptions.WarningConfirm.Icon,
            ("Open Activity", NotificationAction.OpenActivity));
    }

    public static void ShowSuccess(
        VisualStudioExtensibility extensibility,
        TraceSource traceSource,
        HttpGenerationRequest request,
        GenerateResult result)
    {
        Show(
            extensibility,
            traceSource,
            $"{result.Summary} ({request.DisplayName})",
            PromptOptions.InformationConfirm.Icon,
            ("Open Folder", NotificationAction.OpenFolder),
            result.OutputFolder);
    }

    public static void ShowFailure(
        VisualStudioExtensibility extensibility,
        TraceSource traceSource,
        HttpGenerationRequest request,
        GenerateResult result)
    {
        Show(
            extensibility,
            traceSource,
            $"{result.Summary} ({request.DisplayName})",
            PromptOptions.ErrorConfirm.Icon,
            ("View Details", NotificationAction.OpenActivity));
    }

    public static void ShowFailure(
        VisualStudioExtensibility extensibility,
        TraceSource traceSource,
        HttpGenerationRequest request)
    {
        Show(
            extensibility,
            traceSource,
            $"Generation failed for {request.DisplayName}.",
            PromptOptions.ErrorConfirm.Icon,
            ("View Details", NotificationAction.OpenActivity));
    }

    private static void Show(
        VisualStudioExtensibility extensibility,
        TraceSource traceSource,
        string message,
        ImageMoniker? icon,
        (string Text, NotificationAction Action) action,
        string? outputFolder = null)
    {
        FireAndForget(
            ShowCoreAsync(extensibility, message, icon, action, outputFolder),
            traceSource,
            message);
    }

    private static async Task ShowCoreAsync(
        VisualStudioExtensibility extensibility,
        string message,
        ImageMoniker? icon,
        (string Text, NotificationAction Action) action,
        string? outputFolder)
    {
        var choices = new ChoiceResultCollection<NotificationAction>();
        choices.Add(action.Text, action.Action);

        var options = new PromptOptions<NotificationAction>(
            choices,
            defaultChoiceIndex: 0,
            dismissedReturns: NotificationAction.None)
        {
            Title = "HTTP File Generator",
            Icon = icon ?? ImageMoniker.KnownValues.StatusInformation,
        };

        var selectedAction = await extensibility
            .Shell()
            .ShowPromptAsync(message, options, CancellationToken.None)
            .ConfigureAwait(false);

        switch (selectedAction)
        {
            case NotificationAction.OpenFolder when !string.IsNullOrWhiteSpace(outputFolder):
                FolderLauncher.Open(outputFolder);
                break;

            case NotificationAction.OpenActivity:
                await extensibility.Shell().ShowToolWindowAsync<HttpGeneratorToolWindow>(activate: true, CancellationToken.None);
                break;
        }
    }

    private static void FireAndForget(Task task, TraceSource traceSource, string context)
    {
        _ = task.ContinueWith(
            completedTask =>
            {
                if (completedTask.Exception is null)
                {
                    return;
                }

                traceSource.TraceEvent(
                    TraceEventType.Warning,
                    0,
                    "Failed to show notification '{0}': {1}",
                    context,
                    completedTask.Exception.GetBaseException().Message);
            },
            CancellationToken.None,
            TaskContinuationOptions.OnlyOnFaulted,
            TaskScheduler.Default);
    }

    private enum NotificationAction
    {
        None,
        OpenFolder,
        OpenActivity,
    }
}
