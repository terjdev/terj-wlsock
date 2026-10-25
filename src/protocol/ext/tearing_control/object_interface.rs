//! Identifies Tearing control interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpTearingControlManagerV1 { version: u32 },
    WpTearingControlV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpTearingControlManagerV1 { .. } => "wp_tearing_control_manager_v1",
            Self::WpTearingControlV1 { .. } => "wp_tearing_control_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpTearingControlManagerV1 { version } => version,
            Self::WpTearingControlV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpTearingControlManagerV1 { .. } => 1,
            Self::WpTearingControlV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_tearing_control_manager_v1" => Self::WpTearingControlManagerV1 { version },
            "wp_tearing_control_v1" => Self::WpTearingControlV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
