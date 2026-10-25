//! Identifies XDG shell interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XdgWmBase { version: u32 },
    XdgPositioner { version: u32 },
    XdgSurface { version: u32 },
    XdgToplevel { version: u32 },
    XdgPopup { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XdgWmBase { .. } => "xdg_wm_base",
            Self::XdgPositioner { .. } => "xdg_positioner",
            Self::XdgSurface { .. } => "xdg_surface",
            Self::XdgToplevel { .. } => "xdg_toplevel",
            Self::XdgPopup { .. } => "xdg_popup",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XdgWmBase { version } => version,
            Self::XdgPositioner { version } => version,
            Self::XdgSurface { version } => version,
            Self::XdgToplevel { version } => version,
            Self::XdgPopup { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XdgWmBase { .. } => 7,
            Self::XdgPositioner { .. } => 7,
            Self::XdgSurface { .. } => 7,
            Self::XdgToplevel { .. } => 7,
            Self::XdgPopup { .. } => 7,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
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
