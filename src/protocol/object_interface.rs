//! Groups object interface metadata by protocol.

use crate::protocol::core;
#[cfg(feature = "ext-dmabuf")]
use crate::protocol::ext::dmabuf;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
#[cfg(feature = "ext-tablet")]
use crate::protocol::ext::tablet;
#[cfg(feature = "ext-viewporter")]
use crate::protocol::ext::viewporter;
#[cfg(feature = "ext-xdg-shell")]
use crate::protocol::ext::xdg_shell;
#[cfg(feature = "wlr-data-control")]
use crate::protocol::wlr::data_control;
#[cfg(feature = "wlr-layer-shell")]
use crate::protocol::wlr::layer_shell;
#[cfg(feature = "wlr-screencopy")]
use crate::protocol::wlr::screencopy;
#[cfg(feature = "wlr-virtual-pointer")]
use crate::protocol::wlr::virtual_pointer;

/// Identifies an object's protocol group, interface, and negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    /// A core Wayland interface.
    Core(core::ObjectInteface),
    /// A WLR screencopy interface.
    #[cfg(feature = "wlr-screencopy")]
    WlrScreencopy(screencopy::ObjectInteface),
    /// A WLR virtual pointer interface.
    #[cfg(feature = "wlr-virtual-pointer")]
    WlrVirtualPointer(virtual_pointer::ObjectInteface),
    /// A WLR data control interface.
    #[cfg(feature = "wlr-data-control")]
    WlrDataControl(data_control::ObjectInteface),
    /// A WLR layer shell interface.
    #[cfg(feature = "wlr-layer-shell")]
    WlrLayerShell(layer_shell::ObjectInteface),
    /// A Tablet interface.
    #[cfg(feature = "ext-tablet")]
    Tablet(tablet::ObjectInteface),
    /// A XDG shell interface.
    #[cfg(feature = "ext-xdg-shell")]
    XdgShell(xdg_shell::ObjectInteface),
    /// A Linux DMA-BUF interface.
    #[cfg(feature = "ext-dmabuf")]
    Dmabuf(dmabuf::ObjectInteface),
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
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(interface) => interface.name(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(interface) => interface.name(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(interface) => interface.name(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(interface) => interface.name(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(interface) => interface.name(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(interface) => interface.name(),
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
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(interface) => interface.version(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(interface) => interface.version(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(interface) => interface.version(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(interface) => interface.version(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(interface) => interface.version(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(interface) => interface.version(),
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
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(interface) => interface.max_version(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(interface) => interface.max_version(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(interface) => interface.max_version(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(interface) => interface.max_version(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(interface) => interface.max_version(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(interface) => interface.max_version(),
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
        #[cfg(feature = "ext-dmabuf")]
        if let Some(interface) = dmabuf::ObjectInteface::from_name(name, version) {
            return Some(Self::Dmabuf(interface));
        }
        #[cfg(feature = "ext-xdg-shell")]
        if let Some(interface) = xdg_shell::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgShell(interface));
        }
        #[cfg(feature = "ext-tablet")]
        if let Some(interface) = tablet::ObjectInteface::from_name(name, version) {
            return Some(Self::Tablet(interface));
        }
        #[cfg(feature = "wlr-layer-shell")]
        if let Some(interface) = layer_shell::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrLayerShell(interface));
        }
        #[cfg(feature = "wlr-data-control")]
        if let Some(interface) = data_control::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrDataControl(interface));
        }
        #[cfg(feature = "wlr-virtual-pointer")]
        if let Some(interface) = virtual_pointer::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrVirtualPointer(interface));
        }
        #[cfg(feature = "wlr-screencopy")]
        if let Some(interface) = screencopy::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrScreencopy(interface));
        }
        None
    }
}
