//! Identifies Image capture source interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtImageCaptureSourceV1 { version: u32 },
    ExtOutputImageCaptureSourceManagerV1 { version: u32 },
    ExtForeignToplevelImageCaptureSourceManagerV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtImageCaptureSourceV1 { .. } => "ext_image_capture_source_v1",
            Self::ExtOutputImageCaptureSourceManagerV1 { .. } => {
                "ext_output_image_capture_source_manager_v1"
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 { .. } => {
                "ext_foreign_toplevel_image_capture_source_manager_v1"
            }
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtImageCaptureSourceV1 { version } => version,
            Self::ExtOutputImageCaptureSourceManagerV1 { version } => version,
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtImageCaptureSourceV1 { .. } => 1,
            Self::ExtOutputImageCaptureSourceManagerV1 { .. } => 1,
            Self::ExtForeignToplevelImageCaptureSourceManagerV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_image_capture_source_v1" => Self::ExtImageCaptureSourceV1 { version },
            "ext_output_image_capture_source_manager_v1" => {
                Self::ExtOutputImageCaptureSourceManagerV1 { version }
            }
            "ext_foreign_toplevel_image_capture_source_manager_v1" => {
                Self::ExtForeignToplevelImageCaptureSourceManagerV1 { version }
            }
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
