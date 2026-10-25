//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxKeyboardFilterV1 { version: u32 },
    XxKeyboardFilterManagerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxKeyboardFilterV1 { .. } => "xx_keyboard_filter_v1",
            Self::XxKeyboardFilterManagerV1 { .. } => "xx_keyboard_filter_manager_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxKeyboardFilterV1 { version } => version,
            Self::XxKeyboardFilterManagerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxKeyboardFilterV1 { .. } => 1,
            Self::XxKeyboardFilterManagerV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_keyboard_filter_v1" => Self::XxKeyboardFilterV1 { version },
            "xx_keyboard_filter_manager_v1" => Self::XxKeyboardFilterManagerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
