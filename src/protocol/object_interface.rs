//! Groups object interface metadata by protocol.

use crate::protocol::core;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
#[cfg(feature = "ext-viewporter")]
use crate::protocol::ext::viewporter;

/// Identifies an object's protocol group, interface, and negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    /// A core Wayland interface.
    Core(core::ObjectInteface),
    /// A Presentation time interface.
    #[cfg(feature = "ext-presentation-time")]
    PresentationTime(presentation_time::ObjectInteface),
    /// A Viewporter interface.
    #[cfg(feature = "ext-viewporter")]
    Viewporter(viewporter::ObjectInteface),
}

impl ObjectInteface {
    /// Returns the interface name defined in XML.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Core(interface) => interface.name(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(interface) => interface.name(),
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(interface) => interface.name(),
        }
    }
    /// Returns the negotiated object version.
    pub(crate) fn version(self) -> u32 {
        match self {
            Self::Core(interface) => interface.version(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(interface) => interface.version(),
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(interface) => interface.version(),
        }
    }
    /// Returns the highest supported version of this interface.
    pub(crate) fn max_version(self) -> u32 {
        match self {
            Self::Core(interface) => interface.max_version(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(interface) => interface.max_version(),
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(interface) => interface.max_version(),
        }
    }
    /// Creates a known interface, rejecting zero or unsupported versions.
    pub(crate) fn from_name(name: &str, version: u32) -> Option<Self> {
        if let Some(interface) = core::ObjectInteface::from_name(name, version) {
            return Some(Self::Core(interface));
        }
        #[cfg(feature = "ext-presentation-time")]
        if let Some(interface) = presentation_time::ObjectInteface::from_name(name, version) {
            return Some(Self::PresentationTime(interface));
        }
        #[cfg(feature = "ext-viewporter")]
        if let Some(interface) = viewporter::ObjectInteface::from_name(name, version) {
            return Some(Self::Viewporter(interface));
        }
        None
    }
}
