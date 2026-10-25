//! Identifies Foreign toplevel list interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtForeignToplevelListV1 { version: u32 },
    ExtForeignToplevelHandleV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtForeignToplevelListV1 { .. } => "ext_foreign_toplevel_list_v1",
            Self::ExtForeignToplevelHandleV1 { .. } => "ext_foreign_toplevel_handle_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtForeignToplevelListV1 { version } => version,
            Self::ExtForeignToplevelHandleV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtForeignToplevelListV1 { .. } => 1,
            Self::ExtForeignToplevelHandleV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_foreign_toplevel_list_v1" => Self::ExtForeignToplevelListV1 { version },
            "ext_foreign_toplevel_handle_v1" => Self::ExtForeignToplevelHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
