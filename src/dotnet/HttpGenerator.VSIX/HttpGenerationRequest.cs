namespace HttpGenerator.VSIX;

internal sealed record HttpGenerationRequest(
    string OpenApiPath,
    string OutputFolder,
    HttpGeneratorGenerationSettings Settings)
{
    public string DisplayName => Path.GetFileName(OpenApiPath);

    public static HttpGenerationRequest Create(
        string openApiPath,
        HttpGeneratorGenerationSettings settings)
    {
        return new HttpGenerationRequest(
            openApiPath,
            settings.ResolveOutputFolder(openApiPath),
            settings);
    }
}
