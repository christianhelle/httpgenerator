using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using Community.VisualStudio.Toolkit;
using Microsoft.VisualStudio.Shell;
using System.IO;

namespace HttpGenerator.VSIX.Commands;

[VisualStudioContribution]
public class GenerateTestPlansCommandExt : Command
{
    public override CommandConfiguration CommandConfiguration => new("Generate HTTP files")
    {
        Placements = [CommandPlacement.KnownPlacements.ProjectNode]
    };

    public override async Task ExecuteCommandAsync(IClientContext context, CancellationToken cancellationToken)
    {
        // Try to get active project via Community Toolkit (existing helper)
        try
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync(cancellationToken);
        }
        catch
        {
            // ignore
        }

        var project = await VS.Solutions.GetActiveProjectAsync();
        if (project is null)
        {
            return;
        }

        var output = Path.GetDirectoryName(project.FullPath);
        if (string.IsNullOrEmpty(output))
            output = project.FullPath ?? string.Empty;

        output = Path.Combine(output, "HttpFiles");

        using var dialog = new GenerateDialog(output);
        dialog.ShowDialog();
    }
}
