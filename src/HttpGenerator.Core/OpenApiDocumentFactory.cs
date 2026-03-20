using System.Net;
using System.Security;
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
        using var httpClient = CreateHttpClient();
        var content = await GetOpenApiContent(openApiPath, httpClient);
        var settings = CreateReaderSettings(openApiPath);
        using var stream = new MemoryStream(Encoding.UTF8.GetBytes(content));
        var result = await OpenApiDocument.LoadAsync(
            stream,
            GetFormat(openApiPath, content),
            settings,
            CancellationToken.None);

        return result.Document ?? throw CreateOpenApiLoadException(openApiPath, result.Diagnostic);
    }

    private static OpenApiReaderSettings CreateReaderSettings(string openApiPath)
    {
        var settings = new OpenApiReaderSettings
        {
            BaseUrl = GetBaseUrl(openApiPath),
        };

        settings.AddYamlReader();
        return settings;
    }

    private static async Task<string> GetOpenApiContent(
        string openApiPath,
        HttpClient httpClient)
    {
        if (IsHttp(openApiPath))
        {
            try
            {
                return await httpClient.GetStringAsync(openApiPath);
            }
            catch (HttpRequestException ex)
            {
                throw new InvalidOperationException($"Could not download the file at {openApiPath}", ex);
            }
        }

        try
        {
            return File.ReadAllText(openApiPath);
        }
        catch (Exception ex) when (ex is FileNotFoundException ||
                                   ex is PathTooLongException ||
                                   ex is DirectoryNotFoundException ||
                                   ex is IOException ||
                                   ex is UnauthorizedAccessException ||
                                   ex is SecurityException ||
                                   ex is NotSupportedException)
        {
            throw new InvalidOperationException($"Could not open the file at {openApiPath}", ex);
        }
    }

    private static HttpClient CreateHttpClient()
    {
        var httpClientHandler = new HttpClientHandler
        {
            AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate,
            ServerCertificateCustomValidationCallback = (message, cert, chain, errors) => true,
            SslProtocols = System.Security.Authentication.SslProtocols.Tls12,
        };

        return new HttpClient(httpClientHandler);
    }

    private static Uri GetBaseUrl(string openApiPath)
    {
        if (IsHttp(openApiPath))
            return new Uri(openApiPath);

        var directoryName = new FileInfo(openApiPath).DirectoryName;
        if (directoryName is null)
            throw new InvalidOperationException($"Could not determine the base path for {openApiPath}");

        return new Uri(directoryName + Path.DirectorySeparatorChar, UriKind.Absolute);
    }

    private static string GetFormat(
        string openApiPath,
        string content)
    {
        if (openApiPath.EndsWith(".json", StringComparison.OrdinalIgnoreCase))
            return "json";

        if (openApiPath.EndsWith(".yaml", StringComparison.OrdinalIgnoreCase) ||
            openApiPath.EndsWith(".yml", StringComparison.OrdinalIgnoreCase))
        {
            return "yaml";
        }

        var firstNonWhitespaceCharacter = content.FirstOrDefault(c => !char.IsWhiteSpace(c));
        return firstNonWhitespaceCharacter is '{' or '['
            ? "json"
            : "yaml";
    }

    private static InvalidOperationException CreateOpenApiLoadException(
        string openApiPath,
        OpenApiDiagnostic? diagnostic)
    {
        if (diagnostic is not null && diagnostic.Errors.Count > 0)
        {
            var errors = string.Join(
                Environment.NewLine,
                diagnostic.Errors.Select(error => error.ToString()));

            return new InvalidOperationException(
                $"Could not parse the file at {openApiPath}{Environment.NewLine}{errors}");
        }

        return new InvalidOperationException($"Could not parse the file at {openApiPath}");
    }

    private static bool IsHttp(string path)
    {
        return path.StartsWith("http://", StringComparison.OrdinalIgnoreCase) ||
               path.StartsWith("https://", StringComparison.OrdinalIgnoreCase);
    }
}
