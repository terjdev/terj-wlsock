//! Identifies Content type interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpContentTypeManagerV1 { version: u32 },
    WpContentTypeV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpContentTypeManagerV1 { .. } => "wp_content_type_manager_v1",
            Self::WpContentTypeV1 { .. } => "wp_content_type_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpContentTypeManagerV1 { version } => version,
            Self::WpContentTypeV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpContentTypeManagerV1 { .. } => 1,
            Self::WpContentTypeV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_content_type_manager_v1" => Self::WpContentTypeManagerV1 { version },
            "wp_content_type_v1" => Self::WpContentTypeV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
