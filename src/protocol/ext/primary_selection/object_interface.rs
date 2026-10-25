//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpPrimarySelectionDeviceManagerV1 { version: u32 },
    ZwpPrimarySelectionDeviceV1 { version: u32 },
    ZwpPrimarySelectionOfferV1 { version: u32 },
    ZwpPrimarySelectionSourceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { .. } => {
                "zwp_primary_selection_device_manager_v1"
            }
            Self::ZwpPrimarySelectionDeviceV1 { .. } => "zwp_primary_selection_device_v1",
            Self::ZwpPrimarySelectionOfferV1 { .. } => "zwp_primary_selection_offer_v1",
            Self::ZwpPrimarySelectionSourceV1 { .. } => "zwp_primary_selection_source_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { version } => version,
            Self::ZwpPrimarySelectionDeviceV1 { version } => version,
            Self::ZwpPrimarySelectionOfferV1 { version } => version,
            Self::ZwpPrimarySelectionSourceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { .. } => 1,
            Self::ZwpPrimarySelectionDeviceV1 { .. } => 1,
            Self::ZwpPrimarySelectionOfferV1 { .. } => 1,
            Self::ZwpPrimarySelectionSourceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_primary_selection_device_manager_v1" => {
                Self::ZwpPrimarySelectionDeviceManagerV1 { version }
            }
            "zwp_primary_selection_device_v1" => Self::ZwpPrimarySelectionDeviceV1 { version },
            "zwp_primary_selection_offer_v1" => Self::ZwpPrimarySelectionOfferV1 { version },
            "zwp_primary_selection_source_v1" => Self::ZwpPrimarySelectionSourceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
