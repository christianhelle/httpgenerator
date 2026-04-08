using System.Diagnostics.CodeAnalysis;

namespace HttpGenerator.Core;

[ExcludeFromCodeCoverage]
public record GeneratorResult(IReadOnlyCollection<HttpFile> Files)
{
    public IReadOnlyCollection<HttpFile> Files { get; } = Files;
}