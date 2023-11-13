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
}