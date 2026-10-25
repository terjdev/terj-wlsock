//! Identifies Fifo interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpFifoManagerV1 { version: u32 },
    WpFifoV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpFifoManagerV1 { .. } => "wp_fifo_manager_v1",
            Self::WpFifoV1 { .. } => "wp_fifo_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpFifoManagerV1 { version } => version,
            Self::WpFifoV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpFifoManagerV1 { .. } => 1,
            Self::WpFifoV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_fifo_manager_v1" => Self::WpFifoManagerV1 { version },
            "wp_fifo_v1" => Self::WpFifoV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
