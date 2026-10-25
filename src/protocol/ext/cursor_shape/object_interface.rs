//! Identifies Cursor shape interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpCursorShapeManagerV1 { version: u32 },
    WpCursorShapeDeviceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpCursorShapeManagerV1 { .. } => "wp_cursor_shape_manager_v1",
            Self::WpCursorShapeDeviceV1 { .. } => "wp_cursor_shape_device_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpCursorShapeManagerV1 { version } => version,
            Self::WpCursorShapeDeviceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpCursorShapeManagerV1 { .. } => 2,
            Self::WpCursorShapeDeviceV1 { .. } => 2,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_cursor_shape_manager_v1" => Self::WpCursorShapeManagerV1 { version },
            "wp_cursor_shape_device_v1" => Self::WpCursorShapeDeviceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
