using System.Diagnostics;

namespace HttpGenerator.VSIX;

internal static class FolderLauncher
{
    public static void Open(string folderPath)
    {
        if (string.IsNullOrWhiteSpace(folderPath) || !Directory.Exists(folderPath))
        {
            return;
        }

        Process.Start(new ProcessStartInfo
        {
            FileName = folderPath,
            UseShellExecute = true,
        });
    }
}
