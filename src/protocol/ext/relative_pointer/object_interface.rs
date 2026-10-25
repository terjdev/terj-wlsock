//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpRelativePointerManagerV1 { version: u32 },
    ZwpRelativePointerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpRelativePointerManagerV1 { .. } => "zwp_relative_pointer_manager_v1",
            Self::ZwpRelativePointerV1 { .. } => "zwp_relative_pointer_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpRelativePointerManagerV1 { version } => version,
            Self::ZwpRelativePointerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpRelativePointerManagerV1 { .. } => 1,
            Self::ZwpRelativePointerV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_relative_pointer_manager_v1" => Self::ZwpRelativePointerManagerV1 { version },
            "zwp_relative_pointer_v1" => Self::ZwpRelativePointerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
