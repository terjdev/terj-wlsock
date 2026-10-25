//! Identifies Commit timing interfaces and negotiated versions.

/// Identifies an interface and its stored protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectInteface {
    WpCommitTimingManagerV1 { version: u32 },
    WpCommitTimerV1 { version: u32 },
}
impl ObjectInteface {
    /// Returns the wire interface name.
    pub fn name(self) -> &'static str {
        match self {
            Self::WpCommitTimingManagerV1 { .. } => "wp_commit_timing_manager_v1",
            Self::WpCommitTimerV1 { .. } => "wp_commit_timer_v1",
        }
    }
    /// Returns the stored object version.
    pub fn version(self) -> u32 {
        match self {
            Self::WpCommitTimingManagerV1 { version } => version,
            Self::WpCommitTimerV1 { version } => version,
        }
    }
    /// Returns the highest interface version supported by this library.
    pub fn max_version(self) -> u32 {
        match self {
            Self::WpCommitTimingManagerV1 { .. } => 1,
            Self::WpCommitTimerV1 { .. } => 1,
        }
    }
    /// Creates a known interface, returning `None` for unknown names or unsupported versions.
    pub fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_commit_timing_manager_v1" => Self::WpCommitTimingManagerV1 { version },
            "wp_commit_timer_v1" => Self::WpCommitTimerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
