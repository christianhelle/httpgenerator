using Microsoft.Extensions.DependencyInjection;
using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal class ExtensionEntrypoint : Extension
{
    public override ExtensionConfiguration ExtensionConfiguration => new()
    {
        Metadata = new(
            id: "f87f2d98-94ee-4bd1-86a6-aba346499100",
            version: ExtensionAssemblyVersion,
            publisherName: "Christian Resma Helle",
            displayName: "HTTP File Generator (PREVIEW)",
            description: "Generate .http files from OpenAPI (Swagger) specifications")
        {
            Icon = "icon.png",
            License = "LICENSE.txt",
        },
    };

    [VisualStudioContribution]
    public static MenuConfiguration GenerateMenu
        => new("%HttpGenerator.GroupDisplayName%")
        {
            Placements =
            [
                CommandPlacement.KnownPlacements.ToolsMenu,
            ],
            Children =
            [
                MenuChild.Command<Commands.GenerateHttpFallbackCommand>(),
                MenuChild.Command<Commands.ShowHttpGeneratorToolWindowCommand>(),
            ],
        };

    protected override void InitializeServices(IServiceCollection serviceCollection)
    {
        serviceCollection.AddSettingsObservers();
        serviceCollection.AddSingleton<HttpGeneratorSettingsProvider>();
        serviceCollection.AddSingleton<HttpGeneratorToolWindowState>();
        serviceCollection.AddSingleton<BackgroundGenerationCoordinator>();
        base.InitializeServices(serviceCollection);
    }

    protected override async Task OnInitializedAsync(
        VisualStudioExtensibility extensibility,
        CancellationToken cancellationToken)
    {
        await base.OnInitializedAsync(extensibility, cancellationToken);
    }
}
