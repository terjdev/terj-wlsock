//! Defines interfaces and negotiated versions for this protocol.
/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    ZwpPointerConstraintsV1 { version: u32 },
    ZwpLockedPointerV1 { version: u32 },
    ZwpConfinedPointerV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ZwpPointerConstraintsV1 { .. } => "zwp_pointer_constraints_v1",
            Self::ZwpLockedPointerV1 { .. } => "zwp_locked_pointer_v1",
            Self::ZwpConfinedPointerV1 { .. } => "zwp_confined_pointer_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::ZwpPointerConstraintsV1 { version } => version,
            Self::ZwpLockedPointerV1 { version } => version,
            Self::ZwpConfinedPointerV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::ZwpPointerConstraintsV1 { .. } => 1,
            Self::ZwpLockedPointerV1 { .. } => 1,
            Self::ZwpConfinedPointerV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "zwp_pointer_constraints_v1" => Self::ZwpPointerConstraintsV1 { version },
            "zwp_locked_pointer_v1" => Self::ZwpLockedPointerV1 { version },
            "zwp_confined_pointer_v1" => Self::ZwpConfinedPointerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
