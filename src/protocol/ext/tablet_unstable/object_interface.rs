//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpTabletManagerV1 { version: u32 },
    ZwpTabletSeatV1 { version: u32 },
    ZwpTabletToolV1 { version: u32 },
    ZwpTabletV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpTabletManagerV1 { .. } => "zwp_tablet_manager_v1",
            Self::ZwpTabletSeatV1 { .. } => "zwp_tablet_seat_v1",
            Self::ZwpTabletToolV1 { .. } => "zwp_tablet_tool_v1",
            Self::ZwpTabletV1 { .. } => "zwp_tablet_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpTabletManagerV1 { version } => version,
            Self::ZwpTabletSeatV1 { version } => version,
            Self::ZwpTabletToolV1 { version } => version,
            Self::ZwpTabletV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpTabletManagerV1 { .. } => 1,
            Self::ZwpTabletSeatV1 { .. } => 1,
            Self::ZwpTabletToolV1 { .. } => 1,
            Self::ZwpTabletV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_tablet_manager_v1" => Self::ZwpTabletManagerV1 { version },
            "zwp_tablet_seat_v1" => Self::ZwpTabletSeatV1 { version },
            "zwp_tablet_tool_v1" => Self::ZwpTabletToolV1 { version },
            "zwp_tablet_v1" => Self::ZwpTabletV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
