//! Defines Presentation time object interfaces and their negotiated versions.

/// Identifies a client object interface and its negotiated protocol version.
///
/// Object registration validates the negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    /// Object of interface `wp_presentation`; maximum supported version: 3.
    WpPresentation { version: u32 },
    /// Object of interface `wp_presentation_feedback`; maximum supported version: 3.
    WpPresentationFeedback { version: u32 },
}
impl ObjectInteface {
    /// Returns the interface name defined in XML.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::WpPresentationFeedback { .. } => "wp_presentation_feedback",
            Self::WpPresentation { .. } => "wp_presentation",
        }
    }
    /// Returns the negotiated object version.
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::WpPresentationFeedback { version } => version,
            Self::WpPresentation { version } => version,
        }
    }
    /// Returns the highest supported version of this interface.
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::WpPresentationFeedback { .. } => 3,
            Self::WpPresentation { .. } => 3,
        }
    }
    /// Creates a known interface, rejecting zero or unsupported versions.
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        let interface = match name {
            "wp_presentation_feedback" => Self::WpPresentationFeedback { version },
            "wp_presentation" => Self::WpPresentation { version },
            _ => return None,
        };
        (version > 0 && version <= interface.max_version()).then_some(interface)
    }
}
