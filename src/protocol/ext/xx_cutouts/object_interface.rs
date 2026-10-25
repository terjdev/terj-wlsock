//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XxCutoutsManagerV1 { version: u32 },
    XxCutoutsV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XxCutoutsManagerV1 { .. } => "xx_cutouts_manager_v1",
            Self::XxCutoutsV1 { .. } => "xx_cutouts_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XxCutoutsManagerV1 { version } => version,
            Self::XxCutoutsV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XxCutoutsManagerV1 { .. } => 1,
            Self::XxCutoutsV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_cutouts_manager_v1" => Self::XxCutoutsManagerV1 { version },
            "xx_cutouts_v1" => Self::XxCutoutsV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
