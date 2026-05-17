using System.Windows.Forms;
using HttpGenerator.VSIX.Settings;

namespace HttpGenerator.VSIX.Dialogs;

public partial class SettingsDialog : Form
{
    private CheckBox _chkTelemetry;
    private Button _btnOk;
    private Button _btnCancel;

    public TelemetryOptions Options { get; private set; }

    public SettingsDialog(TelemetryOptions options)
    {
        Options = options;
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        this.Text = "HTTP File Generator Settings";
        this.Width = 400;
        this.Height = 140;
        this.FormBorderStyle = FormBorderStyle.FixedDialog;
        this.StartPosition = FormStartPosition.CenterParent;
        this.MaximizeBox = false;
        this.MinimizeBox = false;

        _chkTelemetry = new CheckBox { Left = 12, Top = 12, Width = 360, Text = "Disable telemetry (opt-out)" };
        _chkTelemetry.Checked = Options.TelemetryOptOut;

        _btnOk = new Button { Text = "OK", Left = 212, Width = 75, Top = 44, DialogResult = DialogResult.OK };
        _btnCancel = new Button { Text = "Cancel", Left = 297, Width = 75, Top = 44, DialogResult = DialogResult.Cancel };

        this.Controls.Add(_chkTelemetry);
        this.Controls.Add(_btnOk);
        this.Controls.Add(_btnCancel);

        this.AcceptButton = _btnOk;
        this.CancelButton = _btnCancel;

        _btnOk.Click += OnOk;
    }

    private void OnOk(object? sender, EventArgs e)
    {
        Options.TelemetryOptOut = _chkTelemetry.Checked;
    }
}
