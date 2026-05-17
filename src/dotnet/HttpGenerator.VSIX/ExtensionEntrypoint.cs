using Microsoft.VisualStudio.Extensibility;
using Microsoft.Extensions.DependencyInjection;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal class ExtensionEntrypoint : Extension
{
    const string ExtensionName = "HTTP File Generator for Visual Studio (PREVIEW)";

    public override ExtensionConfiguration ExtensionConfiguration => new()
    {
        Metadata = new(
            id: "f87f2d98-94ee-4bd1-86a6-aba346499100",
            version: typeof(ExtensionEntrypoint).Assembly.GetName().Version?.ToString() ?? "1.0.0",
            publisherName: "Christian Resma Helle",
            displayName: ExtensionName,
            description: "Generate .http files from OpenAPI (Swagger) specifications")
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
        // Initialization logic can be added here. Keep minimal for compatibility while migrating commands.
        return base.OnInitializedAsync(extensibility, cancellationToken);
    }
}
