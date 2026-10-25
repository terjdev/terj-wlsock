//! Identifies WLR virtual pointer interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrVirtualPointerV1 { version: u32 },
    ZwlrVirtualPointerManagerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrVirtualPointerV1 { .. } => "zwlr_virtual_pointer_v1",
            Self::ZwlrVirtualPointerManagerV1 { .. } => "zwlr_virtual_pointer_manager_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrVirtualPointerV1 { version } => version,
            Self::ZwlrVirtualPointerManagerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrVirtualPointerV1 { .. } => 2,
            Self::ZwlrVirtualPointerManagerV1 { .. } => 2,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_virtual_pointer_v1" => Self::ZwlrVirtualPointerV1 { version },
            "zwlr_virtual_pointer_manager_v1" => Self::ZwlrVirtualPointerManagerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
