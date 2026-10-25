//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpInputTimestampsManagerV1 { version: u32 },
    ZwpInputTimestampsV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpInputTimestampsManagerV1 { .. } => "zwp_input_timestamps_manager_v1",
            Self::ZwpInputTimestampsV1 { .. } => "zwp_input_timestamps_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpInputTimestampsManagerV1 { version } => version,
            Self::ZwpInputTimestampsV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpInputTimestampsManagerV1 { .. } => 1,
            Self::ZwpInputTimestampsV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_input_timestamps_manager_v1" => Self::ZwpInputTimestampsManagerV1 { version },
            "zwp_input_timestamps_v1" => Self::ZwpInputTimestampsV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
