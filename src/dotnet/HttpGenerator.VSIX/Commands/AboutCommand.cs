using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using System.Windows.Forms;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
public class AboutCommand : Command
{
    public override CommandConfiguration CommandConfiguration => new("%AboutCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.AboutBox, IconSettings.IconAndText),
        Placements = [CommandPlacement.KnownPlacements.ExtensionsMenu]
    };

    public override async Task ExecuteCommandAsync(IClientContext context, CancellationToken cancellationToken)
    {
        var version = typeof(AboutCommand).Assembly.GetName().Version?.ToString() ?? "1.0.0";
        var message = $"HTTP File Generator\nVersion: {version}\n\nGenerate .http files from OpenAPI (Swagger) specifications.\n\nRepository: https://github.com/christianhelle/httpgenerator";
        MessageBox.Show(message, "About HTTP File Generator", MessageBoxButtons.OK, MessageBoxIcon.Information);
    }
}
