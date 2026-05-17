using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;
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

    [VisualStudioContribution]
    public static MenuConfiguration GenerateMenu
        => new("%HttpGenerator.GroupDisplayName%")
        {
            Placements =
            [
                KnownPlacements.ItemNode,
                KnownPlacements.Node_IncludeExcludeGroup
            ],
            Children =
            [
                MenuChild.Command<Commands.GenerateTestPlansCommandExt>(),
                MenuChild.Separator,
                MenuChild.Command<Commands.AboutCommand>(),
            ],
        };

    [VisualStudioContribution]
    public static MenuConfiguration AddNewMenu
        => new("%AddNewCommand.GroupDisplayName%")
        {
            Placements =
            [
                KnownPlacements.ProjectNode_AddGroup_Submenu_ItemsGroup,
            ],
            Children =
            [
                MenuChild.Group(new CommandGroupConfiguration{
                    Children =
                    [
                        GroupChild.Command<Commands.GenerateTestPlansCommandExt>(),
                    ]
                })
            ],
        };

    protected override void InitializeServices(IServiceCollection serviceCollection)
    {
        serviceCollection.AddSingleton<Settings.ExtensionSettingsProvider>();
        base.InitializeServices(serviceCollection);
    }

    protected override async Task OnInitializedAsync(
        VisualStudioExtensibility extensibility,
        CancellationToken cancellationToken)
    {
        await base.OnInitializedAsync(extensibility, cancellationToken);

        // Check telemetry opt-out and other initialization logic
        try
        {
            var settingsProvider = this.Services.GetRequiredService<Settings.ExtensionSettingsProvider>();
            var opts = await settingsProvider.GetTelemetryOptionsAsync(cancellationToken);
            if (opts.TelemetryOptOut)
            {
                // If telemetry is opted out, rely on no-op analytics (not implemented here)
            }
        }
        catch
        {
            // Ignore initialization failures; extension should still function
        }
    }
}
