using Microsoft.VisualStudio.Extensibility;

namespace HttpGenerator.VSIX;

internal static class ClientContextExtensions
{
    public static async Task<string?> TryGetSelectedOpenApiPathAsync(
        this IClientContext context,
        CancellationToken cancellationToken)
    {
        var selectedUri = await context.GetSelectedPathAsync(cancellationToken);
        var selectedPath = selectedUri?.LocalPath;

        return IsSupportedOpenApiPath(selectedPath)
            ? selectedPath
            : null;
    }

    public static bool IsSupportedOpenApiPath(string? path)
    {
        if (string.IsNullOrWhiteSpace(path))
        {
            return false;
        }

        var extension = Path.GetExtension(path);
        return extension.Equals(".json", StringComparison.OrdinalIgnoreCase)
            || extension.Equals(".yaml", StringComparison.OrdinalIgnoreCase)
            || extension.Equals(".yml", StringComparison.OrdinalIgnoreCase);
    }
}
