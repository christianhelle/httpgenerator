using System.Net;
using Microsoft.OpenApi.Models;
using Microsoft.OpenApi.Readers;

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
            return await ParseOpenApiContent(content);
        }
        else 
        {
            var content = File.ReadAllText(openApiPath);
            return await ParseOpenApiContent(content);
        }
    }

    private static async Task<OpenApiDocument> ParseOpenApiContent(string content)
    {
        // Try to parse with Microsoft.OpenApi first
        try
        {
            using var stream = new MemoryStream(System.Text.Encoding.UTF8.GetBytes(content));
            var reader = new OpenApiStreamReader();
            var result = await reader.ReadAsync(stream, CancellationToken.None);
            return result.OpenApiDocument;
        }
        catch (OverflowException)
        {
            // Handle int64 minimum/maximum values that overflow int32
            // This is a known issue when OpenAPI specs have int64 properties with 
            // minimum values lower than int32.MinValue (-2147483648)
            var fixedContent = FixInt64OverflowIssues(content);
            using var stream = new MemoryStream(System.Text.Encoding.UTF8.GetBytes(fixedContent));
            var reader = new OpenApiStreamReader();
            var result = await reader.ReadAsync(stream, CancellationToken.None);
            return result.OpenApiDocument;
        }
        catch (Exception ex) when (ex.Message.Contains("3.1.0") || ex.Message.Contains("not supported"))
        {
            // If OpenAPI 3.1 is detected, try to downgrade to 3.0 for parsing
            var downgradedContent = DowngradeOpenApi31To30(content);
            using var stream = new MemoryStream(System.Text.Encoding.UTF8.GetBytes(downgradedContent));
            var reader = new OpenApiStreamReader();
            var result = await reader.ReadAsync(stream, CancellationToken.None);
            return result.OpenApiDocument;
        }
    }

    private static string FixInt64OverflowIssues(string content)
    {
        // Replace int64 minimum/maximum values that overflow int32
        // This is a workaround for OpenAPI specs that use the full int64 range
        // The OpenAPI reader has issues parsing these extreme values
        
        // Common problematic values
        content = content
            // int64.MinValue
            .Replace("minimum: -9223372036854775808", "minimum: -9223372036854775807")
            .Replace("\"minimum\": -9223372036854775808", "\"minimum\": -9223372036854775807")
            // int64.MaxValue (shouldn't cause issues but handle it for consistency)
            .Replace("maximum: 9223372036854775808", "maximum: 9223372036854775807")
            .Replace("\"maximum\": 9223372036854775808", "\"maximum\": 9223372036854775807");
        
        // Use regex to find and fix any negative minimum values less than int32.MinValue
        var minimumPattern = new System.Text.RegularExpressions.Regex(
            @"(""minimum""\s*:\s*|minimum:\s*)(-\d{10,})");
        
        content = minimumPattern.Replace(content, match =>
        {
            var prefix = match.Groups[1].Value;
            var valueStr = match.Groups[2].Value;
            
            if (long.TryParse(valueStr, out var value))
            {
                // If the value is less than int32.MinValue, adjust it
                if (value < int.MinValue)
                {
                    // Use int32.MinValue as the replacement
                    return $"{prefix}{int.MinValue}";
                }
            }
            
            return match.Value;
        });
        
        // Use regex to find and fix any positive maximum values greater than int32.MaxValue
        var maximumPattern = new System.Text.RegularExpressions.Regex(
            @"(""maximum""\s*:\s*|maximum:\s*)(\d{10,})");
        
        content = maximumPattern.Replace(content, match =>
        {
            var prefix = match.Groups[1].Value;
            var valueStr = match.Groups[2].Value;
            
            if (long.TryParse(valueStr, out var value))
            {
                // If the value is greater than int32.MaxValue but we're in int64 context
                // Keep the original value - the issue is mainly with minimum values
                return match.Value;
            }
            
            return match.Value;
        });
        
        return content;
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
        var httpMessageHandler = new HttpClientHandler();
        httpMessageHandler.AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate;
        httpMessageHandler.ServerCertificateCustomValidationCallback = (message, cert, chain, errors) => true;
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