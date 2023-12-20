using Azure.Core;
using Azure.Identity;
using System;
using System.Diagnostics.CodeAnalysis;
using System.Threading;
using System.Windows.Forms;

namespace HttpGenerator.VSIX
{
    public partial class AzureAccessTokenDialog : Form
    {
        private readonly CancellationTokenSource cancellationTokenSource = new();

        public AzureAccessTokenDialog()
        {
            InitializeComponent();

        }

        public string AccessToken { get; private set; }

        private void OnCancel_Click(object sender, EventArgs e)
        {
            cancellationTokenSource.Cancel();
            Close();
        }

        [SuppressMessage(
            "Usage",
            "VSTHRD100:Avoid async void methods",
            Justification = "<Pending>")]
        private async void OnOk_Click(object sender, EventArgs e)
        {
            try
            {
                Text = "Acquiring Azure Entra ID Access Token...";
                txtTenantId.Enabled = false;
                txtScope.Enabled = false;
                btnOk.Enabled = false;

                var tenantId = string.IsNullOrWhiteSpace(txtTenantId.Text) ? null : txtTenantId.Text;
                var request = new TokenRequestContext([txtScope.Text], tenantId: tenantId);
                var credentials = new DefaultAzureCredential(
                    new DefaultAzureCredentialOptions
                    {
                        ExcludeInteractiveBrowserCredential = true,
                        ExcludeManagedIdentityCredential = true,
                        ExcludeEnvironmentCredential = true
                    });
                var token = await credentials.GetTokenAsync(request, cancellationTokenSource.Token);
                AccessToken = token.Token;
                Close();
            }
            catch (OperationCanceledException)
            {
                // Ignore
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
            finally
            {
                Text = "Azure Access Token";
                txtTenantId.Enabled = true;
                txtScope.Enabled = true;
                btnOk.Enabled = true;
            }
        }
    }
}
