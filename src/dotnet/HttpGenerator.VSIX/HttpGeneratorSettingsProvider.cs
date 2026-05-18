using HttpGenerator.VSIX.Settings;

using Microsoft.VisualStudio.Extensibility;

#pragma warning disable VSEXTPREVIEW_SETTINGS

namespace HttpGenerator.VSIX;

internal sealed class HttpGeneratorSettingsProvider(
    VisualStudioExtensibility extensibility,
    HttpGeneratorCategoryObserver settingsObserver)
{
    public event Func<HttpGeneratorCategorySnapshot, Task>? Changed
    {
        add => settingsObserver.Changed += value;
        remove => settingsObserver.Changed -= value;
    }

    public Task<HttpGeneratorCategorySnapshot> GetRawSnapshotAsync(CancellationToken cancellationToken)
        => settingsObserver.GetSnapshotAsync(cancellationToken);

    public async Task<HttpGeneratorGenerationSettings> GetSnapshotAsync(CancellationToken cancellationToken)
    {
        var snapshot = await settingsObserver.GetSnapshotAsync(cancellationToken);
        return ToSettings(snapshot);
    }

    public async Task SaveAsync(HttpGeneratorGenerationSettings settings, CancellationToken cancellationToken)
    {
        var sanitized = Sanitize(settings);

        await extensibility.Settings().WriteAsync(
            batch =>
            {
                batch.WriteSetting(HttpGeneratorSettingDefinitions.UseSiblingHttpFilesFolder, sanitized.UseSiblingHttpFilesFolder);
                batch.WriteSetting(HttpGeneratorSettingDefinitions.BaseUrl, sanitized.BaseUrl);
                batch.WriteSetting(HttpGeneratorSettingDefinitions.ContentType, sanitized.ContentType);
                batch.WriteSetting(HttpGeneratorSettingDefinitions.GenerateMultipleFiles, sanitized.GenerateMultipleFiles);
            },
            description: "Update HTTP File Generator settings",
            cancellationToken);
    }

    public static HttpGeneratorGenerationSettings ToSettings(HttpGeneratorCategorySnapshot snapshot)
    {
        return Sanitize(new HttpGeneratorGenerationSettings(
            snapshot.UseSiblingHttpFilesFolder.ValueOrDefault(HttpGeneratorSettingDefinitions.UseSiblingHttpFilesFolder.DefaultValue),
            snapshot.BaseUrl.ValueOrDefault(HttpGeneratorSettingDefinitions.BaseUrl.DefaultValue),
            snapshot.ContentType.ValueOrDefault(HttpGeneratorSettingDefinitions.ContentType.DefaultValue),
            snapshot.GenerateMultipleFiles.ValueOrDefault(HttpGeneratorSettingDefinitions.GenerateMultipleFiles.DefaultValue)));
    }

    public static HttpGeneratorGenerationSettings Sanitize(HttpGeneratorGenerationSettings settings)
    {
        return settings with
        {
            BaseUrl = (settings.BaseUrl ?? string.Empty).Trim(),
            ContentType = string.IsNullOrWhiteSpace(settings.ContentType)
                ? HttpGeneratorSettingDefinitions.ContentType.DefaultValue
                : settings.ContentType.Trim(),
        };
    }
}

#pragma warning restore VSEXTPREVIEW_SETTINGS
