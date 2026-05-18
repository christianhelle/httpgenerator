using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.Settings;

namespace HttpGenerator.VSIX;

#pragma warning disable VSEXTPREVIEW_SETTINGS

internal static class HttpGeneratorSettingDefinitions
{
    [VisualStudioContribution]
    internal static SettingCategory HttpGeneratorCategory { get; } = new("httpFileGenerator", "%Settings.HttpGenerator.DisplayName%")
    {
        Description = "%Settings.HttpGenerator.Description%",
        GenerateObserverClass = true,
    };

    [VisualStudioContribution]
    internal static Setting.Boolean UseSiblingHttpFilesFolder { get; } = new(
        "useSiblingHttpFilesFolder",
        "%Settings.HttpGenerator.OutputFolderPolicy.DisplayName%",
        HttpGeneratorCategory,
        defaultValue: true)
    {
        Description = "%Settings.HttpGenerator.OutputFolderPolicy.Description%",
    };

    [VisualStudioContribution]
    internal static Setting.String BaseUrl { get; } = new(
        "baseUrl",
        "%Settings.HttpGenerator.BaseUrl.DisplayName%",
        HttpGeneratorCategory,
        defaultValue: "")
    {
        Description = "%Settings.HttpGenerator.BaseUrl.Description%",
    };

    [VisualStudioContribution]
    internal static Setting.String ContentType { get; } = new(
        "contentType",
        "%Settings.HttpGenerator.ContentType.DisplayName%",
        HttpGeneratorCategory,
        defaultValue: "application/json")
    {
        Description = "%Settings.HttpGenerator.ContentType.Description%",
    };

    [VisualStudioContribution]
    internal static Setting.Boolean GenerateMultipleFiles { get; } = new(
        "generateMultipleFiles",
        "%Settings.HttpGenerator.GenerateMultipleFiles.DisplayName%",
        HttpGeneratorCategory,
        defaultValue: true)
    {
        Description = "%Settings.HttpGenerator.GenerateMultipleFiles.Description%",
    };
}

#pragma warning restore VSEXTPREVIEW_SETTINGS
