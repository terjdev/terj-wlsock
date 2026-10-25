//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxHotkeyManagerV1 { version: u32 },
    XxHotkeyV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxHotkeyManagerV1 { .. } => "xx_hotkey_manager_v1",
            Self::XxHotkeyV1 { .. } => "xx_hotkey_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxHotkeyManagerV1 { version } => version,
            Self::XxHotkeyV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxHotkeyManagerV1 { .. } => 1,
            Self::XxHotkeyV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_hotkey_manager_v1" => Self::XxHotkeyManagerV1 { version },
            "xx_hotkey_v1" => Self::XxHotkeyV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
