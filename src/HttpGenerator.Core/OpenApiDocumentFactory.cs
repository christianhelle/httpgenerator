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
        var result = await OpenApiMultiFileReader.Read(openApiPath);
        return result.OpenApiDocument;
    }
}