//! Identifies Image copy capture interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtImageCopyCaptureManagerV1 { version: u32 },
    ExtImageCopyCaptureSessionV1 { version: u32 },
    ExtImageCopyCaptureFrameV1 { version: u32 },
    ExtImageCopyCaptureCursorSessionV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtImageCopyCaptureManagerV1 { .. } => "ext_image_copy_capture_manager_v1",
            Self::ExtImageCopyCaptureSessionV1 { .. } => "ext_image_copy_capture_session_v1",
            Self::ExtImageCopyCaptureFrameV1 { .. } => "ext_image_copy_capture_frame_v1",
            Self::ExtImageCopyCaptureCursorSessionV1 { .. } => {
                "ext_image_copy_capture_cursor_session_v1"
            }
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtImageCopyCaptureManagerV1 { version } => version,
            Self::ExtImageCopyCaptureSessionV1 { version } => version,
            Self::ExtImageCopyCaptureFrameV1 { version } => version,
            Self::ExtImageCopyCaptureCursorSessionV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtImageCopyCaptureManagerV1 { .. } => 1,
            Self::ExtImageCopyCaptureSessionV1 { .. } => 1,
            Self::ExtImageCopyCaptureFrameV1 { .. } => 1,
            Self::ExtImageCopyCaptureCursorSessionV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_image_copy_capture_manager_v1" => Self::ExtImageCopyCaptureManagerV1 { version },
            "ext_image_copy_capture_session_v1" => Self::ExtImageCopyCaptureSessionV1 { version },
            "ext_image_copy_capture_frame_v1" => Self::ExtImageCopyCaptureFrameV1 { version },
            "ext_image_copy_capture_cursor_session_v1" => {
                Self::ExtImageCopyCaptureCursorSessionV1 { version }
            }
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
