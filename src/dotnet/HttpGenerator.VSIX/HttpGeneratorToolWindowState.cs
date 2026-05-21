using System.Diagnostics;
using System.Runtime.Serialization;

using HttpGenerator.VSIX.Settings;

using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.UI;

#pragma warning disable VSEXTPREVIEW_SETTINGS

namespace HttpGenerator.VSIX;

[DataContract]
internal sealed class HttpGeneratorToolWindowState : NotifyPropertyChangedObject
{
    private readonly HttpGeneratorSettingsProvider settingsProvider;
    private readonly IAsyncCommand saveSettingsCommand;
    private readonly IAsyncCommand openOutputFolderCommand;

    private bool useSiblingHttpFilesFolder = HttpGeneratorGenerationSettings.Default.UseSiblingHttpFilesFolder;
    private string baseUrl = HttpGeneratorGenerationSettings.Default.BaseUrl;
    private string contentType = HttpGeneratorGenerationSettings.Default.ContentType;
    private bool generateMultipleFiles = HttpGeneratorGenerationSettings.Default.GenerateMultipleFiles;
    private string statusHeadline = "No background runs yet.";
    private string statusDetails = "Choose an OpenAPI spec in Solution Explorer, then use the right-click command or the Tools fallback to start generation.";
    private string activeJobSummary = "No active generations.";
    private string lastOutputFolder = string.Empty;
    private bool canOpenOutputFolder;

    public HttpGeneratorToolWindowState(
        VisualStudioExtensibility extensibility,
        HttpGeneratorSettingsProvider settingsProvider)
    {
        this.settingsProvider = settingsProvider;
        this.saveSettingsCommand = new AsyncCommand(this.SaveSettingsAsync);
        this.openOutputFolderCommand = new AsyncCommand(this.OpenOutputFolderAsync);
        settingsProvider.Changed += this.OnSettingsChangedAsync;
        _ = this.InitializeAsync();
    }

    [DataMember]
    public bool UseSiblingHttpFilesFolder
    {
        get => useSiblingHttpFilesFolder;
        set => SetProperty(ref useSiblingHttpFilesFolder, value);
    }

    [DataMember]
    public string BaseUrl
    {
        get => baseUrl;
        set => SetProperty(ref baseUrl, value);
    }

    [DataMember]
    public string ContentType
    {
        get => contentType;
        set => SetProperty(ref contentType, value);
    }

    [DataMember]
    public bool GenerateMultipleFiles
    {
        get => generateMultipleFiles;
        set => SetProperty(ref generateMultipleFiles, value);
    }

    [DataMember]
    public string StatusHeadline
    {
        get => statusHeadline;
        private set => SetProperty(ref statusHeadline, value);
    }

    [DataMember]
    public string StatusDetails
    {
        get => statusDetails;
        private set => SetProperty(ref statusDetails, value);
    }

    [DataMember]
    public string ActiveJobSummary
    {
        get => activeJobSummary;
        private set => SetProperty(ref activeJobSummary, value);
    }

    [DataMember]
    public bool CanOpenOutputFolder
    {
        get => canOpenOutputFolder;
        private set => SetProperty(ref canOpenOutputFolder, value);
    }

    [DataMember]
    public IAsyncCommand SaveSettingsCommand => saveSettingsCommand;

    [DataMember]
    public IAsyncCommand OpenOutputFolderCommand => openOutputFolderCommand;

