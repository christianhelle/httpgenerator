namespace HttpGenerator.VSIX;

internal sealed record HttpGeneratorGenerationSettings(
    bool UseSiblingHttpFilesFolder,
    string BaseUrl,
    string ContentType,
    bool GenerateMultipleFiles)
{
    public string ResolveOutputFolder(string openApiPath)
    {
        var directory = Path.GetDirectoryName(openApiPath) ?? string.Empty;
        return UseSiblingHttpFilesFolder
            ? Path.Combine(directory, "HttpFiles")
            : directory;
    }

    public string OutputFolderPolicyLabel => UseSiblingHttpFilesFolder
        ? "Sibling HttpFiles folder"
        : "Same folder as spec";

    public static HttpGeneratorGenerationSettings Default { get; } = new(
        UseSiblingHttpFilesFolder: true,
        BaseUrl: string.Empty,
        ContentType: "application/json",
        GenerateMultipleFiles: true);
}
