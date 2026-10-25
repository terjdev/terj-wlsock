//! Identifies Xdg toplevel drag interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgToplevelDragManagerV1 { version: u32 },
    XdgToplevelDragV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgToplevelDragManagerV1 { .. } => "xdg_toplevel_drag_manager_v1",
            Self::XdgToplevelDragV1 { .. } => "xdg_toplevel_drag_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgToplevelDragManagerV1 { version } => version,
            Self::XdgToplevelDragV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgToplevelDragManagerV1 { .. } => 1,
            Self::XdgToplevelDragV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_toplevel_drag_manager_v1" => Self::XdgToplevelDragManagerV1 { version },
            "xdg_toplevel_drag_v1" => Self::XdgToplevelDragV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
