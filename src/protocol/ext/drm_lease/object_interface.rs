//! Identifies DRM lease interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpDrmLeaseDeviceV1 { version: u32 },
    WpDrmLeaseConnectorV1 { version: u32 },
    WpDrmLeaseRequestV1 { version: u32 },
    WpDrmLeaseV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpDrmLeaseDeviceV1 { .. } => "wp_drm_lease_device_v1",
            Self::WpDrmLeaseConnectorV1 { .. } => "wp_drm_lease_connector_v1",
            Self::WpDrmLeaseRequestV1 { .. } => "wp_drm_lease_request_v1",
            Self::WpDrmLeaseV1 { .. } => "wp_drm_lease_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1 { version } => version,
            Self::WpDrmLeaseConnectorV1 { version } => version,
            Self::WpDrmLeaseRequestV1 { version } => version,
            Self::WpDrmLeaseV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1 { .. } => 1,
            Self::WpDrmLeaseConnectorV1 { .. } => 1,
            Self::WpDrmLeaseRequestV1 { .. } => 1,
            Self::WpDrmLeaseV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_drm_lease_device_v1" => Self::WpDrmLeaseDeviceV1 { version },
            "wp_drm_lease_connector_v1" => Self::WpDrmLeaseConnectorV1 { version },
            "wp_drm_lease_request_v1" => Self::WpDrmLeaseRequestV1 { version },
            "wp_drm_lease_v1" => Self::WpDrmLeaseV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
