using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using Community.VisualStudio.Toolkit;
using Microsoft.VisualStudio.Shell;
using System.Net.Http;
using System.IO;
using System.Windows.Forms;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
public class AddNewOpenApiCommand : Command
{
    public override CommandConfiguration CommandConfiguration => new("%AddNewCommand.GroupDisplayName%")
    {
        Placements = [CommandPlacement.KnownPlacements.ProjectNode_AddGroup_Submenu_ItemsGroup]
    };

    public override async Task ExecuteCommandAsync(IClientContext context, CancellationToken cancellationToken)
    {
        await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync(cancellationToken);
        var project = await VS.Solutions.GetActiveProjectAsync();
        if (project is null)
        {
            MessageBox.Show("No active project found.", "Add New HTTP File", MessageBoxButtons.OK, MessageBoxIcon.Warning);
            return;
        }

        using var dlg = new Dialogs.InputUrlDialog();
        if (dlg.ShowDialog() != DialogResult.OK)
            return;

        var url = dlg.Url?.Trim();
        if (string.IsNullOrWhiteSpace(url))
            return;

        if (!Uri.TryCreate(url, UriKind.Absolute, out var uri))
        {
            MessageBox.Show("Invalid URL.", "Add New HTTP File", MessageBoxButtons.OK, MessageBoxIcon.Error);
            return;
        }

        var directory = Path.GetDirectoryName(project.FullPath) ?? project.FullPath ?? string.Empty;
        if (string.IsNullOrEmpty(directory) || !Directory.Exists(directory))
        {
            MessageBox.Show("Project directory not found.", "Add New HTTP File", MessageBoxButtons.OK, MessageBoxIcon.Error);
            return;
        }

        var fileName = uri.GetComponents(UriComponents.Path, UriFormat.Unescaped)
            .Split('/', StringSplitOptions.RemoveEmptyEntries)
            .LastOrDefault() ?? "openapi.json";

        var destinationPath = Path.Combine(directory, fileName);

        try
        {
            using var http = new HttpClient { Timeout = TimeSpan.FromSeconds(30) };
            var content = await http.GetStringAsync(uri, cancellationToken);
            await File.WriteAllTextAsync(destinationPath, content, cancellationToken);

            // Open GenerateDialog for the new file
            using var genDlg = new GenerateDialog(destinationPath);
            genDlg.ShowDialog();
        }
        catch (Exception ex)
        {
            MessageBox.Show($"Failed to download or save file: {ex.Message}", "Add New HTTP File", MessageBoxButtons.OK, MessageBoxIcon.Error);
        }
    }
}
