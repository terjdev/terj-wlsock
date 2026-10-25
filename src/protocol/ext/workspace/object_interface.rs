//! Identifies Workspace interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtWorkspaceManagerV1 { version: u32 },
    ExtWorkspaceGroupHandleV1 { version: u32 },
    ExtWorkspaceHandleV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtWorkspaceManagerV1 { .. } => "ext_workspace_manager_v1",
            Self::ExtWorkspaceGroupHandleV1 { .. } => "ext_workspace_group_handle_v1",
            Self::ExtWorkspaceHandleV1 { .. } => "ext_workspace_handle_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtWorkspaceManagerV1 { version } => version,
            Self::ExtWorkspaceGroupHandleV1 { version } => version,
            Self::ExtWorkspaceHandleV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtWorkspaceManagerV1 { .. } => 1,
            Self::ExtWorkspaceGroupHandleV1 { .. } => 1,
            Self::ExtWorkspaceHandleV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_workspace_manager_v1" => Self::ExtWorkspaceManagerV1 { version },
            "ext_workspace_group_handle_v1" => Self::ExtWorkspaceGroupHandleV1 { version },
            "ext_workspace_handle_v1" => Self::ExtWorkspaceHandleV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
