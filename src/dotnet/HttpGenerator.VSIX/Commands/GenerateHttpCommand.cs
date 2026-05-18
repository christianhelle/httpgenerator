using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
internal sealed class GenerateHttpCommand(
    BackgroundGenerationCoordinator coordinator,
    HttpGeneratorSettingsProvider settingsProvider) : Command
{
    public override CommandConfiguration CommandConfiguration => new("%GenerateHttpCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.GenerateFile, IconSettings.IconAndText),
        VisibleWhen = ActivationConstraint.ClientContext(
            ClientContextKey.Shell.ActiveSelectionFileName,
            "\\.(json|ya?ml)$"),
        Placements =
        [
            CommandPlacement.VsctParent(KnownVsctIds.ShellMainMenu, KnownVsctIds.SolutionExplorerFileContextMenu, priority: 0x0100),
        ],
    };

    public override async Task ExecuteCommandAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        var selectedPath = await context.TryGetSelectedOpenApiPathAsync(cancellationToken);
        if (selectedPath is null)
        {
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
