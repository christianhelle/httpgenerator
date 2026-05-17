using System.Windows.Forms;

namespace HttpGenerator.VSIX.Dialogs;

public partial class InputUrlDialog : Form
{
    private TextBox _txtUrl;
    private Button _btnOk;
    private Button _btnCancel;

    public string? Url => _txtUrl.Text;

    public InputUrlDialog()
    {
        InitializeComponent();
    }

    private void InitializeComponent()
    {
        this.Text = "Enter OpenAPI URL";
        this.Width = 600;
        this.Height = 140;
        this.FormBorderStyle = FormBorderStyle.FixedDialog;
        this.StartPosition = FormStartPosition.CenterParent;
        this.MaximizeBox = false;
        this.MinimizeBox = false;

        _txtUrl = new TextBox { Left = 12, Top = 12, Width = 560 };
        _btnOk = new Button { Text = "OK", Left = 412, Width = 75, Top = 44, DialogResult = DialogResult.OK };
        _btnCancel = new Button { Text = "Cancel", Left = 497, Width = 75, Top = 44, DialogResult = DialogResult.Cancel };

        this.Controls.Add(_txtUrl);
        this.Controls.Add(_btnOk);
        this.Controls.Add(_btnCancel);

        this.AcceptButton = _btnOk;
        this.CancelButton = _btnCancel;
    }
}
