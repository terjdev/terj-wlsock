//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XxSessionManagerV1 { version: u32 },
    XxSessionV1 { version: u32 },
    XxToplevelSessionV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XxSessionManagerV1 { .. } => "xx_session_manager_v1",
            Self::XxSessionV1 { .. } => "xx_session_v1",
            Self::XxToplevelSessionV1 { .. } => "xx_toplevel_session_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XxSessionManagerV1 { version } => version,
            Self::XxSessionV1 { version } => version,
            Self::XxToplevelSessionV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XxSessionManagerV1 { .. } => 1,
            Self::XxSessionV1 { .. } => 1,
            Self::XxToplevelSessionV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_session_manager_v1" => Self::XxSessionManagerV1 { version },
            "xx_session_v1" => Self::XxSessionV1 { version },
            "xx_toplevel_session_v1" => Self::XxToplevelSessionV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