    public void RecordQueued(HttpGenerationRequest request, int activeJobCount)
    {
        StatusHeadline = $"Queued generation for {request.DisplayName}.";
        StatusDetails = BuildRequestDetails(request);
        ClearOutputFolder();
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordDuplicate(HttpGenerationRequest request, int activeJobCount)
    {
        StatusHeadline = $"Generation is already running for {request.DisplayName}.";
        StatusDetails = BuildRequestDetails(request);
        ClearOutputFolder();
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordStarted(HttpGenerationRequest request, int activeJobCount)
    {
        StatusHeadline = $"Generating .http files for {request.DisplayName}...";
        StatusDetails = BuildRequestDetails(request);
        ClearOutputFolder();
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordCompleted(HttpGenerationRequest request, GenerateResult result, int activeJobCount)
    {
        StatusHeadline = result.Summary;
        StatusDetails = result.Details;
        lastOutputFolder = request.OutputFolder;
        CanOpenOutputFolder = Directory.Exists(lastOutputFolder);
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordCancelled(HttpGenerationRequest request, GenerateResult result, int activeJobCount)
    {
        StatusHeadline = $"Cancelled generation for {request.DisplayName}.";
        StatusDetails = result.Details;
        lastOutputFolder = request.OutputFolder;
        CanOpenOutputFolder = Directory.Exists(lastOutputFolder);
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordFailure(HttpGenerationRequest request, Exception exception, int activeJobCount)
    {
        StatusHeadline = $"Generation failed for {request.DisplayName}.";
        StatusDetails = exception.ToString().Trim();
        lastOutputFolder = request.OutputFolder;
        CanOpenOutputFolder = Directory.Exists(lastOutputFolder);
        UpdateActiveJobCount(activeJobCount);
    }

    public void RecordFailure(HttpGenerationRequest request, GenerateResult result, int activeJobCount)
    {
        StatusHeadline = $"Generation failed for {request.DisplayName}.";
        StatusDetails = result.Details;
        lastOutputFolder = request.OutputFolder;
        CanOpenOutputFolder = Directory.Exists(lastOutputFolder);
        UpdateActiveJobCount(activeJobCount);
    }

    public void UpdateActiveJobCount(int activeJobCount)
    {
        ActiveJobSummary = activeJobCount switch
        {
            <= 0 => "No active generations.",
            1 => "1 generation is still running.",
            _ => $"{activeJobCount} generations are still running.",
        };
    }

    public void RecordSelectionRequired()
    {
        StatusHeadline = "Select an OpenAPI file to start generation.";
        StatusDetails = "Choose a .json, .yaml, or .yml file in Solution Explorer, then use Generate .http files again.";
    }

    private async Task InitializeAsync()
    {
        try
        {
            await ApplySnapshotAsync(await settingsProvider.GetRawSnapshotAsync(CancellationToken.None));
        }
        catch
        {
            // Keep defaults if the preview settings API is unavailable.
        }
    }

    private async Task OnSettingsChangedAsync(HttpGeneratorCategorySnapshot snapshot)
    {
        await ApplySnapshotAsync(snapshot);
    }

    private Task ApplySnapshotAsync(HttpGeneratorCategorySnapshot snapshot)
    {
        var settings = HttpGeneratorSettingsProvider.ToSettings(snapshot);
        UseSiblingHttpFilesFolder = settings.UseSiblingHttpFilesFolder;
        BaseUrl = settings.BaseUrl;
        ContentType = settings.ContentType;
        GenerateMultipleFiles = settings.GenerateMultipleFiles;
        return Task.CompletedTask;
    }

    private async Task SaveSettingsAsync(object? _, CancellationToken cancellationToken)
    {
        var settings = HttpGeneratorSettingsProvider.Sanitize(new HttpGeneratorGenerationSettings(
            UseSiblingHttpFilesFolder,
            BaseUrl,
            ContentType,
            GenerateMultipleFiles));

        await settingsProvider.SaveAsync(settings, cancellationToken);
        StatusHeadline = "Saved HTTP File Generator settings.";
        StatusDetails = """
            Updated persisted defaults for:
            - Output folder policy
            - Base URL override
            - Content type override
            - Generate-multiple-files
            """;
    }

    private Task OpenOutputFolderAsync(object? _, CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();

        if (!CanOpenOutputFolder)
        {
            return Task.CompletedTask;
        }

        FolderLauncher.Open(lastOutputFolder);

        return Task.CompletedTask;
    }

    private static string BuildRequestDetails(HttpGenerationRequest request)
    {
        return $"""
            OpenAPI file: {request.OpenApiPath}
            Output folder: {request.OutputFolder}
            Output policy: {request.Settings.OutputFolderPolicyLabel}
            Base URL: {(string.IsNullOrWhiteSpace(request.Settings.BaseUrl) ? "(default)" : request.Settings.BaseUrl)}
            Content type: {request.Settings.ContentType}
            Generate multiple files: {request.Settings.GenerateMultipleFiles}
            """;
    }

    private void ClearOutputFolder()
    {
        lastOutputFolder = string.Empty;
        CanOpenOutputFolder = false;
    }
}

#pragma warning restore VSEXTPREVIEW_SETTINGS
