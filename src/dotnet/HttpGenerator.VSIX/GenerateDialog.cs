using System;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using System.IO;
using System.Collections.Generic;
using System.Linq;
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
        var executablePath = await CliInstaller.ResolveOrInstallAsync(CancellationToken.None);
        var arguments = BuildArguments();
        await RunHttpGeneratorAsync(executablePath, arguments);
    }

    private string[] BuildArguments()
    {
        var outputType = chkMultipleFiles.Checked
            ? "OneRequestPerFile"
            : "OneFile";

        var arguments = new List<string>
        {
            txtOpenApiFile.Text,
            "--output",
            txtOutputFolder.Text,
            "--output-type",
            outputType,
        };

        AddOption(arguments, "--base-url", txtBaseUrl.Text);
        AddOption(arguments, "--content-type", txtContentType.Text);
        AddOption(arguments, "--authorization-header", txtAuthorizationHeader.Text);

        return arguments.ToArray();
    }

    private static void AddOption(List<string> arguments, string option, string value)
    {
        if (string.IsNullOrWhiteSpace(value))
        {
            return;
        }

        arguments.Add(option);
        arguments.Add(value);
    }

    private static async Task RunHttpGeneratorAsync(string executablePath, string[] arguments)
    {
        var startInfo = new ProcessStartInfo(executablePath)
        {
            Arguments = string.Join(" ", arguments.Select(QuoteArgument)),
            CreateNoWindow = true,
            RedirectStandardError = true,
            RedirectStandardOutput = true,
            UseShellExecute = false,
        };

        using var process = Process.Start(startInfo)
            ?? throw new InvalidOperationException("Failed to start httpgenerator.");

        var outputTask = process.StandardOutput.ReadToEndAsync();
        var errorTask = process.StandardError.ReadToEndAsync();
        await Task.Run(process.WaitForExit);

        var output = await outputTask;
        var error = await errorTask;
        if (process.ExitCode != 0)
        {
            throw new InvalidOperationException(
                $"httpgenerator failed with exit code {process.ExitCode}.{Environment.NewLine}{output}{Environment.NewLine}{error}");
        }
    }

    private static string QuoteArgument(string argument)
    {
        var quoted = "\"";
        var backslashes = 0;

        foreach (var character in argument)
        {
            if (character == '\\')
            {
                backslashes++;
                continue;
            }

            if (character == '"')
            {
                quoted += new string('\\', (backslashes * 2) + 1);
                quoted += character;
                backslashes = 0;
                continue;
            }

            quoted += new string('\\', backslashes);
            quoted += character;
            backslashes = 0;
        }

        quoted += new string('\\', backslashes * 2);
        quoted += "\"";

        return quoted;
    }

    private void btnAzureAccessToken_Click(object sender, EventArgs e)
    {
        using var dialog = new AzureAccessTokenDialog();
        dialog.ShowDialog();
        txtAuthorizationHeader.Text = $"Bearer {dialog.AccessToken}";
    }
}
