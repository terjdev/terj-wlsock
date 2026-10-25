//! Identifies Xwayland shell interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XwaylandShellV1 { version: u32 },
    XwaylandSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XwaylandShellV1 { .. } => "xwayland_shell_v1",
            Self::XwaylandSurfaceV1 { .. } => "xwayland_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XwaylandShellV1 { version } => version,
            Self::XwaylandSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XwaylandShellV1 { .. } => 1,
            Self::XwaylandSurfaceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xwayland_shell_v1" => Self::XwaylandShellV1 { version },
            "xwayland_surface_v1" => Self::XwaylandSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
