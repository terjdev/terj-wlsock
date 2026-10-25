//! Identifies Single pixel buffer interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpSinglePixelBufferManagerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpSinglePixelBufferManagerV1 { .. } => "wp_single_pixel_buffer_manager_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpSinglePixelBufferManagerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpSinglePixelBufferManagerV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_single_pixel_buffer_manager_v1" => Self::WpSinglePixelBufferManagerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
