//! Groups object interface metadata by protocol.

use crate::protocol::core;
#[cfg(feature = "ext-alpha-modifier")]
use crate::protocol::ext::alpha_modifier;
#[cfg(feature = "ext-background-effect")]
use crate::protocol::ext::background_effect;
#[cfg(feature = "ext-color-management")]
use crate::protocol::ext::color_management;
#[cfg(feature = "ext-color-representation")]
use crate::protocol::ext::color_representation;
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
#[cfg(feature = "ext-fifo")]
use crate::protocol::ext::fifo;
#[cfg(feature = "ext-foreign-toplevel-list")]
use crate::protocol::ext::foreign_toplevel_list;
#[cfg(feature = "ext-fractional-scale")]
use crate::protocol::ext::fractional_scale;
#[cfg(feature = "ext-fullscreen-shell")]
use crate::protocol::ext::fullscreen_shell;
#[cfg(feature = "ext-idle-inhibit")]
use crate::protocol::ext::idle_inhibit;
#[cfg(feature = "ext-idle-notify")]
use crate::protocol::ext::idle_notify;
#[cfg(feature = "ext-image-capture-source")]
use crate::protocol::ext::image_capture_source;
#[cfg(feature = "ext-image-copy-capture")]
use crate::protocol::ext::image_copy_capture;
#[cfg(feature = "ext-input-method")]
use crate::protocol::ext::input_method;
#[cfg(feature = "ext-input-timestamps")]
use crate::protocol::ext::input_timestamps;
#[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
use crate::protocol::ext::keyboard_shortcuts_inhibit;
#[cfg(feature = "ext-linux-drm-syncobj")]
use crate::protocol::ext::linux_drm_syncobj;
#[cfg(feature = "ext-linux-explicit-synchronization")]
use crate::protocol::ext::linux_explicit_synchronization;
#[cfg(feature = "ext-pointer-constraints")]
use crate::protocol::ext::pointer_constraints;
#[cfg(feature = "ext-pointer-gestures")]
use crate::protocol::ext::pointer_gestures;
#[cfg(feature = "ext-pointer-warp")]
use crate::protocol::ext::pointer_warp;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
#[cfg(feature = "ext-primary-selection")]
use crate::protocol::ext::primary_selection;
#[cfg(feature = "ext-relative-pointer")]
use crate::protocol::ext::relative_pointer;
#[cfg(feature = "ext-security-context")]
use crate::protocol::ext::security_context;
#[cfg(feature = "ext-session-lock")]
use crate::protocol::ext::session_lock;
#[cfg(feature = "ext-single-pixel-buffer")]
use crate::protocol::ext::single_pixel_buffer;
#[cfg(feature = "ext-tablet")]
use crate::protocol::ext::tablet;
#[cfg(feature = "ext-tearing-control")]
use crate::protocol::ext::tearing_control;
#[cfg(feature = "ext-text-input")]
use crate::protocol::ext::text_input;
#[cfg(feature = "ext-transient-seat")]
use crate::protocol::ext::transient_seat;
#[cfg(feature = "ext-viewporter")]
use crate::protocol::ext::viewporter;
#[cfg(feature = "ext-workspace")]
use crate::protocol::ext::workspace;
#[cfg(feature = "ext-xdg-activation")]
use crate::protocol::ext::xdg_activation;
#[cfg(feature = "ext-xdg-decoration")]
use crate::protocol::ext::xdg_decoration;
#[cfg(feature = "ext-xdg-dialog")]
use crate::protocol::ext::xdg_dialog;
#[cfg(feature = "ext-xdg-foreign")]
use crate::protocol::ext::xdg_foreign;
#[cfg(feature = "ext-xdg-output")]
use crate::protocol::ext::xdg_output;
#[cfg(feature = "ext-xdg-session-management")]
use crate::protocol::ext::xdg_session_management;
#[cfg(feature = "ext-xdg-shell")]
use crate::protocol::ext::xdg_shell;
#[cfg(feature = "ext-xdg-system-bell")]
use crate::protocol::ext::xdg_system_bell;
#[cfg(feature = "ext-xdg-toplevel-drag")]
use crate::protocol::ext::xdg_toplevel_drag;
#[cfg(feature = "ext-xdg-toplevel-icon")]
use crate::protocol::ext::xdg_toplevel_icon;
#[cfg(feature = "ext-xdg-toplevel-tag")]
use crate::protocol::ext::xdg_toplevel_tag;
#[cfg(feature = "ext-xwayland-keyboard-grab")]
use crate::protocol::ext::xwayland_keyboard_grab;
#[cfg(feature = "ext-xwayland-shell")]
use crate::protocol::ext::xwayland_shell;
#[cfg(feature = "wlr-data-control")]
use crate::protocol::wlr::data_control;
#[cfg(feature = "wlr-export-dmabuf")]
use crate::protocol::wlr::export_dmabuf;
#[cfg(feature = "wlr-foreign-toplevel-management")]
use crate::protocol::wlr::foreign_toplevel_management;
#[cfg(feature = "wlr-gamma-control")]
use crate::protocol::wlr::gamma_control;
#[cfg(feature = "wlr-input-inhibitor")]
use crate::protocol::wlr::input_inhibitor;
#[cfg(feature = "wlr-layer-shell")]
use crate::protocol::wlr::layer_shell;
#[cfg(feature = "wlr-output-management")]
use crate::protocol::wlr::output_management;
#[cfg(feature = "wlr-output-power-management")]
use crate::protocol::wlr::output_power_management;
#[cfg(feature = "wlr-screencopy")]
use crate::protocol::wlr::screencopy;
#[cfg(feature = "wlr-virtual-pointer")]
use crate::protocol::wlr::virtual_pointer;

