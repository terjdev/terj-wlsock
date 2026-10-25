//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpIdleInhibitManagerV1 { version: u32 },
    ZwpIdleInhibitorV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpIdleInhibitManagerV1 { .. } => "zwp_idle_inhibit_manager_v1",
            Self::ZwpIdleInhibitorV1 { .. } => "zwp_idle_inhibitor_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpIdleInhibitManagerV1 { version } => version,
            Self::ZwpIdleInhibitorV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpIdleInhibitManagerV1 { .. } => 1,
            Self::ZwpIdleInhibitorV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_idle_inhibit_manager_v1" => Self::ZwpIdleInhibitManagerV1 { version },
            "zwp_idle_inhibitor_v1" => Self::ZwpIdleInhibitorV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
