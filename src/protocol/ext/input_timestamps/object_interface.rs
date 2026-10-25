//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpInputTimestampsManagerV1 { version: u32 },
    ZwpInputTimestampsV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpInputTimestampsManagerV1 { .. } => "zwp_input_timestamps_manager_v1",
            Self::ZwpInputTimestampsV1 { .. } => "zwp_input_timestamps_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpInputTimestampsManagerV1 { version } => version,
            Self::ZwpInputTimestampsV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpInputTimestampsManagerV1 { .. } => 1,
            Self::ZwpInputTimestampsV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_input_timestamps_manager_v1" => Self::ZwpInputTimestampsManagerV1 { version },
            "zwp_input_timestamps_v1" => Self::ZwpInputTimestampsV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
