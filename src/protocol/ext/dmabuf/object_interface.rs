//! Identifies Linux DMA-BUF interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpLinuxDmabufV1 { version: u32 },
    ZwpLinuxBufferParamsV1 { version: u32 },
    ZwpLinuxDmabufFeedbackV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpLinuxDmabufV1 { .. } => "zwp_linux_dmabuf_v1",
            Self::ZwpLinuxBufferParamsV1 { .. } => "zwp_linux_buffer_params_v1",
            Self::ZwpLinuxDmabufFeedbackV1 { .. } => "zwp_linux_dmabuf_feedback_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpLinuxDmabufV1 { version } => version,
            Self::ZwpLinuxBufferParamsV1 { version } => version,
            Self::ZwpLinuxDmabufFeedbackV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpLinuxDmabufV1 { .. } => 6,
            Self::ZwpLinuxBufferParamsV1 { .. } => 6,
            Self::ZwpLinuxDmabufFeedbackV1 { .. } => 6,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_linux_dmabuf_v1" => Self::ZwpLinuxDmabufV1 { version },
            "zwp_linux_buffer_params_v1" => Self::ZwpLinuxBufferParamsV1 { version },
            "zwp_linux_dmabuf_feedback_v1" => Self::ZwpLinuxDmabufFeedbackV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
