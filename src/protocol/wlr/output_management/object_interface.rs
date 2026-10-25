//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrOutputManagerV1 { version: u32 },
    ZwlrOutputHeadV1 { version: u32 },
    ZwlrOutputModeV1 { version: u32 },
    ZwlrOutputConfigurationV1 { version: u32 },
    ZwlrOutputConfigurationHeadV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrOutputManagerV1 { .. } => "zwlr_output_manager_v1",
            Self::ZwlrOutputHeadV1 { .. } => "zwlr_output_head_v1",
            Self::ZwlrOutputModeV1 { .. } => "zwlr_output_mode_v1",
            Self::ZwlrOutputConfigurationV1 { .. } => "zwlr_output_configuration_v1",
            Self::ZwlrOutputConfigurationHeadV1 { .. } => "zwlr_output_configuration_head_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrOutputManagerV1 { version } => version,
            Self::ZwlrOutputHeadV1 { version } => version,
            Self::ZwlrOutputModeV1 { version } => version,
            Self::ZwlrOutputConfigurationV1 { version } => version,
            Self::ZwlrOutputConfigurationHeadV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrOutputManagerV1 { .. } => 4,
            Self::ZwlrOutputHeadV1 { .. } => 4,
            Self::ZwlrOutputModeV1 { .. } => 3,
            Self::ZwlrOutputConfigurationV1 { .. } => 4,
            Self::ZwlrOutputConfigurationHeadV1 { .. } => 4,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_output_manager_v1" => Self::ZwlrOutputManagerV1 { version },
            "zwlr_output_head_v1" => Self::ZwlrOutputHeadV1 { version },
            "zwlr_output_mode_v1" => Self::ZwlrOutputModeV1 { version },
            "zwlr_output_configuration_v1" => Self::ZwlrOutputConfigurationV1 { version },
            "zwlr_output_configuration_head_v1" => Self::ZwlrOutputConfigurationHeadV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
