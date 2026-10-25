//! Identifies XDG shell interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    XdgWmBase { version: u32 },
    XdgPositioner { version: u32 },
    XdgSurface { version: u32 },
    XdgToplevel { version: u32 },
    XdgPopup { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::XdgWmBase { .. } => "xdg_wm_base",
            Self::XdgPositioner { .. } => "xdg_positioner",
            Self::XdgSurface { .. } => "xdg_surface",
            Self::XdgToplevel { .. } => "xdg_toplevel",
            Self::XdgPopup { .. } => "xdg_popup",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::XdgWmBase { version } => version,
            Self::XdgPositioner { version } => version,
            Self::XdgSurface { version } => version,
            Self::XdgToplevel { version } => version,
            Self::XdgPopup { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::XdgWmBase { .. } => 7,
            Self::XdgPositioner { .. } => 7,
            Self::XdgSurface { .. } => 7,
            Self::XdgToplevel { .. } => 7,
            Self::XdgPopup { .. } => 7,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xdg_wm_base" => Self::XdgWmBase { version },
            "xdg_positioner" => Self::XdgPositioner { version },
            "xdg_surface" => Self::XdgSurface { version },
            "xdg_toplevel" => Self::XdgToplevel { version },
            "xdg_popup" => Self::XdgPopup { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
