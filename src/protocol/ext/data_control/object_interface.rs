//! Identifies Data control interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtDataControlManagerV1 { version: u32 },
    ExtDataControlDeviceV1 { version: u32 },
    ExtDataControlSourceV1 { version: u32 },
    ExtDataControlOfferV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtDataControlManagerV1 { .. } => "ext_data_control_manager_v1",
            Self::ExtDataControlDeviceV1 { .. } => "ext_data_control_device_v1",
            Self::ExtDataControlSourceV1 { .. } => "ext_data_control_source_v1",
            Self::ExtDataControlOfferV1 { .. } => "ext_data_control_offer_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtDataControlManagerV1 { version } => version,
            Self::ExtDataControlDeviceV1 { version } => version,
            Self::ExtDataControlSourceV1 { version } => version,
            Self::ExtDataControlOfferV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtDataControlManagerV1 { .. } => 1,
            Self::ExtDataControlDeviceV1 { .. } => 1,
            Self::ExtDataControlSourceV1 { .. } => 1,
            Self::ExtDataControlOfferV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_data_control_manager_v1" => Self::ExtDataControlManagerV1 { version },
            "ext_data_control_device_v1" => Self::ExtDataControlDeviceV1 { version },
            "ext_data_control_source_v1" => Self::ExtDataControlSourceV1 { version },
            "ext_data_control_offer_v1" => Self::ExtDataControlOfferV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
