//! Identifies WLR layer shell interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrLayerShellV1 { version: u32 },
    ZwlrLayerSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrLayerShellV1 { .. } => "zwlr_layer_shell_v1",
            Self::ZwlrLayerSurfaceV1 { .. } => "zwlr_layer_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrLayerShellV1 { version } => version,
            Self::ZwlrLayerSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrLayerShellV1 { .. } => 5,
            Self::ZwlrLayerSurfaceV1 { .. } => 5,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_layer_shell_v1" => Self::ZwlrLayerShellV1 { version },
            "zwlr_layer_surface_v1" => Self::ZwlrLayerSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
