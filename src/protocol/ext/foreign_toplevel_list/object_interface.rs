//! Identifies Foreign toplevel list interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtForeignToplevelListV1 { version: u32 },
    ExtForeignToplevelHandleV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtForeignToplevelListV1 { .. } => "ext_foreign_toplevel_list_v1",
            Self::ExtForeignToplevelHandleV1 { .. } => "ext_foreign_toplevel_handle_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtForeignToplevelListV1 { version } => version,
            Self::ExtForeignToplevelHandleV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtForeignToplevelListV1 { .. } => 1,
            Self::ExtForeignToplevelHandleV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_foreign_toplevel_list_v1" => Self::ExtForeignToplevelListV1 { version },
            "ext_foreign_toplevel_handle_v1" => Self::ExtForeignToplevelHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
