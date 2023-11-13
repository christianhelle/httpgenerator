namespace HttpGenerator.Core;

public record GeneratorResult(IReadOnlyCollection<HttpFile> Files)
{
    public IReadOnlyCollection<HttpFile> Files { get; } = Files;
}