//! Identifies Xdg system bell interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgSystemBellV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgSystemBellV1 { .. } => "xdg_system_bell_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgSystemBellV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgSystemBellV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_system_bell_v1" => Self::XdgSystemBellV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
