using System;
using System.Windows.Forms;

namespace HttpGenerator.VSIX
{
    public partial class AzureAccessTokenDialog : Form
    {
        public AzureAccessTokenDialog()
        {
            InitializeComponent();
        }

        public string? TenantId { get; private set; }

        public string Scope { get; private set; } = string.Empty;

        private void OnCancel_Click(object sender, EventArgs e)
        {
            Close();
        }

        private void OnOk_Click(object sender, EventArgs e)
        {
            TenantId = string.IsNullOrWhiteSpace(txtTenantId.Text)
                ? null
                : txtTenantId.Text;
            Scope = txtScope.Text;
            DialogResult = DialogResult.OK;
            Close();
        }
    }
}
