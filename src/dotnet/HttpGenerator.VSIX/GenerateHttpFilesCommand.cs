using EnvDTE;
using EnvDTE80;
using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
using Microsoft.VisualStudio.Extensibility.VSSdkCompatibility;
using Microsoft.VisualStudio.Shell;
using Command = Microsoft.VisualStudio.Extensibility.Commands.Command;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal sealed class GenerateHttpFilesCommand : Command
{
    private readonly AsyncServiceProviderInjection<DTE, DTE2> dte;

    public GenerateHttpFilesCommand(AsyncServiceProviderInjection<DTE, DTE2> dte)
    {
        this.dte = dte;
    }

    public override CommandConfiguration CommandConfiguration => new("%HttpGenerator.GenerateHttpFiles.DisplayName%")
    {
        Placements = [CommandPlacement.KnownPlacements.ToolsMenu],
        Icon = new(ImageMoniker.KnownValues.Document, IconSettings.IconAndText),
    };

    public override async Task ExecuteCommandAsync(IClientContext context, CancellationToken cancellationToken)
    {
        var dte2 = await dte.GetServiceAsync();

        await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync(cancellationToken);

        var project = GetActiveProject(dte2);
        if (project is null)
        {
            return;
        }

        var projectDirectory = Path.GetDirectoryName(project.FullName);
        if (string.IsNullOrWhiteSpace(projectDirectory))
        {
            return;
        }

        using var dialog = new GenerateDialog(Path.Combine(projectDirectory, "HttpFiles"));
        dialog.ShowDialog();
    }

    private static Project? GetActiveProject(DTE2 dte)
    {
        ThreadHelper.ThrowIfNotOnUIThread();

        return (dte.ActiveSolutionProjects as Array)?
            .OfType<Project>()
            .FirstOrDefault();
    }
}
