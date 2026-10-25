//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpKeyboardShortcutsInhibitManagerV1 { version: u32 },
    ZwpKeyboardShortcutsInhibitorV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { .. } => {
                "zwp_keyboard_shortcuts_inhibit_manager_v1"
            }
            Self::ZwpKeyboardShortcutsInhibitorV1 { .. } => "zwp_keyboard_shortcuts_inhibitor_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { version } => version,
            Self::ZwpKeyboardShortcutsInhibitorV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { .. } => 1,
            Self::ZwpKeyboardShortcutsInhibitorV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_keyboard_shortcuts_inhibit_manager_v1" => {
                Self::ZwpKeyboardShortcutsInhibitManagerV1 { version }
            }
            "zwp_keyboard_shortcuts_inhibitor_v1" => {
                Self::ZwpKeyboardShortcutsInhibitorV1 { version }
            }
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
