//! Identifies Workspace interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtWorkspaceManagerV1 { version: u32 },
    ExtWorkspaceGroupHandleV1 { version: u32 },
    ExtWorkspaceHandleV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtWorkspaceManagerV1 { .. } => "ext_workspace_manager_v1",
            Self::ExtWorkspaceGroupHandleV1 { .. } => "ext_workspace_group_handle_v1",
            Self::ExtWorkspaceHandleV1 { .. } => "ext_workspace_handle_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtWorkspaceManagerV1 { version } => version,
            Self::ExtWorkspaceGroupHandleV1 { version } => version,
            Self::ExtWorkspaceHandleV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtWorkspaceManagerV1 { .. } => 1,
            Self::ExtWorkspaceGroupHandleV1 { .. } => 1,
            Self::ExtWorkspaceHandleV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_workspace_manager_v1" => Self::ExtWorkspaceManagerV1 { version },
            "ext_workspace_group_handle_v1" => Self::ExtWorkspaceGroupHandleV1 { version },
            "ext_workspace_handle_v1" => Self::ExtWorkspaceHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
