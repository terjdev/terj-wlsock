//! Identifies Linux drm syncobj interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpLinuxDrmSyncobjManagerV1 { version: u32 },
    WpLinuxDrmSyncobjTimelineV1 { version: u32 },
    WpLinuxDrmSyncobjSurfaceV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1 { .. } => "wp_linux_drm_syncobj_manager_v1",
            Self::WpLinuxDrmSyncobjTimelineV1 { .. } => "wp_linux_drm_syncobj_timeline_v1",
            Self::WpLinuxDrmSyncobjSurfaceV1 { .. } => "wp_linux_drm_syncobj_surface_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1 { version } => version,
            Self::WpLinuxDrmSyncobjTimelineV1 { version } => version,
            Self::WpLinuxDrmSyncobjSurfaceV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpLinuxDrmSyncobjManagerV1 { .. } => 1,
            Self::WpLinuxDrmSyncobjTimelineV1 { .. } => 1,
            Self::WpLinuxDrmSyncobjSurfaceV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_linux_drm_syncobj_manager_v1" => Self::WpLinuxDrmSyncobjManagerV1 { version },
            "wp_linux_drm_syncobj_timeline_v1" => Self::WpLinuxDrmSyncobjTimelineV1 { version },
            "wp_linux_drm_syncobj_surface_v1" => Self::WpLinuxDrmSyncobjSurfaceV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
