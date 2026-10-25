//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XxTextInputV3 { version: u32 },
    XxTextInputManagerV3 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XxTextInputV3 { .. } => "xx_text_input_v3",
            Self::XxTextInputManagerV3 { .. } => "xx_text_input_manager_v3",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XxTextInputV3 { version } => version,
            Self::XxTextInputManagerV3 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XxTextInputV3 { .. } => 3,
            Self::XxTextInputManagerV3 { .. } => 3,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_text_input_v3" => Self::XxTextInputV3 { version },
            "xx_text_input_manager_v3" => Self::XxTextInputManagerV3 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
