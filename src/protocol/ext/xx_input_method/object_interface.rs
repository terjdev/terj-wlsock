//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    XxInputMethodV1 { version: u32 },
    XxInputPopupSurfaceV2 { version: u32 },
    XxInputPopupPositionerV1 { version: u32 },
    XxInputMethodManagerV2 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::XxInputMethodV1 { .. } => "xx_input_method_v1",
            Self::XxInputPopupSurfaceV2 { .. } => "xx_input_popup_surface_v2",
            Self::XxInputPopupPositionerV1 { .. } => "xx_input_popup_positioner_v1",
            Self::XxInputMethodManagerV2 { .. } => "xx_input_method_manager_v2",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::XxInputMethodV1 { version } => version,
            Self::XxInputPopupSurfaceV2 { version } => version,
            Self::XxInputPopupPositionerV1 { version } => version,
            Self::XxInputMethodManagerV2 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::XxInputMethodV1 { .. } => 4,
            Self::XxInputPopupSurfaceV2 { .. } => 1,
            Self::XxInputPopupPositionerV1 { .. } => 1,
            Self::XxInputMethodManagerV2 { .. } => 4,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "xx_input_method_v1" => Self::XxInputMethodV1 { version },
            "xx_input_popup_surface_v2" => Self::XxInputPopupSurfaceV2 { version },
            "xx_input_popup_positioner_v1" => Self::XxInputPopupPositionerV1 { version },
            "xx_input_method_manager_v2" => Self::XxInputMethodManagerV2 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
