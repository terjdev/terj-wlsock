//! Defines Viewporter object interfaces and their negotiated versions.

/// Identifies a client object interface and its negotiated protocol version.
///
/// Object registration validates the negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    /// Object of interface `wp_viewporter`; maximum supported version: 1.
    WpViewporter { version: u32 },
    /// Object of interface `wp_viewport`; maximum supported version: 1.
    WpViewport { version: u32 },
}
impl ObjectInteface {
    /// Returns the interface name defined in XML.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpViewport { .. } => "wp_viewport",
            Self::WpViewporter { .. } => "wp_viewporter",
        }
    }
    /// Returns the negotiated object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpViewport { version } => version,
            Self::WpViewporter { version } => version,
        }
    }
    /// Returns the highest supported version of this interface.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpViewport { .. } => 1,
            Self::WpViewporter { .. } => 1,
        }
    }
    /// Creates a known interface, rejecting zero or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_viewport" => Self::WpViewport { version },
            "wp_viewporter" => Self::WpViewporter { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
