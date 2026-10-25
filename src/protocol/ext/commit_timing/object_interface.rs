//! Identifies Commit timing interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpCommitTimingManagerV1 { version: u32 },
    WpCommitTimerV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpCommitTimingManagerV1 { .. } => "wp_commit_timing_manager_v1",
            Self::WpCommitTimerV1 { .. } => "wp_commit_timer_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpCommitTimingManagerV1 { version } => version,
            Self::WpCommitTimerV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpCommitTimingManagerV1 { .. } => 1,
            Self::WpCommitTimerV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_commit_timing_manager_v1" => Self::WpCommitTimingManagerV1 { version },
            "wp_commit_timer_v1" => Self::WpCommitTimerV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
