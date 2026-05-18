namespace HttpGenerator.VSIX;

internal sealed class HttpGeneratorCliException(GenerateResult result) : Exception(result.Summary)
{
    public GenerateResult Result { get; } = result;
}
