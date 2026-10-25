//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrExportDmabufManagerV1 { version: u32 },
    ZwlrExportDmabufFrameV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrExportDmabufManagerV1 { .. } => "zwlr_export_dmabuf_manager_v1",
            Self::ZwlrExportDmabufFrameV1 { .. } => "zwlr_export_dmabuf_frame_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrExportDmabufManagerV1 { version } => version,
            Self::ZwlrExportDmabufFrameV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrExportDmabufManagerV1 { .. } => 1,
            Self::ZwlrExportDmabufFrameV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_export_dmabuf_manager_v1" => Self::ZwlrExportDmabufManagerV1 { version },
            "zwlr_export_dmabuf_frame_v1" => Self::ZwlrExportDmabufFrameV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
