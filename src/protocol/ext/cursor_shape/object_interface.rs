//! Identifies Cursor shape interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpCursorShapeManagerV1 { version: u32 },
    WpCursorShapeDeviceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpCursorShapeManagerV1 { .. } => "wp_cursor_shape_manager_v1",
            Self::WpCursorShapeDeviceV1 { .. } => "wp_cursor_shape_device_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpCursorShapeManagerV1 { version } => version,
            Self::WpCursorShapeDeviceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpCursorShapeManagerV1 { .. } => 2,
            Self::WpCursorShapeDeviceV1 { .. } => 2,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_cursor_shape_manager_v1" => Self::WpCursorShapeManagerV1 { version },
            "wp_cursor_shape_device_v1" => Self::WpCursorShapeDeviceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
