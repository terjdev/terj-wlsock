//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpTextInputV1 { version: u32 },
    ZwpTextInputManagerV1 { version: u32 },
    ZwpTextInputV3 { version: u32 },
    ZwpTextInputManagerV3 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpTextInputV1 { .. } => "zwp_text_input_v1",
            Self::ZwpTextInputManagerV1 { .. } => "zwp_text_input_manager_v1",
            Self::ZwpTextInputV3 { .. } => "zwp_text_input_v3",
            Self::ZwpTextInputManagerV3 { .. } => "zwp_text_input_manager_v3",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpTextInputV1 { version } => version,
            Self::ZwpTextInputManagerV1 { version } => version,
            Self::ZwpTextInputV3 { version } => version,
            Self::ZwpTextInputManagerV3 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpTextInputV1 { .. } => 1,
            Self::ZwpTextInputManagerV1 { .. } => 1,
            Self::ZwpTextInputV3 { .. } => 2,
            Self::ZwpTextInputManagerV3 { .. } => 2,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_text_input_v1" => Self::ZwpTextInputV1 { version },
            "zwp_text_input_manager_v1" => Self::ZwpTextInputManagerV1 { version },
            "zwp_text_input_v3" => Self::ZwpTextInputV3 { version },
            "zwp_text_input_manager_v3" => Self::ZwpTextInputManagerV3 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
