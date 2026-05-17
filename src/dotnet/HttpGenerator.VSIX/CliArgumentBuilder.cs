namespace HttpGenerator.VSIX;

internal static class CliArgumentBuilder
{
    public static string BuildArguments(
        string openApiPath,
        string outputFolder,
        string? baseUrl,
        string contentType,
        string? authorizationHeader,
        bool generateMultipleFiles)
    {
        var args = Escape(openApiPath) + " --output " + Escape(outputFolder);

        if (!string.IsNullOrWhiteSpace(baseUrl))
        {
            args += " --base-url " + Escape(baseUrl);
        }

        if (!string.IsNullOrWhiteSpace(contentType) && contentType != "application/json")
        {
            args += " --content-type " + Escape(contentType);
        }

        if (!string.IsNullOrWhiteSpace(authorizationHeader))
        {
            args += " --authorization-header " + Escape(authorizationHeader);
        }

        if (generateMultipleFiles)
        {
            args += " --output-type OneRequestPerFile";
        }
        else
        {
            args += " --output-type OneFile";
        }

        return args;
    }

    private static string Escape(string value)
    {
        var escaped = value.Replace("\\", "\\\\").Replace(" ", "\\ ");
        return "\"" + escaped + "\"";
    }
}
