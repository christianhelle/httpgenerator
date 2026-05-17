using Azure.Core.Diagnostics;
using Azure.Core;
using Azure.Identity;
using System;
using System.ComponentModel;
using System.Diagnostics.CodeAnalysis;
using System.Threading;
using System.Threading.Tasks;
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

        public string AccessToken { get; private set; } = string.Empty;

        protected override void OnClosing(CancelEventArgs e)
        {
            base.OnClosing(e);
            cancellationTokenSource.Cancel();
            cancellationTokenSource.Dispose();
        }

        private void OnCancel_Click(object sender, EventArgs e)
        {
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
                var scope = txtScope.Text;

                var token = cancellationTokenSource.Token;
                using var listener = AzureEventSourceListener.CreateConsoleLogger();
                AccessToken = await TryGetAccessTokenAsync(tenantId, scope, token);
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

        private static async Task<string> TryGetAccessTokenAsync(
            string? tenantId,
            string scope,
            CancellationToken cancellationToken)
        {
            var request = new TokenRequestContext([scope], tenantId: tenantId);
            var credentials = new ChainedTokenCredential(
                new AzureCliCredential(),
                new VisualStudioCredential(),
                new DefaultAzureCredential(
                    new DefaultAzureCredentialOptions
                    {
                        ExcludeWorkloadIdentityCredential = true,
                        ExcludeManagedIdentityCredential = true,
                        ExcludeVisualStudioCredential = true,
                        ExcludeEnvironmentCredential = true,
                        ExcludeAzureCliCredential = true,
                    }));

            var token = await credentials.GetTokenAsync(request, cancellationToken);
            return token.Token;
        }
    }
}
