//! Identifies Alpha modifier interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpAlphaModifierV1 { version: u32 },
    WpAlphaModifierSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpAlphaModifierV1 { .. } => "wp_alpha_modifier_v1",
            Self::WpAlphaModifierSurfaceV1 { .. } => "wp_alpha_modifier_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpAlphaModifierV1 { version } => version,
            Self::WpAlphaModifierSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpAlphaModifierV1 { .. } => 1,
            Self::WpAlphaModifierSurfaceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_alpha_modifier_v1" => Self::WpAlphaModifierV1 { version },
            "wp_alpha_modifier_surface_v1" => Self::WpAlphaModifierSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
