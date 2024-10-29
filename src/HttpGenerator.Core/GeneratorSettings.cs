using System.Diagnostics.CodeAnalysis;

namespace HttpGenerator.Core;

/// <summary>
/// Provide settings for the .http file generator.
/// </summary>
[ExcludeFromCodeCoverage]
public class GeneratorSettings
{
    /// <summary>
    /// Gets or sets the path to the Open API (local file or URL)
    /// </summary>
    public string OpenApiPath { get; set; } = null!;
    
    /// <summary>
    /// Gets or sets the authorization header to use for all requests
    /// </summary>
    public string? AuthorizationHeader { get; set; }

    /// <summary>
    /// Gets or sets whether to load the authorization header
    /// from an environment variable or define it in the .http file
    /// </summary>
    public bool AuthorizationHeaderFromEnvironmentVariable { get; set; }

    /// <summary>
    /// Gets or sets the name of the environment variable to load the authorization header from
    /// </summary>
    public string AuthorizationHeaderVariableName { get; set; } = "authorization";
    
    /// <summary>
    /// Gets or sets the default Content-Type header to use for all requests
    /// </summary>
    public string ContentType { get; set; } = "application/json";

    /// <summary>
    /// Gets or sets the default BaseUrl to use for all requests
    /// </summary>
    public string? BaseUrl { get; set; }

    /// <summary>
    /// Gets or sets the default output type for the generated .http files 
    /// </summary>
    public OutputType OutputType { get; set; }
    
    /// <summary>
    /// Gets or sets the timeout (in seconds) for writing files to disk
    /// </summary>
    public int Timeout { get; set; } = 120;

    /// <summary>
    /// Gets or sets whether to generate IntelliJ tests
    /// </summary>
    public bool GenerateIntelliJTests { get; set; }

    /// <summary>
    /// Gets or sets custom HTTP headers to add to the generated request
    /// </summary>
    public string[]? CustomHeaders { get; set; }
}

/// <summary>
/// Defines the output type for the generated .http file
/// </summary>
public enum OutputType
{
    /// <summary>
    /// Generate one .http file per request
    /// </summary>
    OneRequestPerFile,
    
    /// <summary>
    /// Generate a single .http file for all requests
    /// </summary>
    OneFile,

    /// <summary>
    /// Generate one .http file per first tag associated with each request
    /// </summary>
    OneFilePerTag,
}