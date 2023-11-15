using System.Diagnostics.CodeAnalysis;

namespace HttpGenerator.Core;

/// <summary>
/// Provide settings for the .http file generator.
/// </summary>
[ExcludeFromCodeCoverage]
public class GeneratorSettings
{
    public const string DefaultOutputFolder = "./Generated";

    /// <summary>
    /// Gets or sets the path to the Open API (local file or URL)
    /// </summary>
    public string OpenApiPath { get; set; } = null!;
    
    /// <summary>
    /// Gets or sets the authorization header to use for all requests
    /// </summary>
    public string? AuthorizationHeader { get; set; }
    
    /// <summary>
    /// Gets or sets the default Content-Type header to use for all requests
    /// </summary>
    public string ContentType { get; set; } = "application/json";

    /// <summary>
    /// Gets or sets the default BaseUrl to use for all requests
    /// </summary>
    public string? BaseUrl { get; set; }
}