//! Identifies Xdg toplevel icon interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgToplevelIconManagerV1 { version: u32 },
    XdgToplevelIconV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgToplevelIconManagerV1 { .. } => "xdg_toplevel_icon_manager_v1",
            Self::XdgToplevelIconV1 { .. } => "xdg_toplevel_icon_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgToplevelIconManagerV1 { version } => version,
            Self::XdgToplevelIconV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgToplevelIconManagerV1 { .. } => 1,
            Self::XdgToplevelIconV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_toplevel_icon_manager_v1" => Self::XdgToplevelIconManagerV1 { version },
            "xdg_toplevel_icon_v1" => Self::XdgToplevelIconV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
