using System.Threading;
using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
public sealed class GenerateHttpFilesCommand : Command
{
    public override CommandConfiguration CommandConfiguration => new("%HttpGenerator.GenerateCommand.DisplayName%")
    {
        Icon = new(ImageMoniker.KnownValues.GenerateFile, IconSettings.IconAndText),
    };

    public override async Task ExecuteCommandAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        var outputFolder = await GetDefaultOutputFolderAsync(context, cancellationToken);
        await GenerateDialogHost.ShowDialogAsync(outputFolder, cancellationToken);
    }

    private static async Task<string> GetDefaultOutputFolderAsync(
        IClientContext context,
        CancellationToken cancellationToken)
    {
        var project = await context.GetActiveProjectAsync(cancellationToken);
        var projectPath = project?.Path;
        var projectDirectory = !string.IsNullOrWhiteSpace(projectPath)
            ? Path.GetDirectoryName(projectPath) ?? projectPath
            : null;

        if (string.IsNullOrWhiteSpace(projectDirectory))
        {
            projectDirectory = Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments);
        }

        return Path.Combine(projectDirectory, "HttpFiles");
    }
}
