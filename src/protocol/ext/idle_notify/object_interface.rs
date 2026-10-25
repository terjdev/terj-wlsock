//! Identifies Idle notify interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    ExtIdleNotifierV1 { version: u32 },
    ExtIdleNotificationV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ExtIdleNotifierV1 { .. } => "ext_idle_notifier_v1",
            Self::ExtIdleNotificationV1 { .. } => "ext_idle_notification_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::ExtIdleNotifierV1 { version } => version,
            Self::ExtIdleNotificationV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::ExtIdleNotifierV1 { .. } => 2,
            Self::ExtIdleNotificationV1 { .. } => 2,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "ext_idle_notifier_v1" => Self::ExtIdleNotifierV1 { version },
            "ext_idle_notification_v1" => Self::ExtIdleNotificationV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
