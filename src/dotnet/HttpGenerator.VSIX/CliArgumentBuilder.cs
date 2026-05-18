namespace HttpGenerator.VSIX;

internal static class CliArgumentBuilder
{
    public static IReadOnlyList<string> BuildArguments(
        string openApiPath,
        string outputFolder,
        string? baseUrl,
        string contentType,
        string? authorizationHeader,
        bool generateMultipleFiles)
    {
        var args = new List<string>
        {
            openApiPath,
            "--output",
            outputFolder,
        };

        if (!string.IsNullOrWhiteSpace(baseUrl))
        {
            args.Add("--base-url");
            args.Add(baseUrl);
        }

        if (!string.IsNullOrWhiteSpace(contentType) && !contentType.Equals("application/json", StringComparison.OrdinalIgnoreCase))
        {
            args.Add("--content-type");
            args.Add(contentType);
        }

        if (!string.IsNullOrWhiteSpace(authorizationHeader))
        {
            args.Add("--authorization-header");
            args.Add(authorizationHeader);
        }

        args.Add("--output-type");
        args.Add(generateMultipleFiles ? "OneRequestPerFile" : "OneFile");

        return args;
    }
}
