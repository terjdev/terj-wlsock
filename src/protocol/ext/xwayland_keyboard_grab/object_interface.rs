//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpXwaylandKeyboardGrabManagerV1 { version: u32 },
    ZwpXwaylandKeyboardGrabV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1 { .. } => {
                "zwp_xwayland_keyboard_grab_manager_v1"
            }
            Self::ZwpXwaylandKeyboardGrabV1 { .. } => "zwp_xwayland_keyboard_grab_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1 { version } => version,
            Self::ZwpXwaylandKeyboardGrabV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpXwaylandKeyboardGrabManagerV1 { .. } => 1,
            Self::ZwpXwaylandKeyboardGrabV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_xwayland_keyboard_grab_manager_v1" => {
                Self::ZwpXwaylandKeyboardGrabManagerV1 { version }
            }
            "zwp_xwayland_keyboard_grab_v1" => Self::ZwpXwaylandKeyboardGrabV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
