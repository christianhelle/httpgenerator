using Microsoft.VisualStudio.Extensibility;
using Microsoft.VisualStudio.Extensibility.ToolWindows;
using Microsoft.VisualStudio.RpcContracts.RemoteUI;

namespace HttpGenerator.VSIX;

[VisualStudioContribution]
internal sealed class HttpGeneratorToolWindow(HttpGeneratorToolWindowState toolWindowState) : ToolWindow
{
    public override ToolWindowConfiguration ToolWindowConfiguration => new()
    {
        Placement = ToolWindowPlacement.DocumentWell,
        AllowAutoCreation = false,
    };

    public override Task<IRemoteUserControl> GetContentAsync(CancellationToken cancellationToken)
    {
        Title = "HTTP File Generator";
        return Task.FromResult<IRemoteUserControl>(new HttpGeneratorToolWindowControl(toolWindowState));
    }
}
