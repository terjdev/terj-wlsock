//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrOutputManagerV1 { version: u32 },
    ZwlrOutputHeadV1 { version: u32 },
    ZwlrOutputModeV1 { version: u32 },
    ZwlrOutputConfigurationV1 { version: u32 },
    ZwlrOutputConfigurationHeadV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrOutputManagerV1 { .. } => "zwlr_output_manager_v1",
            Self::ZwlrOutputHeadV1 { .. } => "zwlr_output_head_v1",
            Self::ZwlrOutputModeV1 { .. } => "zwlr_output_mode_v1",
            Self::ZwlrOutputConfigurationV1 { .. } => "zwlr_output_configuration_v1",
            Self::ZwlrOutputConfigurationHeadV1 { .. } => "zwlr_output_configuration_head_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrOutputManagerV1 { version } => version,
            Self::ZwlrOutputHeadV1 { version } => version,
            Self::ZwlrOutputModeV1 { version } => version,
            Self::ZwlrOutputConfigurationV1 { version } => version,
            Self::ZwlrOutputConfigurationHeadV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrOutputManagerV1 { .. } => 4,
            Self::ZwlrOutputHeadV1 { .. } => 4,
            Self::ZwlrOutputModeV1 { .. } => 3,
            Self::ZwlrOutputConfigurationV1 { .. } => 4,
            Self::ZwlrOutputConfigurationHeadV1 { .. } => 4,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
