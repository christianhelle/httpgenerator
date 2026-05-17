using System.Threading;
using System.Windows.Forms;

namespace HttpGenerator.VSIX;

internal static class GenerateDialogHost
{
    public static Task ShowDialogAsync(string outputFolder, CancellationToken cancellationToken)
    {
        var completion = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);

        var thread = new Thread(() =>
        {
            try
            {
                cancellationToken.ThrowIfCancellationRequested();
                Application.EnableVisualStyles();
                Application.SetCompatibleTextRenderingDefault(false);

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
}
