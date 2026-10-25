//! Identifies Session lock interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtSessionLockManagerV1 { version: u32 },
    ExtSessionLockV1 { version: u32 },
    ExtSessionLockSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtSessionLockManagerV1 { .. } => "ext_session_lock_manager_v1",
            Self::ExtSessionLockV1 { .. } => "ext_session_lock_v1",
            Self::ExtSessionLockSurfaceV1 { .. } => "ext_session_lock_surface_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtSessionLockManagerV1 { version } => version,
            Self::ExtSessionLockV1 { version } => version,
            Self::ExtSessionLockSurfaceV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtSessionLockManagerV1 { .. } => 1,
            Self::ExtSessionLockV1 { .. } => 1,
            Self::ExtSessionLockSurfaceV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_session_lock_manager_v1" => Self::ExtSessionLockManagerV1 { version },
            "ext_session_lock_v1" => Self::ExtSessionLockV1 { version },
            "ext_session_lock_surface_v1" => Self::ExtSessionLockSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
