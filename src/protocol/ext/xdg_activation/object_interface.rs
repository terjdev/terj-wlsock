//! Identifies Xdg activation interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XdgActivationV1 { version: u32 },
    XdgActivationTokenV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XdgActivationV1 { .. } => "xdg_activation_v1",
            Self::XdgActivationTokenV1 { .. } => "xdg_activation_token_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XdgActivationV1 { version } => version,
            Self::XdgActivationTokenV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XdgActivationV1 { .. } => 1,
            Self::XdgActivationTokenV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_activation_v1" => Self::XdgActivationV1 { version },
            "xdg_activation_token_v1" => Self::XdgActivationTokenV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
