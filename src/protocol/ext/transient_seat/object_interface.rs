//! Identifies Transient seat interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtTransientSeatManagerV1 { version: u32 },
    ExtTransientSeatV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtTransientSeatManagerV1 { .. } => "ext_transient_seat_manager_v1",
            Self::ExtTransientSeatV1 { .. } => "ext_transient_seat_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtTransientSeatManagerV1 { version } => version,
            Self::ExtTransientSeatV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtTransientSeatManagerV1 { .. } => 1,
            Self::ExtTransientSeatV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_transient_seat_manager_v1" => Self::ExtTransientSeatManagerV1 { version },
            "ext_transient_seat_v1" => Self::ExtTransientSeatV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
