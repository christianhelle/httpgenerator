using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
internal sealed class GenerateHttpFallbackCommand(
    BackgroundGenerationCoordinator coordinator,
    HttpGeneratorSettingsProvider settingsProvider,
    HttpGeneratorToolWindowState toolWindowState) : Command
{
    public override CommandConfiguration CommandConfiguration => new("%GenerateHttpCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.GenerateFile, IconSettings.IconAndText),
    };

    public override async Task ExecuteCommandAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        var selectedPath = await context.TryGetSelectedOpenApiPathAsync(cancellationToken);
        if (selectedPath is null)
        {
            toolWindowState.RecordSelectionRequired();
            await Extensibility.Shell().ShowToolWindowAsync<HttpGeneratorToolWindow>(activate: true, cancellationToken);
            return;
        }

        var settings = await settingsProvider.GetSnapshotAsync(cancellationToken);
        var request = HttpGenerationRequest.Create(selectedPath, settings);

        await coordinator.QueueAsync(
            request,
            Extensibility,
            cancellationToken);
    }
}
