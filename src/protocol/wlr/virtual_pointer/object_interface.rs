//! Identifies WLR virtual pointer interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrVirtualPointerV1 { version: u32 },
    ZwlrVirtualPointerManagerV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrVirtualPointerV1 { .. } => "zwlr_virtual_pointer_v1",
            Self::ZwlrVirtualPointerManagerV1 { .. } => "zwlr_virtual_pointer_manager_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrVirtualPointerV1 { version } => version,
            Self::ZwlrVirtualPointerManagerV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrVirtualPointerV1 { .. } => 2,
            Self::ZwlrVirtualPointerManagerV1 { .. } => 2,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_virtual_pointer_v1" => Self::ZwlrVirtualPointerV1 { version },
            "zwlr_virtual_pointer_manager_v1" => Self::ZwlrVirtualPointerManagerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
