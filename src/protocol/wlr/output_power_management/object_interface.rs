//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrOutputPowerManagerV1 { version: u32 },
    ZwlrOutputPowerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrOutputPowerManagerV1 { .. } => "zwlr_output_power_manager_v1",
            Self::ZwlrOutputPowerV1 { .. } => "zwlr_output_power_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrOutputPowerManagerV1 { version } => version,
            Self::ZwlrOutputPowerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrOutputPowerManagerV1 { .. } => 1,
            Self::ZwlrOutputPowerV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_output_power_manager_v1" => Self::ZwlrOutputPowerManagerV1 { version },
            "zwlr_output_power_v1" => Self::ZwlrOutputPowerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
