//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrInputInhibitManagerV1 { version: u32 },
    ZwlrInputInhibitorV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrInputInhibitManagerV1 { .. } => "zwlr_input_inhibit_manager_v1",
            Self::ZwlrInputInhibitorV1 { .. } => "zwlr_input_inhibitor_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrInputInhibitManagerV1 { version } => version,
            Self::ZwlrInputInhibitorV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrInputInhibitManagerV1 { .. } => 1,
            Self::ZwlrInputInhibitorV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_input_inhibit_manager_v1" => Self::ZwlrInputInhibitManagerV1 { version },
            "zwlr_input_inhibitor_v1" => Self::ZwlrInputInhibitorV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
