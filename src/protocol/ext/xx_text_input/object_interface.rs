//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxTextInputV3 { version: u32 },
    XxTextInputManagerV3 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxTextInputV3 { .. } => "xx_text_input_v3",
            Self::XxTextInputManagerV3 { .. } => "xx_text_input_manager_v3",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxTextInputV3 { version } => version,
            Self::XxTextInputManagerV3 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxTextInputV3 { .. } => 3,
            Self::XxTextInputManagerV3 { .. } => 3,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_text_input_v3" => Self::XxTextInputV3 { version },
            "xx_text_input_manager_v3" => Self::XxTextInputManagerV3 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
