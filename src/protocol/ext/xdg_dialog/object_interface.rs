//! Identifies Xdg dialog interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgWmDialogV1 { version: u32 },
    XdgDialogV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgWmDialogV1 { .. } => "xdg_wm_dialog_v1",
            Self::XdgDialogV1 { .. } => "xdg_dialog_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgWmDialogV1 { version } => version,
            Self::XdgDialogV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgWmDialogV1 { .. } => 1,
            Self::XdgDialogV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_wm_dialog_v1" => Self::XdgWmDialogV1 { version },
            "xdg_dialog_v1" => Self::XdgDialogV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
