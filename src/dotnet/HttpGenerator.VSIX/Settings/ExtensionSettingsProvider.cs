using System.Text.Json;

namespace HttpGenerator.VSIX.Settings;

public class TelemetryOptions
{
    public bool TelemetryOptOut { get; set; }
}

public class ExtensionSettingsProvider
{
    private readonly string _settingsPath;

    public ExtensionSettingsProvider()
    {
        var appData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
        var dir = Path.Combine(appData, "HttpGenerator.VSIX");
        Directory.CreateDirectory(dir);
        _settingsPath = Path.Combine(dir, "settings.json");
    }

    public async Task<TelemetryOptions> GetTelemetryOptionsAsync(CancellationToken cancellationToken)
    {
        if (!File.Exists(_settingsPath))
        {
            var defaultOpts = new TelemetryOptions { TelemetryOptOut = false };
            await WriteAsync(defaultOpts, cancellationToken).ConfigureAwait(false);
            return defaultOpts;
        }

        using var fs = File.OpenRead(_settingsPath);
        var opts = await JsonSerializer.DeserializeAsync<TelemetryOptions>(fs, cancellationToken: cancellationToken).ConfigureAwait(false);
        return opts ?? new TelemetryOptions { TelemetryOptOut = false };
    }

    public async Task SetTelemetryOptionsAsync(TelemetryOptions options, CancellationToken cancellationToken)
    {
        await WriteAsync(options, cancellationToken).ConfigureAwait(false);
    }

    private async Task WriteAsync(TelemetryOptions options, CancellationToken cancellationToken)
    {
        using var fs = File.Create(_settingsPath);
        await JsonSerializer.SerializeAsync(fs, options, cancellationToken: cancellationToken).ConfigureAwait(false);
    }
}
