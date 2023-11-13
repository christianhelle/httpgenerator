namespace HttpGenerator.Core;

public record HttpFile(string Filename, string Content)
{
    public string Filename { get; } = Filename;
    public string Content { get; } = Content;
}