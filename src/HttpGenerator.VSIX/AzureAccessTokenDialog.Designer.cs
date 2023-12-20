namespace HttpGenerator.VSIX
{
    partial class AzureAccessTokenDialog
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
            this.btnCancel = new System.Windows.Forms.Button();
            this.btnOk = new System.Windows.Forms.Button();
            this.txtScope = new System.Windows.Forms.TextBox();
            this.lblScope = new System.Windows.Forms.Label();
            this.txtTenantId = new System.Windows.Forms.TextBox();
            this.lblTenant = new System.Windows.Forms.Label();
            this.SuspendLayout();
            // 
            // btnCancel
            // 
            this.btnCancel.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnCancel.DialogResult = System.Windows.Forms.DialogResult.Cancel;
            this.btnCancel.Location = new System.Drawing.Point(317, 73);
            this.btnCancel.Name = "btnCancel";
            this.btnCancel.Size = new System.Drawing.Size(75, 23);
            this.btnCancel.TabIndex = 11;
            this.btnCancel.Text = "Cancel";
            this.btnCancel.UseVisualStyleBackColor = true;
            this.btnCancel.Click += new System.EventHandler(this.OnCancel_Click);
            // 
            // btnOk
            // 
            this.btnOk.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnOk.Location = new System.Drawing.Point(236, 73);
            this.btnOk.Name = "btnOk";
            this.btnOk.Size = new System.Drawing.Size(75, 23);
            this.btnOk.TabIndex = 10;
            this.btnOk.Text = "OK";
            this.btnOk.UseVisualStyleBackColor = true;
            this.btnOk.Click += new System.EventHandler(this.OnOk_Click);
            // 
            // txtScope
            // 
            this.txtScope.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtScope.Location = new System.Drawing.Point(73, 38);
            this.txtScope.Name = "txtScope";
            this.txtScope.Size = new System.Drawing.Size(319, 20);
            this.txtScope.TabIndex = 0;
            // 
            // lblScope
            // 
            this.lblScope.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblScope.AutoSize = true;
            this.lblScope.Location = new System.Drawing.Point(29, 41);
            this.lblScope.Name = "lblScope";
            this.lblScope.Size = new System.Drawing.Size(38, 13);
            this.lblScope.TabIndex = 8;
            this.lblScope.Text = "Scope";
            // 
            // txtTenantId
            // 
            this.txtTenantId.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.txtTenantId.Location = new System.Drawing.Point(73, 12);
            this.txtTenantId.Name = "txtTenantId";
            this.txtTenantId.Size = new System.Drawing.Size(319, 20);
            this.txtTenantId.TabIndex = 12;
            // 
            // lblTenant
            // 
            this.lblTenant.Anchor = ((System.Windows.Forms.AnchorStyles)(((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left) 
            | System.Windows.Forms.AnchorStyles.Right)));
            this.lblTenant.AutoSize = true;
            this.lblTenant.Location = new System.Drawing.Point(12, 15);
            this.lblTenant.Name = "lblTenant";
            this.lblTenant.Size = new System.Drawing.Size(55, 13);
            this.lblTenant.TabIndex = 13;
            this.lblTenant.Text = "Tenant ID";
            // 
            // AzureAccessTokenDialog
            // 
            this.AcceptButton = this.btnOk;
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.CancelButton = this.btnCancel;
            this.ClientSize = new System.Drawing.Size(404, 108);
            this.Controls.Add(this.txtTenantId);
            this.Controls.Add(this.lblTenant);
            this.Controls.Add(this.btnCancel);
            this.Controls.Add(this.btnOk);
            this.Controls.Add(this.txtScope);
            this.Controls.Add(this.lblScope);
            this.MinimumSize = new System.Drawing.Size(420, 147);
            this.Name = "AzureAccessTokenDialog";
            this.Text = "Azure Access Token";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Button btnCancel;
        private System.Windows.Forms.Button btnOk;
        private System.Windows.Forms.TextBox txtScope;
        private System.Windows.Forms.Label lblScope;
        private System.Windows.Forms.TextBox txtTenantId;
        private System.Windows.Forms.Label lblTenant;
    }
}