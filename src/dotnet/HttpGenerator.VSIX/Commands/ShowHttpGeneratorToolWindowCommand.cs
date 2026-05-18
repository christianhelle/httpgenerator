using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
internal sealed class ShowHttpGeneratorToolWindowCommand : Command
{
    public override CommandConfiguration CommandConfiguration => new("%ShowHttpGeneratorToolWindowCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.ToolWindow, IconSettings.IconAndText),
    };

    public override async Task ExecuteCommandAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        await Extensibility.Shell().ShowToolWindowAsync<HttpGeneratorToolWindow>(activate: true, cancellationToken);
    }
}
