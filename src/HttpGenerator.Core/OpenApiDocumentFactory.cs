using System.Linq;
using System.Net;
using System.Text;
using Microsoft.OpenApi;
using Microsoft.OpenApi.Reader;

namespace HttpGenerator.Core;

/// <summary>
/// Creates an <see cref="OpenApiDocument"/> from a specified path or URL.
/// </summary>
public static class OpenApiDocumentFactory
{
    /// <summary>
    /// Creates a new instance of the <see cref="OpenApiDocument"/> class asynchronously.
    /// </summary>
    /// <returns>A new instance of the <see cref="OpenApiDocument"/> class.</returns>
    public static async Task<OpenApiDocument> CreateAsync(string openApiPath)
    {
        var baseUrl = GetBaseUrl(openApiPath);

        if (IsHttp(openApiPath))
        {
            var content = await GetHttpContent(openApiPath);
            return await ParseOpenApiContent(content, GetFormat(openApiPath, content), baseUrl);
        }

        var fileContent = File.ReadAllBytes(openApiPath);
        return await ParseOpenApiContent(
            fileContent,
            GetFormat(openApiPath, fileContent),
            baseUrl);
    }

    private static async Task<OpenApiDocument> ParseOpenApiContent(
        byte[] content,
        string format,
        Uri? baseUrl)
    {
        using var stream = new MemoryStream(content);
        var readerSettings = new OpenApiReaderSettings
        {
            BaseUrl = baseUrl
        };
        readerSettings.AddYamlReader();

        var result = await OpenApiDocument.LoadAsync(
            stream,
            format,
            readerSettings,
            CancellationToken.None);

        return result.Document ?? throw CreateDocumentLoadException(result.Diagnostic);
    }

    /// <summary>
    /// Determines whether the specified path is an HTTP URL.
    /// </summary>
    /// <param name="path">The path to check.</param>
    /// <returns>True if the path is an HTTP URL, otherwise false.</returns>
    private static bool IsHttp(string path)
    {
        return path.StartsWith("http://", StringComparison.OrdinalIgnoreCase) || 
               path.StartsWith("https://", StringComparison.OrdinalIgnoreCase);
    }

    private static Uri? GetBaseUrl(string openApiPath)
    {
        if (IsHttp(openApiPath))
        {
            return new Uri(openApiPath);
        }

        var directoryName = Path.GetDirectoryName(Path.GetFullPath(openApiPath));
        if (string.IsNullOrWhiteSpace(directoryName))
        {
            return null;
        }

        var endsWithDirectorySeparator =
            directoryName.EndsWith(Path.DirectorySeparatorChar.ToString(), StringComparison.Ordinal) ||
            directoryName.EndsWith(Path.AltDirectorySeparatorChar.ToString(), StringComparison.Ordinal);

        var fullDirectoryPath = endsWithDirectorySeparator
            ? directoryName
            : directoryName + Path.DirectorySeparatorChar;

        return new Uri(fullDirectoryPath);
    }

    private static async Task<byte[]> GetHttpContent(string openApiPath)
    {
        using var http = CreateHttpClient();
        return await http.GetByteArrayAsync(openApiPath);
    }

    private static string GetFormat(string openApiPath, byte[] content)
    {
        var extension = Path.GetExtension(openApiPath);
        if (extension.Equals(".yaml", StringComparison.OrdinalIgnoreCase) ||
            extension.Equals(".yml", StringComparison.OrdinalIgnoreCase))
        {
            return "yaml";
        }

        if (extension.Equals(".json", StringComparison.OrdinalIgnoreCase))
        {
            return "json";
        }

        var text = Encoding.UTF8.GetString(content);
        var firstMeaningfulCharacter = text
            .SkipWhile(character => char.IsWhiteSpace(character) || character == '\uFEFF')
            .FirstOrDefault();

        return firstMeaningfulCharacter is '{' or '['
            ? "json"
            : "yaml";
    }

    private static HttpClient CreateHttpClient()
    {
        var httpMessageHandler = new HttpClientHandler
        {
            AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate,
            ServerCertificateCustomValidationCallback = (message, cert, chain, errors) => true
        };
        return new HttpClient(httpMessageHandler);
    }

    private static InvalidOperationException CreateDocumentLoadException(OpenApiDiagnostic? diagnostic)
    {
        if (diagnostic is null)
        {
            return new InvalidOperationException("Could not parse the OpenAPI document.");
        }

        var messages = diagnostic.Errors
            .Concat(diagnostic.Warnings)
            .Select(error => error.ToString())
            .Where(message => !string.IsNullOrWhiteSpace(message))
            .ToArray();

        if (messages.Length == 0)
        {
            return new InvalidOperationException("Could not parse the OpenAPI document.");
        }

        return new InvalidOperationException(
            $"Could not parse the OpenAPI document.{Environment.NewLine}{string.Join(Environment.NewLine, messages)}");
    }
}
