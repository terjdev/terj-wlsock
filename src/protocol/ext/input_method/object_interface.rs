//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpInputMethodContextV1 { version: u32 },
    ZwpInputMethodV1 { version: u32 },
    ZwpInputPanelV1 { version: u32 },
    ZwpInputPanelSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpInputMethodContextV1 { .. } => "zwp_input_method_context_v1",
            Self::ZwpInputMethodV1 { .. } => "zwp_input_method_v1",
            Self::ZwpInputPanelV1 { .. } => "zwp_input_panel_v1",
            Self::ZwpInputPanelSurfaceV1 { .. } => "zwp_input_panel_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpInputMethodContextV1 { version } => version,
            Self::ZwpInputMethodV1 { version } => version,
            Self::ZwpInputPanelV1 { version } => version,
            Self::ZwpInputPanelSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpInputMethodContextV1 { .. } => 1,
            Self::ZwpInputMethodV1 { .. } => 1,
            Self::ZwpInputPanelV1 { .. } => 1,
            Self::ZwpInputPanelSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_input_method_context_v1" => Self::ZwpInputMethodContextV1 { version },
            "zwp_input_method_v1" => Self::ZwpInputMethodV1 { version },
            "zwp_input_panel_v1" => Self::ZwpInputPanelV1 { version },
            "zwp_input_panel_surface_v1" => Self::ZwpInputPanelSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
