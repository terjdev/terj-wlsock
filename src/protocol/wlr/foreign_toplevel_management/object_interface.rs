//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrForeignToplevelManagerV1 { version: u32 },
    ZwlrForeignToplevelHandleV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { .. } => "zwlr_foreign_toplevel_manager_v1",
            Self::ZwlrForeignToplevelHandleV1 { .. } => "zwlr_foreign_toplevel_handle_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { version } => version,
            Self::ZwlrForeignToplevelHandleV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { .. } => 3,
            Self::ZwlrForeignToplevelHandleV1 { .. } => 3,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_foreign_toplevel_manager_v1" => Self::ZwlrForeignToplevelManagerV1 { version },
            "zwlr_foreign_toplevel_handle_v1" => Self::ZwlrForeignToplevelHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
