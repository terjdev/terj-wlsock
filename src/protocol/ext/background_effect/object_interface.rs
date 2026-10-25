//! Identifies Background effect interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtBackgroundEffectManagerV1 { version: u32 },
    ExtBackgroundEffectSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtBackgroundEffectManagerV1 { .. } => "ext_background_effect_manager_v1",
            Self::ExtBackgroundEffectSurfaceV1 { .. } => "ext_background_effect_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtBackgroundEffectManagerV1 { version } => version,
            Self::ExtBackgroundEffectSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtBackgroundEffectManagerV1 { .. } => 1,
            Self::ExtBackgroundEffectSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_background_effect_manager_v1" => Self::ExtBackgroundEffectManagerV1 { version },
            "ext_background_effect_surface_v1" => Self::ExtBackgroundEffectSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
