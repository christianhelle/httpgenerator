using Microsoft.VisualStudio.Extensibility;
using Microsoft.Extensions.DependencyInjection;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal class ExtensionEntrypoint : Extension
{
    const string ExtensionName = "HTTP File Generator";

    public override ExtensionConfiguration ExtensionConfiguration => new()
    {
        Metadata = new(
            id: "e8a3f2b4-3b6d-4f0b-9c2a-1a2b3c4d5e6f",
            version: typeof(ExtensionEntrypoint).Assembly.GetName().Version?.ToString() ?? "1.0.0",
            publisherName: "Christian Resma Helle",
            displayName: ExtensionName,
            description: "Generate HTTP files from REST API specifications")
        {
            Icon = "icon.png",
            License = "License.txt",
        },
    };

    protected override void InitializeServices(IServiceCollection serviceCollection)
    {
        // Register any services here if needed
        base.InitializeServices(serviceCollection);
    }

    protected override Task OnInitializedAsync(VisualStudioExtensibility extensibility, CancellationToken cancellationToken)
    {
        // Initialization logic can be added here. Keep minimal for compatibility with existing code while we migrate commands.
        return base.OnInitializedAsync(extensibility, cancellationToken);
    }
}
