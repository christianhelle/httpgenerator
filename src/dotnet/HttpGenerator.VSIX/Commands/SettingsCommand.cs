using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using System.Windows.Forms;
using HttpGenerator.VSIX.Settings;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
public class SettingsCommand : Command
{
    public override CommandConfiguration CommandConfiguration => new("Settings")
    {
        Placements = [CommandPlacement.KnownPlacements.ExtensionsMenu]
    };

    public override async Task ExecuteCommandAsync(IClientContext context, CancellationToken cancellationToken)
    {
        var provider = new ExtensionSettingsProvider();
        var opts = await provider.GetTelemetryOptionsAsync(cancellationToken);

        using var dlg = new Dialogs.SettingsDialog(opts);
        if (dlg.ShowDialog() == DialogResult.OK)
        {
            await provider.SetTelemetryOptionsAsync(dlg.Options, cancellationToken);
            MessageBox.Show("Settings saved.", "Settings", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }
    }
}
