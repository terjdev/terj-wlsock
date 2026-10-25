//! Defines interfaces and negotiated versions for this protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ZwpFullscreenShellV1 { version: u32 },
    ZwpFullscreenShellModeFeedbackV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ZwpFullscreenShellV1 { .. } => "zwp_fullscreen_shell_v1",
            Self::ZwpFullscreenShellModeFeedbackV1 { .. } => {
                "zwp_fullscreen_shell_mode_feedback_v1"
            }
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ZwpFullscreenShellV1 { version } => version,
            Self::ZwpFullscreenShellModeFeedbackV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ZwpFullscreenShellV1 { .. } => 1,
            Self::ZwpFullscreenShellModeFeedbackV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_fullscreen_shell_v1" => Self::ZwpFullscreenShellV1 { version },
            "zwp_fullscreen_shell_mode_feedback_v1" => {
                Self::ZwpFullscreenShellModeFeedbackV1 { version }
            }
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
