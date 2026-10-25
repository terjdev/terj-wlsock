//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZxdgOutputManagerV1 { version: u32 },
    ZxdgOutputV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZxdgOutputManagerV1 { .. } => "zxdg_output_manager_v1",
            Self::ZxdgOutputV1 { .. } => "zxdg_output_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZxdgOutputManagerV1 { version } => version,
            Self::ZxdgOutputV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZxdgOutputManagerV1 { .. } => 3,
            Self::ZxdgOutputV1 { .. } => 3,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zxdg_output_manager_v1" => Self::ZxdgOutputManagerV1 { version },
            "zxdg_output_v1" => Self::ZxdgOutputV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
