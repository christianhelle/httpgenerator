using System.Net;
using NSwag;

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
        OpenApiDocument document;
        if (IsHttp(openApiPath))
        {
            var content = await GetHttpContent(openApiPath);

            if (IsYaml(openApiPath))
            {
                document = await OpenApiYamlDocument.FromYamlAsync(content);
            }
            else
            {
                document = await OpenApiDocument.FromJsonAsync(content);
            }
        }
        else 
        {
            if (IsYaml(openApiPath))
            {
                document = await OpenApiYamlDocument.FromFileAsync(openApiPath);
            }
            else
            {
                document = await OpenApiDocument.FromFileAsync(openApiPath);
            }
        }

        return document;
    }

    /// <summary>
    /// Gets the content of the URI as a string and decompresses it if necessary. 
    /// </summary>
    /// <returns>The content of the HTTP request.</returns>
    private static async Task<string> GetHttpContent(string openApiPath)
    {
        var httpMessageHandler = new HttpClientHandler();
        httpMessageHandler.AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate;
        using var http = new HttpClient(httpMessageHandler);
        var content = await http.GetStringAsync(openApiPath);
        return content;
    }

    /// <summary>
    /// Determines whether the specified path is an HTTP URL.
    /// </summary>
    /// <param name="path">The path to check.</param>
    /// <returns>True if the path is an HTTP URL, otherwise false.</returns>
    private static bool IsHttp(string path)
    {
        return path.StartsWith("http://") || path.StartsWith("https://");
    }

    /// <summary>
    /// Determines whether the specified path is a YAML file.
    /// </summary>
    /// <param name="path">The path to check.</param>
    /// <returns>True if the path is a YAML file, otherwise false.</returns>
    private static bool IsYaml(string path)
    {
        return path.EndsWith("yaml") || path.EndsWith("yml");
    }
}