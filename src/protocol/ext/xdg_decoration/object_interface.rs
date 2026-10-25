//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZxdgDecorationManagerV1 { version: u32 },
    ZxdgToplevelDecorationV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZxdgDecorationManagerV1 { .. } => "zxdg_decoration_manager_v1",
            Self::ZxdgToplevelDecorationV1 { .. } => "zxdg_toplevel_decoration_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZxdgDecorationManagerV1 { version } => version,
            Self::ZxdgToplevelDecorationV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZxdgDecorationManagerV1 { .. } => 2,
            Self::ZxdgToplevelDecorationV1 { .. } => 2,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zxdg_decoration_manager_v1" => Self::ZxdgDecorationManagerV1 { version },
            "zxdg_toplevel_decoration_v1" => Self::ZxdgToplevelDecorationV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
