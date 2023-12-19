using HttpGenerator.Core;
using System;
using System.Diagnostics.CodeAnalysis;
using System.IO;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Linq;

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

    private void btnSelectOpenApiFile_Click(object sender, EventArgs e)
    {
        openFileDialog.ShowDialog();
        txtOpenApiFile.Text = openFileDialog.FileName;
    }

    private void btnSelectOutputFolder_Click(object sender, EventArgs e)
    {
        folderBrowserDialog.ShowDialog();
        txtOutputFolder.Text = folderBrowserDialog.SelectedPath;
    }

    private void btnCancel_Click(object sender, EventArgs e)
    {
        Close();
    }

    [SuppressMessage(
        "Usage",
        "VSTHRD100:Avoid async void methods",
        Justification = "Legacy API")]
    private async void btnOk_Click(object sender, EventArgs e)
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
        var result = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = txtOpenApiFile.Text,
                BaseUrl = txtBaseUrl.Text,
                ContentType = txtContentType.Text,
                AuthorizationHeader = txtAuthorizationHeader.Text,
                OutputType = chkMultipleFiles.Checked
                    ? OutputType.OneRequestPerFile
                    : OutputType.OneFile,
            });

        var output = txtOutputFolder.Text;
        if (!Directory.Exists(output))
            Directory.CreateDirectory(output);

        var tasks = result
            .Files
            .Select(file => Task.Run(
                () => File.WriteAllText(
                    Path.Combine(output, file.Filename),
                    file.Content)));

        await Task.WhenAll(tasks);
    }
}
