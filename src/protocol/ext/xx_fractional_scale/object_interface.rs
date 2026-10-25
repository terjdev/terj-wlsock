//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxFractionalScaleManagerV2 { version: u32 },
    XxFractionalScaleV2 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxFractionalScaleManagerV2 { .. } => "xx_fractional_scale_manager_v2",
            Self::XxFractionalScaleV2 { .. } => "xx_fractional_scale_v2",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxFractionalScaleManagerV2 { version } => version,
            Self::XxFractionalScaleV2 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxFractionalScaleManagerV2 { .. } => 1,
            Self::XxFractionalScaleV2 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_fractional_scale_manager_v2" => Self::XxFractionalScaleManagerV2 { version },
            "xx_fractional_scale_v2" => Self::XxFractionalScaleV2 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
