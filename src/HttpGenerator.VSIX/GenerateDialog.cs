using System;
using System.Diagnostics.CodeAnalysis;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace HttpGenerator.VSIX;

public partial class GenerateDialog : Form
{
    private string? azureTenantId;
    private string azureScope = string.Empty;

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
            await GenerateFilesAsync();
        }
        catch (Exception ex)
        {
            MessageBox.Show(
                ex.Message,
                "Operation failed",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error,
                MessageBoxDefaultButton.Button1);
        }

        Close();
    }

    private async Task GenerateFilesAsync()
    {
        var warning = await HttpGeneratorCli.GenerateAsync(
            new HttpGeneratorCliOptions
            {
                OpenApiPath = txtOpenApiFile.Text,
                OutputFolder = txtOutputFolder.Text,
                BaseUrl = txtBaseUrl.Text,
                ContentType = txtContentType.Text,
                AuthorizationHeader = txtAuthorizationHeader.Text,
                AzureTenantId = azureTenantId,
                AzureScope = azureScope,
                OutputType = chkMultipleFiles.Checked
                    ? "OneRequestPerFile"
                    : "OneFile",
            });

        if (!string.IsNullOrWhiteSpace(warning))
        {
            MessageBox.Show(
                warning,
                "Generation completed with warnings",
                MessageBoxButtons.OK,
                MessageBoxIcon.Warning,
                MessageBoxDefaultButton.Button1);
        }
    }

    private void btnAzureAccessToken_Click(object sender, EventArgs e)
    {
        using var dialog = new AzureAccessTokenDialog();
        if (dialog.ShowDialog() != DialogResult.OK)
        {
            return;
        }

        azureTenantId = dialog.TenantId;
        azureScope = dialog.Scope;
        txtAuthorizationHeader.Text = string.Empty;

        MessageBox.Show(
            "Azure Entra ID settings saved. The Rust CLI will acquire an access token during generation.",
            "Azure Entra ID",
            MessageBoxButtons.OK,
            MessageBoxIcon.Information,
            MessageBoxDefaultButton.Button1);
    }
}
