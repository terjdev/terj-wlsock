//! Identifies Session lock interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ExtSessionLockManagerV1 { version: u32 },
    ExtSessionLockV1 { version: u32 },
    ExtSessionLockSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ExtSessionLockManagerV1 { .. } => "ext_session_lock_manager_v1",
            Self::ExtSessionLockV1 { .. } => "ext_session_lock_v1",
            Self::ExtSessionLockSurfaceV1 { .. } => "ext_session_lock_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ExtSessionLockManagerV1 { version } => version,
            Self::ExtSessionLockV1 { version } => version,
            Self::ExtSessionLockSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ExtSessionLockManagerV1 { .. } => 1,
            Self::ExtSessionLockV1 { .. } => 1,
            Self::ExtSessionLockSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_session_lock_manager_v1" => Self::ExtSessionLockManagerV1 { version },
            "ext_session_lock_v1" => Self::ExtSessionLockV1 { version },
            "ext_session_lock_surface_v1" => Self::ExtSessionLockSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
