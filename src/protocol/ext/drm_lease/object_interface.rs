//! Identifies DRM lease interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpDrmLeaseDeviceV1 { version: u32 },
    WpDrmLeaseConnectorV1 { version: u32 },
    WpDrmLeaseRequestV1 { version: u32 },
    WpDrmLeaseV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpDrmLeaseDeviceV1 { .. } => "wp_drm_lease_device_v1",
            Self::WpDrmLeaseConnectorV1 { .. } => "wp_drm_lease_connector_v1",
            Self::WpDrmLeaseRequestV1 { .. } => "wp_drm_lease_request_v1",
            Self::WpDrmLeaseV1 { .. } => "wp_drm_lease_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1 { version } => version,
            Self::WpDrmLeaseConnectorV1 { version } => version,
            Self::WpDrmLeaseRequestV1 { version } => version,
            Self::WpDrmLeaseV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1 { .. } => 1,
            Self::WpDrmLeaseConnectorV1 { .. } => 1,
            Self::WpDrmLeaseRequestV1 { .. } => 1,
            Self::WpDrmLeaseV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
