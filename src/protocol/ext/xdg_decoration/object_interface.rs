//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZxdgDecorationManagerV1 { version: u32 },
    ZxdgToplevelDecorationV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZxdgDecorationManagerV1 { .. } => "zxdg_decoration_manager_v1",
            Self::ZxdgToplevelDecorationV1 { .. } => "zxdg_toplevel_decoration_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZxdgDecorationManagerV1 { version } => version,
            Self::ZxdgToplevelDecorationV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZxdgDecorationManagerV1 { .. } => 2,
            Self::ZxdgToplevelDecorationV1 { .. } => 2,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zxdg_decoration_manager_v1" => Self::ZxdgDecorationManagerV1 { version },
            "zxdg_toplevel_decoration_v1" => Self::ZxdgToplevelDecorationV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
