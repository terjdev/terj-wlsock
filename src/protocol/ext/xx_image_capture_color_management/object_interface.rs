//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxImageCaptureColorManagerV1 { version: u32 },
    XxImageCaptureSourceColorsV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxImageCaptureColorManagerV1 { .. } => "xx_image_capture_color_manager_v1",
            Self::XxImageCaptureSourceColorsV1 { .. } => "xx_image_capture_source_colors_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxImageCaptureColorManagerV1 { version } => version,
            Self::XxImageCaptureSourceColorsV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxImageCaptureColorManagerV1 { .. } => 1,
            Self::XxImageCaptureSourceColorsV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_image_capture_color_manager_v1" => Self::XxImageCaptureColorManagerV1 { version },
            "xx_image_capture_source_colors_v1" => Self::XxImageCaptureSourceColorsV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
