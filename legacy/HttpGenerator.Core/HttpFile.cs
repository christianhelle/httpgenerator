using System.Diagnostics.CodeAnalysis;

namespace HttpGenerator.Core;

[ExcludeFromCodeCoverage]
public record HttpFile(string Filename, string Content)
{
    public string Filename { get; } = Filename;
    public string Content { get; } = Content;
}