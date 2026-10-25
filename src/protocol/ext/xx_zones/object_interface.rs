//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XxZoneManagerV1 { version: u32 },
    XxZoneItemV1 { version: u32 },
    XxZoneV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XxZoneManagerV1 { .. } => "xx_zone_manager_v1",
            Self::XxZoneItemV1 { .. } => "xx_zone_item_v1",
            Self::XxZoneV1 { .. } => "xx_zone_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XxZoneManagerV1 { version } => version,
            Self::XxZoneItemV1 { version } => version,
            Self::XxZoneV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XxZoneManagerV1 { .. } => 1,
            Self::XxZoneItemV1 { .. } => 1,
            Self::XxZoneV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_zone_manager_v1" => Self::XxZoneManagerV1 { version },
            "xx_zone_item_v1" => Self::XxZoneItemV1 { version },
            "xx_zone_v1" => Self::XxZoneV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
