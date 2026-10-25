//! Identifies WLR data control interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwlrDataControlManagerV1 { version: u32 },
    ZwlrDataControlDeviceV1 { version: u32 },
    ZwlrDataControlSourceV1 { version: u32 },
    ZwlrDataControlOfferV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwlrDataControlManagerV1 { .. } => "zwlr_data_control_manager_v1",
            Self::ZwlrDataControlDeviceV1 { .. } => "zwlr_data_control_device_v1",
            Self::ZwlrDataControlSourceV1 { .. } => "zwlr_data_control_source_v1",
            Self::ZwlrDataControlOfferV1 { .. } => "zwlr_data_control_offer_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwlrDataControlManagerV1 { version } => version,
            Self::ZwlrDataControlDeviceV1 { version } => version,
            Self::ZwlrDataControlSourceV1 { version } => version,
            Self::ZwlrDataControlOfferV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwlrDataControlManagerV1 { .. } => 2,
            Self::ZwlrDataControlDeviceV1 { .. } => 2,
            Self::ZwlrDataControlSourceV1 { .. } => 1,
            Self::ZwlrDataControlOfferV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
