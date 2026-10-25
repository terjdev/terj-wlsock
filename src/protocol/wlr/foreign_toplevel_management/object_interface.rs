//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrForeignToplevelManagerV1 { version: u32 },
    ZwlrForeignToplevelHandleV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { .. } => "zwlr_foreign_toplevel_manager_v1",
            Self::ZwlrForeignToplevelHandleV1 { .. } => "zwlr_foreign_toplevel_handle_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { version } => version,
            Self::ZwlrForeignToplevelHandleV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrForeignToplevelManagerV1 { .. } => 3,
            Self::ZwlrForeignToplevelHandleV1 { .. } => 3,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_foreign_toplevel_manager_v1" => Self::ZwlrForeignToplevelManagerV1 { version },
            "zwlr_foreign_toplevel_handle_v1" => Self::ZwlrForeignToplevelHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
