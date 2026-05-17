using System.Threading;
using System.Windows.Forms;

namespace HttpGenerator.VSIX;

internal static class GenerateDialogHost
{
    private static int visualStylesInitialized;

    public static Task ShowDialogAsync(string outputFolder, CancellationToken cancellationToken)
    {
        var completion = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);

        var thread = new Thread(() =>
        {
            try
            {
                cancellationToken.ThrowIfCancellationRequested();
                InitializeVisualStyles();

                using var dialog = new GenerateDialog(outputFolder);
                dialog.ShowDialog();
                completion.TrySetResult();
            }
            catch (OperationCanceledException exception)
            {
                completion.TrySetCanceled(exception.CancellationToken);
            }
            catch (Exception exception)
            {
                completion.TrySetException(exception);
            }
        });

        thread.SetApartmentState(ApartmentState.STA);
        thread.Start();

        return completion.Task;
    }

    private static void InitializeVisualStyles()
    {
        if (Interlocked.Exchange(ref visualStylesInitialized, 1) == 0)
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
        }
    }
}
