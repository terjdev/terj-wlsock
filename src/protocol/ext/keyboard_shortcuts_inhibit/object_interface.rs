//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpKeyboardShortcutsInhibitManagerV1 { version: u32 },
    ZwpKeyboardShortcutsInhibitorV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { .. } => {
                "zwp_keyboard_shortcuts_inhibit_manager_v1"
            }
            Self::ZwpKeyboardShortcutsInhibitorV1 { .. } => "zwp_keyboard_shortcuts_inhibitor_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { version } => version,
            Self::ZwpKeyboardShortcutsInhibitorV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1 { .. } => 1,
            Self::ZwpKeyboardShortcutsInhibitorV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
