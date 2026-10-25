//! Groups object interface metadata by protocol.

use crate::protocol::core;
#[cfg(feature = "ext-alpha-modifier")]
use crate::protocol::ext::alpha_modifier;
#[cfg(feature = "ext-background-effect")]
use crate::protocol::ext::background_effect;
#[cfg(feature = "ext-commit-timing")]
use crate::protocol::ext::commit_timing;
#[cfg(feature = "ext-content-type")]
use crate::protocol::ext::content_type;
#[cfg(feature = "ext-cursor-shape")]
use crate::protocol::ext::cursor_shape;
#[cfg(feature = "ext-data-control")]
use crate::protocol::ext::data_control as ext_data_control;
#[cfg(feature = "ext-dmabuf")]
use crate::protocol::ext::dmabuf;
#[cfg(feature = "ext-drm-lease")]
use crate::protocol::ext::drm_lease;
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
    /// A Data control interface.
    #[cfg(feature = "ext-data-control")]
    DataControl(ext_data_control::ObjectInteface),
    /// A Background effect interface.
    #[cfg(feature = "ext-background-effect")]
    BackgroundEffect(background_effect::ObjectInteface),
    /// A DRM lease interface.
    #[cfg(feature = "ext-drm-lease")]
    DrmLease(drm_lease::ObjectInteface),
    /// A Cursor shape interface.
    #[cfg(feature = "ext-cursor-shape")]
    CursorShape(cursor_shape::ObjectInteface),
    /// A Content type interface.
    #[cfg(feature = "ext-content-type")]
    ContentType(content_type::ObjectInteface),
    /// A Commit timing interface.
    #[cfg(feature = "ext-commit-timing")]
    CommitTiming(commit_timing::ObjectInteface),
    /// A Alpha modifier interface.
    #[cfg(feature = "ext-alpha-modifier")]
    AlphaModifier(alpha_modifier::ObjectInteface),
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
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(interface) => interface.name(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(interface) => interface.name(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(interface) => interface.name(),
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(interface) => interface.name(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(interface) => interface.name(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(interface) => interface.name(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(interface) => interface.name(),
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
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(interface) => interface.version(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(interface) => interface.version(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(interface) => interface.version(),
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(interface) => interface.version(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(interface) => interface.version(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(interface) => interface.version(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(interface) => interface.version(),
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
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(interface) => interface.max_version(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(interface) => interface.max_version(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(interface) => interface.max_version(),
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(interface) => interface.max_version(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(interface) => interface.max_version(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(interface) => interface.max_version(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(interface) => interface.max_version(),
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
        #[cfg(feature = "ext-alpha-modifier")]
        if let Some(interface) = alpha_modifier::ObjectInteface::from_name(name, version) {
            return Some(Self::AlphaModifier(interface));
        }
        #[cfg(feature = "ext-commit-timing")]
        if let Some(interface) = commit_timing::ObjectInteface::from_name(name, version) {
            return Some(Self::CommitTiming(interface));
        }
        #[cfg(feature = "ext-content-type")]
        if let Some(interface) = content_type::ObjectInteface::from_name(name, version) {
            return Some(Self::ContentType(interface));
        }
        #[cfg(feature = "ext-cursor-shape")]
        if let Some(interface) = cursor_shape::ObjectInteface::from_name(name, version) {
            return Some(Self::CursorShape(interface));
        }
        #[cfg(feature = "ext-drm-lease")]
        if let Some(interface) = drm_lease::ObjectInteface::from_name(name, version) {
            return Some(Self::DrmLease(interface));
        }
        #[cfg(feature = "ext-background-effect")]
        if let Some(interface) = background_effect::ObjectInteface::from_name(name, version) {
            return Some(Self::BackgroundEffect(interface));
        }
        #[cfg(feature = "ext-data-control")]
        if let Some(interface) = ext_data_control::ObjectInteface::from_name(name, version) {
            return Some(Self::DataControl(interface));
        }
        None
    }
}
