//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpLinuxExplicitSynchronizationV1 { version: u32 },
    ZwpLinuxSurfaceSynchronizationV1 { version: u32 },
    ZwpLinuxBufferReleaseV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1 { .. } => {
                "zwp_linux_explicit_synchronization_v1"
            }
            Self::ZwpLinuxSurfaceSynchronizationV1 { .. } => "zwp_linux_surface_synchronization_v1",
            Self::ZwpLinuxBufferReleaseV1 { .. } => "zwp_linux_buffer_release_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1 { version } => version,
            Self::ZwpLinuxSurfaceSynchronizationV1 { version } => version,
            Self::ZwpLinuxBufferReleaseV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpLinuxExplicitSynchronizationV1 { .. } => 2,
            Self::ZwpLinuxSurfaceSynchronizationV1 { .. } => 2,
            Self::ZwpLinuxBufferReleaseV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_linux_explicit_synchronization_v1" => {
                Self::ZwpLinuxExplicitSynchronizationV1 { version }
            }
            "zwp_linux_surface_synchronization_v1" => {
                Self::ZwpLinuxSurfaceSynchronizationV1 { version }
            }
            "zwp_linux_buffer_release_v1" => Self::ZwpLinuxBufferReleaseV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
