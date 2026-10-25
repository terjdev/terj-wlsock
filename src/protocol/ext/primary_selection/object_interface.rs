//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpPrimarySelectionDeviceManagerV1 { version: u32 },
    ZwpPrimarySelectionDeviceV1 { version: u32 },
    ZwpPrimarySelectionOfferV1 { version: u32 },
    ZwpPrimarySelectionSourceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { .. } => {
                "zwp_primary_selection_device_manager_v1"
            }
            Self::ZwpPrimarySelectionDeviceV1 { .. } => "zwp_primary_selection_device_v1",
            Self::ZwpPrimarySelectionOfferV1 { .. } => "zwp_primary_selection_offer_v1",
            Self::ZwpPrimarySelectionSourceV1 { .. } => "zwp_primary_selection_source_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { version } => version,
            Self::ZwpPrimarySelectionDeviceV1 { version } => version,
            Self::ZwpPrimarySelectionOfferV1 { version } => version,
            Self::ZwpPrimarySelectionSourceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1 { .. } => 1,
            Self::ZwpPrimarySelectionDeviceV1 { .. } => 1,
            Self::ZwpPrimarySelectionOfferV1 { .. } => 1,
            Self::ZwpPrimarySelectionSourceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
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
