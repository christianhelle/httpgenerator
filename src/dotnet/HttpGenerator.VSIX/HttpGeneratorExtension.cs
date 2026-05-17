using Microsoft.VisualStudio.Extensibility;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
public sealed class HttpGeneratorExtension : Extension
{
    public override ExtensionConfiguration ExtensionConfiguration => new()
    {
        RequiresInProcessHosting = true,
    };

    protected override void InitializeServices(IServiceCollection serviceCollection)
    {
        base.InitializeServices(serviceCollection);
        CliInstaller.EnsureInstalledInBackground();
    }
}
