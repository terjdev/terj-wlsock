//! Identifies Xdg dialog interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XdgWmDialogV1 { version: u32 },
    XdgDialogV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XdgWmDialogV1 { .. } => "xdg_wm_dialog_v1",
            Self::XdgDialogV1 { .. } => "xdg_dialog_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XdgWmDialogV1 { version } => version,
            Self::XdgDialogV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XdgWmDialogV1 { .. } => 1,
            Self::XdgDialogV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_wm_dialog_v1" => Self::XdgWmDialogV1 { version },
            "xdg_dialog_v1" => Self::XdgDialogV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
