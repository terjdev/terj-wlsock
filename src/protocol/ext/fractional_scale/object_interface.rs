//! Identifies Fractional scale interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpFractionalScaleManagerV1 { version: u32 },
    WpFractionalScaleV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpFractionalScaleManagerV1 { .. } => "wp_fractional_scale_manager_v1",
            Self::WpFractionalScaleV1 { .. } => "wp_fractional_scale_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpFractionalScaleManagerV1 { version } => version,
            Self::WpFractionalScaleV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpFractionalScaleManagerV1 { .. } => 1,
            Self::WpFractionalScaleV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_fractional_scale_manager_v1" => Self::WpFractionalScaleManagerV1 { version },
            "wp_fractional_scale_v1" => Self::WpFractionalScaleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
