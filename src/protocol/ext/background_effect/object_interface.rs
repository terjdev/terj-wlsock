//! Identifies Background effect interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtBackgroundEffectManagerV1 { version: u32 },
    ExtBackgroundEffectSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtBackgroundEffectManagerV1 { .. } => "ext_background_effect_manager_v1",
            Self::ExtBackgroundEffectSurfaceV1 { .. } => "ext_background_effect_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtBackgroundEffectManagerV1 { version } => version,
            Self::ExtBackgroundEffectSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtBackgroundEffectManagerV1 { .. } => 1,
            Self::ExtBackgroundEffectSurfaceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_background_effect_manager_v1" => Self::ExtBackgroundEffectManagerV1 { version },
            "ext_background_effect_surface_v1" => Self::ExtBackgroundEffectSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
