//! Identifies Tablet interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpTabletManagerV2 { version: u32 },
    ZwpTabletSeatV2 { version: u32 },
    ZwpTabletToolV2 { version: u32 },
    ZwpTabletV2 { version: u32 },
    ZwpTabletPadRingV2 { version: u32 },
    ZwpTabletPadStripV2 { version: u32 },
    ZwpTabletPadGroupV2 { version: u32 },
    ZwpTabletPadV2 { version: u32 },
    ZwpTabletPadDialV2 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpTabletManagerV2 { .. } => "zwp_tablet_manager_v2",
            Self::ZwpTabletSeatV2 { .. } => "zwp_tablet_seat_v2",
            Self::ZwpTabletToolV2 { .. } => "zwp_tablet_tool_v2",
            Self::ZwpTabletV2 { .. } => "zwp_tablet_v2",
            Self::ZwpTabletPadRingV2 { .. } => "zwp_tablet_pad_ring_v2",
            Self::ZwpTabletPadStripV2 { .. } => "zwp_tablet_pad_strip_v2",
            Self::ZwpTabletPadGroupV2 { .. } => "zwp_tablet_pad_group_v2",
            Self::ZwpTabletPadV2 { .. } => "zwp_tablet_pad_v2",
            Self::ZwpTabletPadDialV2 { .. } => "zwp_tablet_pad_dial_v2",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpTabletManagerV2 { version } => version,
            Self::ZwpTabletSeatV2 { version } => version,
            Self::ZwpTabletToolV2 { version } => version,
            Self::ZwpTabletV2 { version } => version,
            Self::ZwpTabletPadRingV2 { version } => version,
            Self::ZwpTabletPadStripV2 { version } => version,
            Self::ZwpTabletPadGroupV2 { version } => version,
            Self::ZwpTabletPadV2 { version } => version,
            Self::ZwpTabletPadDialV2 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpTabletManagerV2 { .. } => 2,
            Self::ZwpTabletSeatV2 { .. } => 2,
            Self::ZwpTabletToolV2 { .. } => 2,
            Self::ZwpTabletV2 { .. } => 2,
            Self::ZwpTabletPadRingV2 { .. } => 2,
            Self::ZwpTabletPadStripV2 { .. } => 2,
            Self::ZwpTabletPadGroupV2 { .. } => 2,
            Self::ZwpTabletPadV2 { .. } => 2,
            Self::ZwpTabletPadDialV2 { .. } => 2,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_tablet_manager_v2" => Self::ZwpTabletManagerV2 { version },
            "zwp_tablet_seat_v2" => Self::ZwpTabletSeatV2 { version },
            "zwp_tablet_tool_v2" => Self::ZwpTabletToolV2 { version },
            "zwp_tablet_v2" => Self::ZwpTabletV2 { version },
            "zwp_tablet_pad_ring_v2" => Self::ZwpTabletPadRingV2 { version },
            "zwp_tablet_pad_strip_v2" => Self::ZwpTabletPadStripV2 { version },
            "zwp_tablet_pad_group_v2" => Self::ZwpTabletPadGroupV2 { version },
            "zwp_tablet_pad_v2" => Self::ZwpTabletPadV2 { version },
            "zwp_tablet_pad_dial_v2" => Self::ZwpTabletPadDialV2 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
