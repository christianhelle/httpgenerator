using System.Diagnostics;

namespace HttpGenerator.VSIX;

internal static class ProcessRunner
{
    public static void WaitForExit(Process process, CancellationToken cancellationToken, TimeSpan? timeout = null)
    {
        var startedAt = DateTimeOffset.UtcNow;
        while (!process.WaitForExit(250))
        {
            if (cancellationToken.IsCancellationRequested)
            {
                TryKill(process);
                cancellationToken.ThrowIfCancellationRequested();
            }

            if (timeout is not null && DateTimeOffset.UtcNow - startedAt > timeout)
            {
                TryKill(process);
                throw new TimeoutException($"The process did not exit within {timeout.Value.TotalMinutes:N0} minutes.");
            }
        }

        process.WaitForExit();
    }

    private static void TryKill(Process process)
    {
        try
        {
            if (!process.HasExited)
            {
                process.Kill();
            }
        }
        catch (InvalidOperationException)
        {
        }
    }
}
