//! Identifies Xwayland shell interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XwaylandShellV1 { version: u32 },
    XwaylandSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XwaylandShellV1 { .. } => "xwayland_shell_v1",
            Self::XwaylandSurfaceV1 { .. } => "xwayland_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XwaylandShellV1 { version } => version,
            Self::XwaylandSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XwaylandShellV1 { .. } => 1,
            Self::XwaylandSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xwayland_shell_v1" => Self::XwaylandShellV1 { version },
            "xwayland_surface_v1" => Self::XwaylandSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
