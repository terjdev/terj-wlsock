//! Identifies Security context interfaces and negotiated versions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    WpSecurityContextManagerV1 { version: u32 },
    WpSecurityContextV1 { version: u32 },
}
impl ObjectInteface {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpSecurityContextManagerV1 { .. } => "wp_security_context_manager_v1",
            Self::WpSecurityContextV1 { .. } => "wp_security_context_v1",
        }
    }
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpSecurityContextManagerV1 { version } => version,
            Self::WpSecurityContextV1 { version } => version,
        }
    }
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpSecurityContextManagerV1 { .. } => 1,
            Self::WpSecurityContextV1 { .. } => 1,
        }
    }
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_security_context_manager_v1" => Self::WpSecurityContextManagerV1 { version },
            "wp_security_context_v1" => Self::WpSecurityContextV1 { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
