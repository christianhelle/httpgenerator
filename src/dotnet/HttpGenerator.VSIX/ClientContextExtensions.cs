using Microsoft.VisualStudio.Extensibility;

namespace HttpGenerator.VSIX;

public static class ClientContextExtensions
{
    public static async Task<string?> GetSelectedPathAsync(
        this IClientContext context,
        CancellationToken cancellationToken)
    {
        var item = await context.GetActiveProjectAsync(cancellationToken);
        return item?.Path;
    }
}
