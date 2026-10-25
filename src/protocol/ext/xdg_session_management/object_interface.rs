//! Identifies Xdg session management interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgSessionManagerV1 { version: u32 },
    XdgSessionV1 { version: u32 },
    XdgToplevelSessionV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgSessionManagerV1 { .. } => "xdg_session_manager_v1",
            Self::XdgSessionV1 { .. } => "xdg_session_v1",
            Self::XdgToplevelSessionV1 { .. } => "xdg_toplevel_session_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgSessionManagerV1 { version } => version,
            Self::XdgSessionV1 { version } => version,
            Self::XdgToplevelSessionV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgSessionManagerV1 { .. } => 1,
            Self::XdgSessionV1 { .. } => 1,
            Self::XdgToplevelSessionV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_session_manager_v1" => Self::XdgSessionManagerV1 { version },
            "xdg_session_v1" => Self::XdgSessionV1 { version },
            "xdg_toplevel_session_v1" => Self::XdgToplevelSessionV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
