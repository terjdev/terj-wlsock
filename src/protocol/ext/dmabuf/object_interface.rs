//! Identifies Linux DMA-BUF interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpLinuxDmabufV1 { version: u32 },
    ZwpLinuxBufferParamsV1 { version: u32 },
    ZwpLinuxDmabufFeedbackV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpLinuxDmabufV1 { .. } => "zwp_linux_dmabuf_v1",
            Self::ZwpLinuxBufferParamsV1 { .. } => "zwp_linux_buffer_params_v1",
            Self::ZwpLinuxDmabufFeedbackV1 { .. } => "zwp_linux_dmabuf_feedback_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpLinuxDmabufV1 { version } => version,
            Self::ZwpLinuxBufferParamsV1 { version } => version,
            Self::ZwpLinuxDmabufFeedbackV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpLinuxDmabufV1 { .. } => 6,
            Self::ZwpLinuxBufferParamsV1 { .. } => 6,
            Self::ZwpLinuxDmabufFeedbackV1 { .. } => 6,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_linux_dmabuf_v1" => Self::ZwpLinuxDmabufV1 { version },
            "zwp_linux_buffer_params_v1" => Self::ZwpLinuxBufferParamsV1 { version },
            "zwp_linux_dmabuf_feedback_v1" => Self::ZwpLinuxDmabufFeedbackV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
