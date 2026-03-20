using System.Net;
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
        if (IsHttp(openApiPath))
        {
            var content = await GetHttpContent(openApiPath);
            return await ParseOpenApiContent(content, openApiPath);
        }
        else 
        {
            var content = File.ReadAllText(openApiPath);
            return await ParseOpenApiContent(content, openApiPath);
        }
    }

    private static async Task<OpenApiDocument> ParseOpenApiContent(string content, string openApiPath)
    {
        // Try to parse with Microsoft.OpenApi first
        try
        {
            using var stream = new MemoryStream(System.Text.Encoding.UTF8.GetBytes(content));
            var settings = CreateReaderSettings(openApiPath);
            var format = GetFormat(openApiPath, content);
            var result = await OpenApiDocument.LoadAsync(
                stream,
                format,
                settings,
                CancellationToken.None);
            return GetDocument(result, openApiPath);
        }
        catch (OpenApiUnsupportedSpecVersionException ex)
            when (ex.Message.Contains("3.1.0") || ex.Message.Contains("not supported"))
        {
            // If OpenAPI 3.1 is detected, try to downgrade to 3.0 for parsing
            var downgradedContent = DowngradeOpenApi31To30(content);
            using var stream = new MemoryStream(System.Text.Encoding.UTF8.GetBytes(downgradedContent));
            var settings = CreateReaderSettings(openApiPath);
            var format = GetFormat(openApiPath, downgradedContent);
            var result = await OpenApiDocument.LoadAsync(
                stream,
                format,
                settings,
                CancellationToken.None);
            return GetDocument(result, openApiPath);
        }
    }

    private static OpenApiDocument GetDocument(ReadResult result, string openApiPath)
    {
        return result.Document ?? throw new InvalidOperationException(
            $"OpenAPI document could not be parsed from {openApiPath}.");
    }

    private static OpenApiReaderSettings CreateReaderSettings(string openApiPath)
    {
        var settings = new OpenApiReaderSettings
        {
            BaseUrl = GetBaseUrl(openApiPath)
        };
        settings.AddYamlReader();
        return settings;
    }

    private static Uri GetBaseUrl(string openApiPath)
    {
        if (IsHttp(openApiPath))
        {
            return new Uri(openApiPath);
        }

        var directoryName = new FileInfo(openApiPath).DirectoryName;
        return new Uri($"file://{directoryName}{Path.DirectorySeparatorChar}");
    }

    private static string GetFormat(string openApiPath, string content)
    {
        if (IsYaml(openApiPath))
            return "yaml";

        var trimmed = content.AsSpan().TrimStart();
        return trimmed.StartsWith("{") || trimmed.StartsWith("[")
            ? "json"
            : "yaml";
    }

    private static string DowngradeOpenApi31To30(string content)
    {
        // Simple downgrade strategy: replace 3.1.0 with 3.0.3 and remove unsupported 3.1 features
        return content
            .Replace("\"openapi\": \"3.1.0\"", "\"openapi\": \"3.0.3\"")
            .Replace("openapi: 3.1.0", "openapi: 3.0.3")
            .Replace("openapi: \"3.1.0\"", "openapi: \"3.0.3\"")
            // Remove webhooks section which is 3.1 specific
            .Replace("\"webhooks\":", "\"x-webhooks\":")
            .Replace("webhooks:", "x-webhooks:");
    }

    /// <summary>
    /// Gets the content of the URI as a string and decompresses it if necessary. 
    /// </summary>
    /// <returns>The content of the HTTP request.</returns>
    private static async Task<string> GetHttpContent(string openApiPath)
    {
        var httpMessageHandler = new HttpClientHandler
        {
            AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate
        };
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
        return path.StartsWith("http://", StringComparison.OrdinalIgnoreCase) || 
               path.StartsWith("https://", StringComparison.OrdinalIgnoreCase);
    }

    /// <summary>
    /// Determines whether the specified path is a YAML file.
    /// </summary>
    /// <param name="path">The path to check.</param>
    /// <returns>True if the path is a YAML file, otherwise false.</returns>
    private static bool IsYaml(string path)
    {
        return path.EndsWith(".yaml", StringComparison.OrdinalIgnoreCase) || 
               path.EndsWith(".yml", StringComparison.OrdinalIgnoreCase);
    }
}
