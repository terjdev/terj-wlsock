//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwlrInputInhibitManagerV1 { version: u32 },
    ZwlrInputInhibitorV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwlrInputInhibitManagerV1 { .. } => "zwlr_input_inhibit_manager_v1",
            Self::ZwlrInputInhibitorV1 { .. } => "zwlr_input_inhibitor_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwlrInputInhibitManagerV1 { version } => version,
            Self::ZwlrInputInhibitorV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwlrInputInhibitManagerV1 { .. } => 1,
            Self::ZwlrInputInhibitorV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwlr_input_inhibit_manager_v1" => Self::ZwlrInputInhibitManagerV1 { version },
            "zwlr_input_inhibitor_v1" => Self::ZwlrInputInhibitorV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
