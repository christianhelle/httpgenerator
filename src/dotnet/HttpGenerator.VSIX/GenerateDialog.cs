using System;
using System.Diagnostics.CodeAnalysis;
using System.IO;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace HttpGenerator.VSIX;

public partial class GenerateDialog : Form
{
    public GenerateDialog()
    {
        InitializeComponent();
    }

    public GenerateDialog(string projectPath) : this()
    {
        txtOutputFolder.Text = projectPath;
        folderBrowserDialog.SelectedPath = Path.GetDirectoryName(projectPath);
    }

    private void OnSelectOpenApiFile_Click(object sender, EventArgs e)
    {
        openFileDialog.ShowDialog();
        txtOpenApiFile.Text = openFileDialog.FileName;
    }

    private void OnSelectOutputFolder_Click(object sender, EventArgs e)
    {
        folderBrowserDialog.ShowDialog();
        txtOutputFolder.Text = folderBrowserDialog.SelectedPath;
    }

    private void OnCancel_Click(object sender, EventArgs e)
    {
        Close();
    }

    [SuppressMessage(
        "Usage",
        "VSTHRD100:Avoid async void methods",
        Justification = "Legacy API")]
    private async void OnOk_Click(object sender, EventArgs e)
    {
        try
        {
            btnOk.Enabled = false;
            btnCancel.Enabled = false;
            btnSelectOpenApiFile.Enabled = false;
            btnSelectOutputFolder.Enabled = false;
            Text = "Generating...";

            var progress = new Progress<string>(message =>
            {
                Text = message;
            });

            var outputFolder = txtOutputFolder.Text;
            if (!Directory.Exists(outputFolder))
            {
                Directory.CreateDirectory(outputFolder);
            }

            var result = await HttpGeneratorCli.ExecuteAsync(
                txtOpenApiFile.Text,
                outputFolder,
                string.IsNullOrWhiteSpace(txtBaseUrl.Text) ? null : txtBaseUrl.Text,
                txtContentType.Text,
                string.IsNullOrWhiteSpace(txtAuthorizationHeader.Text) ? null : txtAuthorizationHeader.Text,
                chkMultipleFiles.Checked,
                progress,
                CancellationToken.None).ConfigureAwait(false);

            if (result.Success && result.FileCount > 0)
            {
                var message = $"Successfully generated {result.FileCount} file(s) in {outputFolder}";
                MessageBox.Show(
                    message,
                    "Generation Complete",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Information,
                    MessageBoxDefaultButton.Button1);
            }
            else
            {
                MessageBox.Show(
                    $"Generation completed but the result could not be verified. Check the output folder: {outputFolder}",
                    "Generation Complete",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Information,
                    MessageBoxDefaultButton.Button1);
            }
        }
        catch (Exception ex)
        {
            var shouldOfferDownload = ex.Message.Contains("httpgenerator") ||
                                      ex.Message.Contains("install") ||
                                      ex.Message.Contains("download");

            if (shouldOfferDownload)
            {
                var dialogResult = MessageBox.Show(
                    ex.Message + "\n\nWould you like to download the latest version of httpgenerator?",
                    "Download httpgenerator",
                    MessageBoxButtons.YesNoCancel,
                    MessageBoxIcon.Warning,
                    MessageBoxDefaultButton.Button1);

                if (dialogResult == DialogResult.Yes)
                {
                    try
                    {
                        var psi = new System.Diagnostics.ProcessStartInfo
                        {
                            FileName = "powershell",
                            Arguments = "-NoProfile -Command \"irm https://christianhelle.com/httpgenerator/install.ps1 | iex\"",
                            UseShellExecute = true,
                        };
                        System.Diagnostics.Process.Start(psi);
                        MessageBox.Show(
                            "Please complete the installation, then try again.",
                            "Installation",
                            MessageBoxButtons.OK,
                            MessageBoxIcon.Information,
                            MessageBoxDefaultButton.Button1);
                    }
                    catch (Exception innerEx)
                    {
                        MessageBox.Show(
                            innerEx.Message,
                            "Failed to launch installer",
                            MessageBoxButtons.OK,
                            MessageBoxIcon.Error,
                            MessageBoxDefaultButton.Button1);
                    }
                }
                else if (dialogResult == DialogResult.No)
                {
                    MessageBox.Show(
                        "Please install httpgenerator manually from https://github.com/christianhelle/httpgenerator/releases\n\n" + ex.Message,
                        "Installation Required",
                        MessageBoxButtons.OK,
                        MessageBoxIcon.Error,
                        MessageBoxDefaultButton.Button1);
                }
            }
            else
            {
                MessageBox.Show(
                    ex.Message,
                    "Operation failed",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error,
                    MessageBoxDefaultButton.Button1);
            }
        }
        finally
        {
            btnOk.Enabled = true;
            btnCancel.Enabled = true;
            btnSelectOpenApiFile.Enabled = true;
            btnSelectOutputFolder.Enabled = true;
            Text = "HTTP File Generator";
        }

        Close();
    }

    private void btnAzureAccessToken_Click(object sender, EventArgs e)
    {
        using var dialog = new AzureAccessTokenDialog();
        dialog.ShowDialog();
        txtAuthorizationHeader.Text = $"Bearer {dialog.AccessToken}";
    }
}