/// Identifies an object's protocol group, interface, and negotiated version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectInteface {
    /// A core Wayland interface.
    Core(core::ObjectInteface),
    /// A output power management interface.
    #[cfg(feature = "wlr-output-power-management")]
    WlrOutputPowerManagement(output_power_management::ObjectInteface),
    /// A output management interface.
    #[cfg(feature = "wlr-output-management")]
    WlrOutputManagement(output_management::ObjectInteface),
    /// A input inhibitor interface.
    #[cfg(feature = "wlr-input-inhibitor")]
    WlrInputInhibitor(input_inhibitor::ObjectInteface),
    /// A gamma control interface.
    #[cfg(feature = "wlr-gamma-control")]
    WlrGammaControl(gamma_control::ObjectInteface),
    /// A foreign toplevel management interface.
    #[cfg(feature = "wlr-foreign-toplevel-management")]
    WlrForeignToplevelManagement(foreign_toplevel_management::ObjectInteface),
    /// A export dmabuf interface.
    #[cfg(feature = "wlr-export-dmabuf")]
    WlrExportDmabuf(export_dmabuf::ObjectInteface),
    /// A xwayland keyboard grab interface.
    #[cfg(feature = "ext-xwayland-keyboard-grab")]
    XwaylandKeyboardGrab(xwayland_keyboard_grab::ObjectInteface),
    /// A xdg output interface.
    #[cfg(feature = "ext-xdg-output")]
    XdgOutput(xdg_output::ObjectInteface),
    /// A xdg foreign interface.
    #[cfg(feature = "ext-xdg-foreign")]
    XdgForeign(xdg_foreign::ObjectInteface),
    /// A xdg decoration interface.
    #[cfg(feature = "ext-xdg-decoration")]
    XdgDecoration(xdg_decoration::ObjectInteface),
    /// A text input interface.
    #[cfg(feature = "ext-text-input")]
    TextInput(text_input::ObjectInteface),
    /// A relative pointer interface.
    #[cfg(feature = "ext-relative-pointer")]
    RelativePointer(relative_pointer::ObjectInteface),
    /// A primary selection interface.
    #[cfg(feature = "ext-primary-selection")]
    PrimarySelection(primary_selection::ObjectInteface),
    /// A pointer gestures interface.
    #[cfg(feature = "ext-pointer-gestures")]
    PointerGestures(pointer_gestures::ObjectInteface),
    /// A pointer constraints interface.
    #[cfg(feature = "ext-pointer-constraints")]
    PointerConstraints(pointer_constraints::ObjectInteface),
    /// A linux explicit synchronization interface.
    #[cfg(feature = "ext-linux-explicit-synchronization")]
    LinuxExplicitSynchronization(linux_explicit_synchronization::ObjectInteface),
    /// A keyboard shortcuts inhibit interface.
    #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
    KeyboardShortcutsInhibit(keyboard_shortcuts_inhibit::ObjectInteface),
    /// A input timestamps interface.
    #[cfg(feature = "ext-input-timestamps")]
    InputTimestamps(input_timestamps::ObjectInteface),
    /// A input method interface.
    #[cfg(feature = "ext-input-method")]
    InputMethod(input_method::ObjectInteface),
    /// A idle inhibit interface.
    #[cfg(feature = "ext-idle-inhibit")]
    IdleInhibit(idle_inhibit::ObjectInteface),
    /// A fullscreen shell interface.
    #[cfg(feature = "ext-fullscreen-shell")]
    FullscreenShell(fullscreen_shell::ObjectInteface),
    /// A Xwayland shell interface.
    #[cfg(feature = "ext-xwayland-shell")]
    XwaylandShell(xwayland_shell::ObjectInteface),
    /// A Xdg toplevel tag interface.
    #[cfg(feature = "ext-xdg-toplevel-tag")]
    XdgToplevelTag(xdg_toplevel_tag::ObjectInteface),
    /// A Xdg toplevel icon interface.
    #[cfg(feature = "ext-xdg-toplevel-icon")]
    XdgToplevelIcon(xdg_toplevel_icon::ObjectInteface),
    /// A Xdg toplevel drag interface.
    #[cfg(feature = "ext-xdg-toplevel-drag")]
    XdgToplevelDrag(xdg_toplevel_drag::ObjectInteface),
    /// A Xdg system bell interface.
    #[cfg(feature = "ext-xdg-system-bell")]
    XdgSystemBell(xdg_system_bell::ObjectInteface),
    /// A Xdg session management interface.
    #[cfg(feature = "ext-xdg-session-management")]
    XdgSessionManagement(xdg_session_management::ObjectInteface),
    /// A Xdg dialog interface.
    #[cfg(feature = "ext-xdg-dialog")]
    XdgDialog(xdg_dialog::ObjectInteface),
    /// A Xdg activation interface.
    #[cfg(feature = "ext-xdg-activation")]
    XdgActivation(xdg_activation::ObjectInteface),
    /// A Tearing control interface.
    #[cfg(feature = "ext-tearing-control")]
    TearingControl(tearing_control::ObjectInteface),
    /// A Single pixel buffer interface.
    #[cfg(feature = "ext-single-pixel-buffer")]
    SinglePixelBuffer(single_pixel_buffer::ObjectInteface),
    /// A Security context interface.
    #[cfg(feature = "ext-security-context")]
    SecurityContext(security_context::ObjectInteface),
    /// A Pointer warp interface.
    #[cfg(feature = "ext-pointer-warp")]
    PointerWarp(pointer_warp::ObjectInteface),
    /// A Linux drm syncobj interface.
    #[cfg(feature = "ext-linux-drm-syncobj")]
    LinuxDrmSyncobj(linux_drm_syncobj::ObjectInteface),
    /// A Fractional scale interface.
    #[cfg(feature = "ext-fractional-scale")]
    FractionalScale(fractional_scale::ObjectInteface),
    /// A Fifo interface.
    #[cfg(feature = "ext-fifo")]
    Fifo(fifo::ObjectInteface),
    /// A Workspace interface.
    #[cfg(feature = "ext-workspace")]
    Workspace(workspace::ObjectInteface),
    /// A Transient seat interface.
    #[cfg(feature = "ext-transient-seat")]
    TransientSeat(transient_seat::ObjectInteface),
    /// A Session lock interface.
    #[cfg(feature = "ext-session-lock")]
    SessionLock(session_lock::ObjectInteface),
    /// A Image copy capture interface.
    #[cfg(feature = "ext-image-copy-capture")]
    ImageCopyCapture(image_copy_capture::ObjectInteface),
    /// A Image capture source interface.
    #[cfg(feature = "ext-image-capture-source")]
    ImageCaptureSource(image_capture_source::ObjectInteface),
    /// A Idle notify interface.
    #[cfg(feature = "ext-idle-notify")]
    IdleNotify(idle_notify::ObjectInteface),
    /// A Foreign toplevel list interface.
    #[cfg(feature = "ext-foreign-toplevel-list")]
    ForeignToplevelList(foreign_toplevel_list::ObjectInteface),
    /// A Color representation interface.
    #[cfg(feature = "ext-color-representation")]
    ColorRepresentation(color_representation::ObjectInteface),
    /// A Color management interface.
    #[cfg(feature = "ext-color-management")]
    ColorManagement(color_management::ObjectInteface),
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
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(interface) => interface.name(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(interface) => interface.name(),
            #[cfg(feature = "wlr-input-inhibitor")]
            Self::WlrInputInhibitor(interface) => interface.name(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(interface) => interface.name(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(interface) => interface.name(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(interface) => interface.name(),
            #[cfg(feature = "ext-xwayland-keyboard-grab")]
            Self::XwaylandKeyboardGrab(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(interface) => interface.name(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(interface) => interface.name(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(interface) => interface.name(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(interface) => interface.name(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(interface) => interface.name(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(interface) => interface.name(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(interface) => interface.name(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(interface) => interface.name(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(interface) => interface.name(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(interface) => interface.name(),
            #[cfg(feature = "ext-idle-inhibit")]
            Self::IdleInhibit(interface) => interface.name(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(interface) => interface.name(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(interface) => interface.name(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(interface) => interface.name(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(interface) => interface.name(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(interface) => interface.name(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(interface) => interface.name(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(interface) => interface.name(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(interface) => interface.name(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(interface) => interface.name(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(interface) => interface.name(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(interface) => interface.name(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(interface) => interface.name(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(interface) => interface.name(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(interface) => interface.name(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(interface) => interface.name(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(interface) => interface.name(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(interface) => interface.name(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(interface) => interface.name(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(interface) => interface.name(),
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
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(interface) => interface.version(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(interface) => interface.version(),
            #[cfg(feature = "wlr-input-inhibitor")]
            Self::WlrInputInhibitor(interface) => interface.version(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(interface) => interface.version(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(interface) => interface.version(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(interface) => interface.version(),
            #[cfg(feature = "ext-xwayland-keyboard-grab")]
            Self::XwaylandKeyboardGrab(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(interface) => interface.version(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(interface) => interface.version(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(interface) => interface.version(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(interface) => interface.version(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(interface) => interface.version(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(interface) => interface.version(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(interface) => interface.version(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(interface) => interface.version(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(interface) => interface.version(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(interface) => interface.version(),
            #[cfg(feature = "ext-idle-inhibit")]
            Self::IdleInhibit(interface) => interface.version(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(interface) => interface.version(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(interface) => interface.version(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(interface) => interface.version(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(interface) => interface.version(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(interface) => interface.version(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(interface) => interface.version(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(interface) => interface.version(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(interface) => interface.version(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(interface) => interface.version(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(interface) => interface.version(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(interface) => interface.version(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(interface) => interface.version(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(interface) => interface.version(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(interface) => interface.version(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(interface) => interface.version(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(interface) => interface.version(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(interface) => interface.version(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(interface) => interface.version(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(interface) => interface.version(),
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
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(interface) => interface.max_version(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(interface) => interface.max_version(),
            #[cfg(feature = "wlr-input-inhibitor")]
            Self::WlrInputInhibitor(interface) => interface.max_version(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(interface) => interface.max_version(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(interface) => interface.max_version(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(interface) => interface.max_version(),
            #[cfg(feature = "ext-xwayland-keyboard-grab")]
            Self::XwaylandKeyboardGrab(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(interface) => interface.max_version(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(interface) => interface.max_version(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(interface) => interface.max_version(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(interface) => interface.max_version(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(interface) => interface.max_version(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(interface) => interface.max_version(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(interface) => interface.max_version(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(interface) => interface.max_version(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(interface) => interface.max_version(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(interface) => interface.max_version(),
            #[cfg(feature = "ext-idle-inhibit")]
            Self::IdleInhibit(interface) => interface.max_version(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(interface) => interface.max_version(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(interface) => interface.max_version(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(interface) => interface.max_version(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(interface) => interface.max_version(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(interface) => interface.max_version(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(interface) => interface.max_version(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(interface) => interface.max_version(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(interface) => interface.max_version(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(interface) => interface.max_version(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(interface) => interface.max_version(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(interface) => interface.max_version(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(interface) => interface.max_version(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(interface) => interface.max_version(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(interface) => interface.max_version(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(interface) => interface.max_version(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(interface) => interface.max_version(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(interface) => interface.max_version(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(interface) => interface.max_version(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(interface) => interface.max_version(),
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
        #[cfg(feature = "ext-color-management")]
        if let Some(interface) = color_management::ObjectInteface::from_name(name, version) {
            return Some(Self::ColorManagement(interface));
        }
        #[cfg(feature = "ext-color-representation")]
        if let Some(interface) = color_representation::ObjectInteface::from_name(name, version) {
            return Some(Self::ColorRepresentation(interface));
        }
        #[cfg(feature = "ext-foreign-toplevel-list")]
        if let Some(interface) = foreign_toplevel_list::ObjectInteface::from_name(name, version) {
            return Some(Self::ForeignToplevelList(interface));
        }
        #[cfg(feature = "ext-idle-notify")]
        if let Some(interface) = idle_notify::ObjectInteface::from_name(name, version) {
            return Some(Self::IdleNotify(interface));
        }
        #[cfg(feature = "ext-image-capture-source")]
        if let Some(interface) = image_capture_source::ObjectInteface::from_name(name, version) {
            return Some(Self::ImageCaptureSource(interface));
        }
        #[cfg(feature = "ext-image-copy-capture")]
        if let Some(interface) = image_copy_capture::ObjectInteface::from_name(name, version) {
            return Some(Self::ImageCopyCapture(interface));
        }
        #[cfg(feature = "ext-session-lock")]
        if let Some(interface) = session_lock::ObjectInteface::from_name(name, version) {
            return Some(Self::SessionLock(interface));
        }
        #[cfg(feature = "ext-transient-seat")]
        if let Some(interface) = transient_seat::ObjectInteface::from_name(name, version) {
            return Some(Self::TransientSeat(interface));
        }
        #[cfg(feature = "ext-workspace")]
        if let Some(interface) = workspace::ObjectInteface::from_name(name, version) {
            return Some(Self::Workspace(interface));
        }
        #[cfg(feature = "ext-fifo")]
        if let Some(interface) = fifo::ObjectInteface::from_name(name, version) {
            return Some(Self::Fifo(interface));
        }
        #[cfg(feature = "ext-fractional-scale")]
        if let Some(interface) = fractional_scale::ObjectInteface::from_name(name, version) {
            return Some(Self::FractionalScale(interface));
        }
        #[cfg(feature = "ext-linux-drm-syncobj")]
        if let Some(interface) = linux_drm_syncobj::ObjectInteface::from_name(name, version) {
            return Some(Self::LinuxDrmSyncobj(interface));
        }
        #[cfg(feature = "ext-pointer-warp")]
        if let Some(interface) = pointer_warp::ObjectInteface::from_name(name, version) {
            return Some(Self::PointerWarp(interface));
        }
        #[cfg(feature = "ext-security-context")]
        if let Some(interface) = security_context::ObjectInteface::from_name(name, version) {
            return Some(Self::SecurityContext(interface));
        }
        #[cfg(feature = "ext-single-pixel-buffer")]
        if let Some(interface) = single_pixel_buffer::ObjectInteface::from_name(name, version) {
            return Some(Self::SinglePixelBuffer(interface));
        }
        #[cfg(feature = "ext-tearing-control")]
        if let Some(interface) = tearing_control::ObjectInteface::from_name(name, version) {
            return Some(Self::TearingControl(interface));
        }
        #[cfg(feature = "ext-xdg-activation")]
        if let Some(interface) = xdg_activation::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgActivation(interface));
        }
        #[cfg(feature = "ext-xdg-dialog")]
        if let Some(interface) = xdg_dialog::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgDialog(interface));
        }
        #[cfg(feature = "ext-xdg-session-management")]
        if let Some(interface) = xdg_session_management::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgSessionManagement(interface));
        }
        #[cfg(feature = "ext-xdg-system-bell")]
        if let Some(interface) = xdg_system_bell::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgSystemBell(interface));
        }
        #[cfg(feature = "ext-xdg-toplevel-drag")]
        if let Some(interface) = xdg_toplevel_drag::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgToplevelDrag(interface));
        }
        #[cfg(feature = "ext-xdg-toplevel-icon")]
        if let Some(interface) = xdg_toplevel_icon::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgToplevelIcon(interface));
        }
        #[cfg(feature = "ext-xdg-toplevel-tag")]
        if let Some(interface) = xdg_toplevel_tag::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgToplevelTag(interface));
        }
        #[cfg(feature = "ext-xwayland-shell")]
        if let Some(interface) = xwayland_shell::ObjectInteface::from_name(name, version) {
            return Some(Self::XwaylandShell(interface));
        }
        #[cfg(feature = "ext-fullscreen-shell")]
        if let Some(interface) = fullscreen_shell::ObjectInteface::from_name(name, version) {
            return Some(Self::FullscreenShell(interface));
        }
        #[cfg(feature = "ext-idle-inhibit")]
        if let Some(interface) = idle_inhibit::ObjectInteface::from_name(name, version) {
            return Some(Self::IdleInhibit(interface));
        }
        #[cfg(feature = "ext-input-method")]
        if let Some(interface) = input_method::ObjectInteface::from_name(name, version) {
            return Some(Self::InputMethod(interface));
        }
        #[cfg(feature = "ext-input-timestamps")]
        if let Some(interface) = input_timestamps::ObjectInteface::from_name(name, version) {
            return Some(Self::InputTimestamps(interface));
        }
        #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
        if let Some(interface) =
            keyboard_shortcuts_inhibit::ObjectInteface::from_name(name, version)
        {
            return Some(Self::KeyboardShortcutsInhibit(interface));
        }
        #[cfg(feature = "ext-linux-explicit-synchronization")]
        if let Some(interface) =
            linux_explicit_synchronization::ObjectInteface::from_name(name, version)
        {
            return Some(Self::LinuxExplicitSynchronization(interface));
        }
        #[cfg(feature = "ext-pointer-constraints")]
        if let Some(interface) = pointer_constraints::ObjectInteface::from_name(name, version) {
            return Some(Self::PointerConstraints(interface));
        }
        #[cfg(feature = "ext-pointer-gestures")]
        if let Some(interface) = pointer_gestures::ObjectInteface::from_name(name, version) {
            return Some(Self::PointerGestures(interface));
        }
        #[cfg(feature = "ext-primary-selection")]
        if let Some(interface) = primary_selection::ObjectInteface::from_name(name, version) {
            return Some(Self::PrimarySelection(interface));
        }
        #[cfg(feature = "ext-relative-pointer")]
        if let Some(interface) = relative_pointer::ObjectInteface::from_name(name, version) {
            return Some(Self::RelativePointer(interface));
        }
        #[cfg(feature = "ext-text-input")]
        if let Some(interface) = text_input::ObjectInteface::from_name(name, version) {
            return Some(Self::TextInput(interface));
        }
        #[cfg(feature = "ext-xdg-decoration")]
        if let Some(interface) = xdg_decoration::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgDecoration(interface));
        }
        #[cfg(feature = "ext-xdg-foreign")]
        if let Some(interface) = xdg_foreign::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgForeign(interface));
        }
        #[cfg(feature = "ext-xdg-output")]
        if let Some(interface) = xdg_output::ObjectInteface::from_name(name, version) {
            return Some(Self::XdgOutput(interface));
        }
        #[cfg(feature = "ext-xwayland-keyboard-grab")]
        if let Some(interface) = xwayland_keyboard_grab::ObjectInteface::from_name(name, version) {
            return Some(Self::XwaylandKeyboardGrab(interface));
        }
        #[cfg(feature = "wlr-export-dmabuf")]
        if let Some(interface) = export_dmabuf::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrExportDmabuf(interface));
        }
        #[cfg(feature = "wlr-foreign-toplevel-management")]
        if let Some(interface) =
            foreign_toplevel_management::ObjectInteface::from_name(name, version)
        {
            return Some(Self::WlrForeignToplevelManagement(interface));
        }
        #[cfg(feature = "wlr-gamma-control")]
        if let Some(interface) = gamma_control::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrGammaControl(interface));
        }
        #[cfg(feature = "wlr-input-inhibitor")]
        if let Some(interface) = input_inhibitor::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrInputInhibitor(interface));
        }
        #[cfg(feature = "wlr-output-management")]
        if let Some(interface) = output_management::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrOutputManagement(interface));
        }
        #[cfg(feature = "wlr-output-power-management")]
        if let Some(interface) = output_power_management::ObjectInteface::from_name(name, version) {
            return Some(Self::WlrOutputPowerManagement(interface));
        }
        None
    }
}
