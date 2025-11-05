use serde::{Deserialize, Serialize};

/// ServerNode holds the editable fields shown in `details_dlg`.
/// Each field maps 1:1 to a control in the dialog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerNode {
    /// "Remarks" text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// "Tunnel Path" text
    pub tunnel_path: String,
    /// "Disable TLS" checkbox
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_tls: Option<bool>,
    /// "Client ID" text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// "Server Host" text
    pub server_host: String,
    /// "Server Port" spin control (logical model uses u16)
    pub server_port: u16,
    /// "Server Domain" text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_domain: Option<String>,
    /// "CA File/Content" text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_file: Option<String>,
    /// "Dangerous Mode" checkbox
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dangerous_mode: Option<bool>,
}

impl ServerNode {
    /// Create a new ServerNode with sensible defaults.
    /// Equivalent to `Default::default()`.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}
