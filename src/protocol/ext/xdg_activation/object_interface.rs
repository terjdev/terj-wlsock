//! Identifies Xdg activation interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgActivationV1 { version: u32 },
    XdgActivationTokenV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgActivationV1 { .. } => "xdg_activation_v1",
            Self::XdgActivationTokenV1 { .. } => "xdg_activation_token_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgActivationV1 { version } => version,
            Self::XdgActivationTokenV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgActivationV1 { .. } => 1,
            Self::XdgActivationTokenV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_activation_v1" => Self::XdgActivationV1 { version },
            "xdg_activation_token_v1" => Self::XdgActivationTokenV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
