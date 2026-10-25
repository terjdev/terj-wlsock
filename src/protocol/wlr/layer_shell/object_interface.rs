//! Identifies WLR layer shell interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrLayerShellV1 { version: u32 },
    ZwlrLayerSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrLayerShellV1 { .. } => "zwlr_layer_shell_v1",
            Self::ZwlrLayerSurfaceV1 { .. } => "zwlr_layer_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrLayerShellV1 { version } => version,
            Self::ZwlrLayerSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrLayerShellV1 { .. } => 5,
            Self::ZwlrLayerSurfaceV1 { .. } => 5,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_layer_shell_v1" => Self::ZwlrLayerShellV1 { version },
            "zwlr_layer_surface_v1" => Self::ZwlrLayerSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
