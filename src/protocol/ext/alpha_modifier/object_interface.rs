//! Identifies Alpha modifier interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpAlphaModifierV1 { version: u32 },
    WpAlphaModifierSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpAlphaModifierV1 { .. } => "wp_alpha_modifier_v1",
            Self::WpAlphaModifierSurfaceV1 { .. } => "wp_alpha_modifier_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpAlphaModifierV1 { version } => version,
            Self::WpAlphaModifierSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpAlphaModifierV1 { .. } => 1,
            Self::WpAlphaModifierSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_alpha_modifier_v1" => Self::WpAlphaModifierV1 { version },
            "wp_alpha_modifier_surface_v1" => Self::WpAlphaModifierSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
