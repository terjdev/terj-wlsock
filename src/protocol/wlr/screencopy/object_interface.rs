//! Identifies WLR screencopy interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrScreencopyManagerV1 { version: u32 },
    ZwlrScreencopyFrameV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrScreencopyManagerV1 { .. } => "zwlr_screencopy_manager_v1",
            Self::ZwlrScreencopyFrameV1 { .. } => "zwlr_screencopy_frame_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrScreencopyManagerV1 { version } => version,
            Self::ZwlrScreencopyFrameV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrScreencopyManagerV1 { .. } => 3,
            Self::ZwlrScreencopyFrameV1 { .. } => 3,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_screencopy_manager_v1" => Self::ZwlrScreencopyManagerV1 { version },
            "zwlr_screencopy_frame_v1" => Self::ZwlrScreencopyFrameV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
