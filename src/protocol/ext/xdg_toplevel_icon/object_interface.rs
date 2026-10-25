//! Identifies Xdg toplevel icon interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XdgToplevelIconManagerV1 { version: u32 },
    XdgToplevelIconV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XdgToplevelIconManagerV1 { .. } => "xdg_toplevel_icon_manager_v1",
            Self::XdgToplevelIconV1 { .. } => "xdg_toplevel_icon_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XdgToplevelIconManagerV1 { version } => version,
            Self::XdgToplevelIconV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XdgToplevelIconManagerV1 { .. } => 1,
            Self::XdgToplevelIconV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_toplevel_icon_manager_v1" => Self::XdgToplevelIconManagerV1 { version },
            "xdg_toplevel_icon_v1" => Self::XdgToplevelIconV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
