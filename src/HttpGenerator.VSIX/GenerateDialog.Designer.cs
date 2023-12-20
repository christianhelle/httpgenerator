namespace HttpGenerator.VSIX
{
    partial class GenerateDialog
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.lblOpenApiFile = new System.Windows.Forms.Label();
            this.txtOpenApiFile = new System.Windows.Forms.TextBox();
            this.txtOutputFolder = new System.Windows.Forms.TextBox();
            this.lblOutputFolder = new System.Windows.Forms.Label();
            this.folderBrowserDialog = new System.Windows.Forms.FolderBrowserDialog();
            this.btnSelectOutputFolder = new System.Windows.Forms.Button();
            this.btnSelectOpenApiFile = new System.Windows.Forms.Button();
            this.btnOk = new System.Windows.Forms.Button();
            this.btnCancel = new System.Windows.Forms.Button();
            this.openFileDialog = new System.Windows.Forms.OpenFileDialog();
            this.txtBaseUrl = new System.Windows.Forms.TextBox();
            this.lblBaseUrl = new System.Windows.Forms.Label();
            this.txtContentType = new System.Windows.Forms.TextBox();
            this.lblContentType = new System.Windows.Forms.Label();
            this.txtAuthorizationHeader = new System.Windows.Forms.TextBox();
            this.lblAuthorizationHeader = new System.Windows.Forms.Label();
            this.chkMultipleFiles = new System.Windows.Forms.CheckBox();
            this.btnAzureAccessToken = new System.Windows.Forms.Button();
            this.SuspendLayout();
            // 
            // lblOpenApiFile
            // 
            this.lblOpenApiFile.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblOpenApiFile.AutoSize = true;
            this.lblOpenApiFile.Location = new System.Drawing.Point(12, 22);
            this.lblOpenApiFile.Name = "lblOpenApiFile";
            this.lblOpenApiFile.Size = new System.Drawing.Size(119, 13);
            this.lblOpenApiFile.TabIndex = 0;
            this.lblOpenApiFile.Text = "OpenAPI Specifications";
            // 
            // txtOpenApiFile
            // 
            this.txtOpenApiFile.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtOpenApiFile.Location = new System.Drawing.Point(137, 19);
            this.txtOpenApiFile.Name = "txtOpenApiFile";
            this.txtOpenApiFile.Size = new System.Drawing.Size(623, 20);
            this.txtOpenApiFile.TabIndex = 1;
            // 
            // txtOutputFolder
            // 
            this.txtOutputFolder.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtOutputFolder.Location = new System.Drawing.Point(137, 45);
            this.txtOutputFolder.Name = "txtOutputFolder";
            this.txtOutputFolder.Size = new System.Drawing.Size(623, 20);
            this.txtOutputFolder.TabIndex = 3;
            // 
            // lblOutputFolder
            // 
            this.lblOutputFolder.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblOutputFolder.AutoSize = true;
            this.lblOutputFolder.Location = new System.Drawing.Point(63, 48);
            this.lblOutputFolder.Name = "lblOutputFolder";
            this.lblOutputFolder.Size = new System.Drawing.Size(68, 13);
            this.lblOutputFolder.TabIndex = 2;
            this.lblOutputFolder.Text = "Output folder";
            this.lblOutputFolder.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // btnSelectOutputFolder
            // 
            this.btnSelectOutputFolder.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnSelectOutputFolder.Location = new System.Drawing.Point(766, 44);
            this.btnSelectOutputFolder.Name = "btnSelectOutputFolder";
            this.btnSelectOutputFolder.Size = new System.Drawing.Size(26, 21);
            this.btnSelectOutputFolder.TabIndex = 4;
            this.btnSelectOutputFolder.Text = "...";
            this.btnSelectOutputFolder.UseVisualStyleBackColor = true;
            this.btnSelectOutputFolder.Click += new System.EventHandler(this.OnSelectOutputFolder_Click);
            // 
            // btnSelectOpenApiFile
            // 
            this.btnSelectOpenApiFile.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnSelectOpenApiFile.Location = new System.Drawing.Point(766, 18);
            this.btnSelectOpenApiFile.Name = "btnSelectOpenApiFile";
            this.btnSelectOpenApiFile.Size = new System.Drawing.Size(26, 21);
            this.btnSelectOpenApiFile.TabIndex = 5;
            this.btnSelectOpenApiFile.Text = "...";
            this.btnSelectOpenApiFile.UseVisualStyleBackColor = true;
            this.btnSelectOpenApiFile.Click += new System.EventHandler(this.OnSelectOpenApiFile_Click);
            // 
            // btnOk
            // 
            this.btnOk.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnOk.Location = new System.Drawing.Point(604, 321);
            this.btnOk.Name = "btnOk";
            this.btnOk.Size = new System.Drawing.Size(75, 23);
            this.btnOk.TabIndex = 6;
            this.btnOk.Text = "OK";
            this.btnOk.UseVisualStyleBackColor = true;
            this.btnOk.Click += new System.EventHandler(this.OnOk_Click);
            // 
            // btnCancel
            // 
            this.btnCancel.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnCancel.DialogResult = System.Windows.Forms.DialogResult.Cancel;
            this.btnCancel.Location = new System.Drawing.Point(685, 321);
            this.btnCancel.Name = "btnCancel";
            this.btnCancel.Size = new System.Drawing.Size(75, 23);
            this.btnCancel.TabIndex = 7;
            this.btnCancel.Text = "Cancel";
            this.btnCancel.UseVisualStyleBackColor = true;
            this.btnCancel.Click += new System.EventHandler(this.OnCancel_Click);
            // 
            // txtBaseUrl
            // 
            this.txtBaseUrl.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtBaseUrl.Location = new System.Drawing.Point(137, 71);
            this.txtBaseUrl.Name = "txtBaseUrl";
            this.txtBaseUrl.Size = new System.Drawing.Size(623, 20);
            this.txtBaseUrl.TabIndex = 9;
            // 
            // lblBaseUrl
            // 
            this.lblBaseUrl.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblBaseUrl.AutoSize = true;
            this.lblBaseUrl.Location = new System.Drawing.Point(75, 74);
            this.lblBaseUrl.Name = "lblBaseUrl";
            this.lblBaseUrl.Size = new System.Drawing.Size(56, 13);
            this.lblBaseUrl.TabIndex = 8;
            this.lblBaseUrl.Text = "Base URL";
            this.lblBaseUrl.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // txtContentType
            // 
            this.txtContentType.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtContentType.Location = new System.Drawing.Point(137, 97);
            this.txtContentType.Name = "txtContentType";
            this.txtContentType.Size = new System.Drawing.Size(623, 20);
            this.txtContentType.TabIndex = 11;
            this.txtContentType.Text = "application/json";
            // 
            // lblContentType
            // 
            this.lblContentType.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblContentType.AutoSize = true;
            this.lblContentType.Location = new System.Drawing.Point(63, 100);
            this.lblContentType.Name = "lblContentType";
            this.lblContentType.Size = new System.Drawing.Size(67, 13);
            this.lblContentType.TabIndex = 10;
            this.lblContentType.Text = "Content type";
            this.lblContentType.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // txtAuthorizationHeader
            // 
            this.txtAuthorizationHeader.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom) 
            | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtAuthorizationHeader.Location = new System.Drawing.Point(137, 123);
            this.txtAuthorizationHeader.Multiline = true;
            this.txtAuthorizationHeader.Name = "txtAuthorizationHeader";
            this.txtAuthorizationHeader.Size = new System.Drawing.Size(623, 181);
            this.txtAuthorizationHeader.TabIndex = 13;
            // 
            // lblAuthorizationHeader
            // 
            this.lblAuthorizationHeader.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblAuthorizationHeader.AutoSize = true;
            this.lblAuthorizationHeader.Location = new System.Drawing.Point(25, 126);
            this.lblAuthorizationHeader.Name = "lblAuthorizationHeader";
            this.lblAuthorizationHeader.Size = new System.Drawing.Size(106, 13);
            this.lblAuthorizationHeader.TabIndex = 12;
            this.lblAuthorizationHeader.Text = "Authorization Header";
            this.lblAuthorizationHeader.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // chkMultipleFiles
            // 
            this.chkMultipleFiles.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left)));
            this.chkMultipleFiles.AutoSize = true;
            this.chkMultipleFiles.Checked = true;
            this.chkMultipleFiles.CheckState = System.Windows.Forms.CheckState.Checked;
            this.chkMultipleFiles.Location = new System.Drawing.Point(137, 310);
            this.chkMultipleFiles.Name = "chkMultipleFiles";
            this.chkMultipleFiles.Size = new System.Drawing.Size(166, 17);
            this.chkMultipleFiles.TabIndex = 14;
            this.chkMultipleFiles.Text = "Generate .http file per request";
            this.chkMultipleFiles.UseVisualStyleBackColor = true;
            // 
            // btnAzureAccessToken
            // 
            this.btnAzureAccessToken.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.btnAzureAccessToken.Location = new System.Drawing.Point(766, 123);
            this.btnAzureAccessToken.Name = "btnAzureAccessToken";
            this.btnAzureAccessToken.Size = new System.Drawing.Size(26, 21);
            this.btnAzureAccessToken.TabIndex = 15;
            this.btnAzureAccessToken.Text = "...";
            this.btnAzureAccessToken.UseVisualStyleBackColor = true;
            this.btnAzureAccessToken.Click += new System.EventHandler(this.btnAzureAccessToken_Click);
            // 
            // GenerateDialog
            // 
            this.AcceptButton = this.btnOk;
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.CancelButton = this.btnCancel;
            this.ClientSize = new System.Drawing.Size(804, 355);
            this.Controls.Add(this.btnAzureAccessToken);
            this.Controls.Add(this.chkMultipleFiles);
            this.Controls.Add(this.txtAuthorizationHeader);
            this.Controls.Add(this.lblAuthorizationHeader);
            this.Controls.Add(this.txtContentType);
            this.Controls.Add(this.lblContentType);
            this.Controls.Add(this.txtBaseUrl);
            this.Controls.Add(this.lblBaseUrl);
            this.Controls.Add(this.btnCancel);
            this.Controls.Add(this.btnOk);
            this.Controls.Add(this.btnSelectOpenApiFile);
            this.Controls.Add(this.btnSelectOutputFolder);
            this.Controls.Add(this.txtOutputFolder);
            this.Controls.Add(this.lblOutputFolder);
            this.Controls.Add(this.txtOpenApiFile);
            this.Controls.Add(this.lblOpenApiFile);
            this.Name = "GenerateDialog";
            this.Text = "HTTP File Generator";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label lblOpenApiFile;
        private System.Windows.Forms.TextBox txtOpenApiFile;
        private System.Windows.Forms.TextBox txtOutputFolder;
        private System.Windows.Forms.Label lblOutputFolder;
        private System.Windows.Forms.FolderBrowserDialog folderBrowserDialog;
        private System.Windows.Forms.Button btnSelectOutputFolder;
        private System.Windows.Forms.Button btnSelectOpenApiFile;
        private System.Windows.Forms.Button btnOk;
        private System.Windows.Forms.Button btnCancel;
        private System.Windows.Forms.OpenFileDialog openFileDialog;
        private System.Windows.Forms.TextBox txtBaseUrl;
        private System.Windows.Forms.Label lblBaseUrl;
        private System.Windows.Forms.TextBox txtContentType;
        private System.Windows.Forms.Label lblContentType;
        private System.Windows.Forms.TextBox txtAuthorizationHeader;
        private System.Windows.Forms.Label lblAuthorizationHeader;
        private System.Windows.Forms.CheckBox chkMultipleFiles;
        private System.Windows.Forms.Button btnAzureAccessToken;
    }
}