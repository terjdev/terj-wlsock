//! Identifies WLR data control interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrDataControlManagerV1 { version: u32 },
    ZwlrDataControlDeviceV1 { version: u32 },
    ZwlrDataControlSourceV1 { version: u32 },
    ZwlrDataControlOfferV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrDataControlManagerV1 { .. } => "zwlr_data_control_manager_v1",
            Self::ZwlrDataControlDeviceV1 { .. } => "zwlr_data_control_device_v1",
            Self::ZwlrDataControlSourceV1 { .. } => "zwlr_data_control_source_v1",
            Self::ZwlrDataControlOfferV1 { .. } => "zwlr_data_control_offer_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrDataControlManagerV1 { version } => version,
            Self::ZwlrDataControlDeviceV1 { version } => version,
            Self::ZwlrDataControlSourceV1 { version } => version,
            Self::ZwlrDataControlOfferV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrDataControlManagerV1 { .. } => 2,
            Self::ZwlrDataControlDeviceV1 { .. } => 2,
            Self::ZwlrDataControlSourceV1 { .. } => 1,
            Self::ZwlrDataControlOfferV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_data_control_manager_v1" => Self::ZwlrDataControlManagerV1 { version },
            "zwlr_data_control_device_v1" => Self::ZwlrDataControlDeviceV1 { version },
            "zwlr_data_control_source_v1" => Self::ZwlrDataControlSourceV1 { version },
            "zwlr_data_control_offer_v1" => Self::ZwlrDataControlOfferV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
