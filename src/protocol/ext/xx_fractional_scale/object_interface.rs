//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XxFractionalScaleManagerV2 { version: u32 },
    XxFractionalScaleV2 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XxFractionalScaleManagerV2 { .. } => "xx_fractional_scale_manager_v2",
            Self::XxFractionalScaleV2 { .. } => "xx_fractional_scale_v2",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XxFractionalScaleManagerV2 { version } => version,
            Self::XxFractionalScaleV2 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XxFractionalScaleManagerV2 { .. } => 1,
            Self::XxFractionalScaleV2 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_fractional_scale_manager_v2" => Self::XxFractionalScaleManagerV2 { version },
            "xx_fractional_scale_v2" => Self::XxFractionalScaleV2 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
