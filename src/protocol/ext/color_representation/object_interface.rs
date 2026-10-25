//! Identifies Color representation interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpColorRepresentationManagerV1 { version: u32 },
    WpColorRepresentationSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpColorRepresentationManagerV1 { .. } => "wp_color_representation_manager_v1",
            Self::WpColorRepresentationSurfaceV1 { .. } => "wp_color_representation_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpColorRepresentationManagerV1 { version } => version,
            Self::WpColorRepresentationSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpColorRepresentationManagerV1 { .. } => 1,
            Self::WpColorRepresentationSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
