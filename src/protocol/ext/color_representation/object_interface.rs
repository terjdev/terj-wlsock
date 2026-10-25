//! Identifies Color representation interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpColorRepresentationManagerV1 { version: u32 },
    WpColorRepresentationSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpColorRepresentationManagerV1 { .. } => "wp_color_representation_manager_v1",
            Self::WpColorRepresentationSurfaceV1 { .. } => "wp_color_representation_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpColorRepresentationManagerV1 { version } => version,
            Self::WpColorRepresentationSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpColorRepresentationManagerV1 { .. } => 1,
            Self::WpColorRepresentationSurfaceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_color_representation_manager_v1" => {
                Self::WpColorRepresentationManagerV1 { version }
            }
            "wp_color_representation_surface_v1" => {
                Self::WpColorRepresentationSurfaceV1 { version }
            }
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
