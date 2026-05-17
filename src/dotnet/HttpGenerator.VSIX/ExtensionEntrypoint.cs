using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Commands;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal sealed class ExtensionEntrypoint : Extension
{
    private const string ExtensionName = "HTTP File Generator for Visual Studio (PREVIEW)";

    public override ExtensionConfiguration ExtensionConfiguration => new()
    {
        Metadata = new(
            id: "f87f2d98-94ee-4bd1-86a6-aba346499100",
            version: ExtensionAssemblyVersion,
            publisherName: "Christian Resma Helle",
            displayName: ExtensionName,
            description: "Generate .http files from OpenAPI (Swagger) specifications")
        {
            Icon = "icon.png",
            License = "License.txt",
            Preview = true,
            Tags = ["OpenAPI", "Swagger", "REST Client"],
        },
    };

    [VisualStudioContribution]
    public static MenuConfiguration ToolsMenu => new("%HttpGenerator.GroupDisplayName%")
    {
        Placements = [CommandPlacement.KnownPlacements.ToolsMenu],
        Children =
        [
            MenuChild.Command<GenerateHttpFilesCommand>(),
        ],
    };
}
