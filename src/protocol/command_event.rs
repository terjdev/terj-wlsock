use crate::protocol::core;
#[cfg(feature = "ext-background-effect")]
use crate::protocol::ext::background_effect;
#[cfg(feature = "ext-color-management")]
use crate::protocol::ext::color_management;
#[cfg(feature = "ext-color-representation")]
use crate::protocol::ext::color_representation;
#[cfg(feature = "ext-data-control")]
use crate::protocol::ext::data_control as ext_data_control;
#[cfg(feature = "ext-dmabuf")]
use crate::protocol::ext::dmabuf;
#[cfg(feature = "ext-drm-lease")]
use crate::protocol::ext::drm_lease;
#[cfg(feature = "ext-foreign-toplevel-list")]
use crate::protocol::ext::foreign_toplevel_list;
#[cfg(feature = "ext-fractional-scale")]
use crate::protocol::ext::fractional_scale;
#[cfg(feature = "ext-fullscreen-shell")]
use crate::protocol::ext::fullscreen_shell;
#[cfg(feature = "ext-idle-notify")]
use crate::protocol::ext::idle_notify;
#[cfg(feature = "ext-image-copy-capture")]
use crate::protocol::ext::image_copy_capture;
#[cfg(feature = "ext-input-method")]
use crate::protocol::ext::input_method;
#[cfg(feature = "ext-input-timestamps")]
use crate::protocol::ext::input_timestamps;
#[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
use crate::protocol::ext::keyboard_shortcuts_inhibit;
#[cfg(feature = "ext-linux-explicit-synchronization")]
use crate::protocol::ext::linux_explicit_synchronization;
#[cfg(feature = "ext-pointer-constraints")]
use crate::protocol::ext::pointer_constraints;
#[cfg(feature = "ext-pointer-gestures")]
use crate::protocol::ext::pointer_gestures;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
#[cfg(feature = "ext-primary-selection")]
use crate::protocol::ext::primary_selection;
#[cfg(feature = "ext-relative-pointer")]
use crate::protocol::ext::relative_pointer;
#[cfg(feature = "ext-session-lock")]
use crate::protocol::ext::session_lock;
#[cfg(feature = "ext-tablet")]
use crate::protocol::ext::tablet;
#[cfg(feature = "ext-tablet-unstable")]
use crate::protocol::ext::tablet_unstable;
#[cfg(feature = "ext-text-input")]
use crate::protocol::ext::text_input;
#[cfg(feature = "ext-transient-seat")]
use crate::protocol::ext::transient_seat;
#[cfg(feature = "ext-workspace")]
use crate::protocol::ext::workspace;
#[cfg(feature = "ext-xdg-activation")]
use crate::protocol::ext::xdg_activation;
#[cfg(feature = "ext-xdg-decoration")]
use crate::protocol::ext::xdg_decoration;
#[cfg(feature = "ext-xdg-foreign")]
use crate::protocol::ext::xdg_foreign;
#[cfg(feature = "ext-xdg-output")]
use crate::protocol::ext::xdg_output;
#[cfg(feature = "ext-xdg-session-management")]
use crate::protocol::ext::xdg_session_management;
#[cfg(feature = "ext-xdg-shell")]
use crate::protocol::ext::xdg_shell;
#[cfg(feature = "ext-xdg-toplevel-icon")]
use crate::protocol::ext::xdg_toplevel_icon;
#[cfg(feature = "wlr-data-control")]
use crate::protocol::wlr::data_control;
#[cfg(feature = "wlr-export-dmabuf")]
use crate::protocol::wlr::export_dmabuf;
#[cfg(feature = "wlr-foreign-toplevel-management")]
use crate::protocol::wlr::foreign_toplevel_management;
#[cfg(feature = "wlr-gamma-control")]
use crate::protocol::wlr::gamma_control;
#[cfg(feature = "wlr-layer-shell")]
use crate::protocol::wlr::layer_shell;
#[cfg(feature = "wlr-output-management")]
use crate::protocol::wlr::output_management;
#[cfg(feature = "wlr-output-power-management")]
use crate::protocol::wlr::output_power_management;
#[cfg(feature = "wlr-screencopy")]
use crate::protocol::wlr::screencopy;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Groups enabled protocol events by their defining protocol.
#[derive(Debug)]
pub enum CommandEvent {
    /// Core Wayland protocol commands.
    Core(core::CommandEvent),
    /// output power management messages.
    #[cfg(feature = "wlr-output-power-management")]
    WlrOutputPowerManagement(output_power_management::CommandEvent),
    /// output management messages.
    #[cfg(feature = "wlr-output-management")]
    WlrOutputManagement(output_management::CommandEvent),
    /// gamma control messages.
    #[cfg(feature = "wlr-gamma-control")]
    WlrGammaControl(gamma_control::CommandEvent),
    /// foreign toplevel management messages.
    #[cfg(feature = "wlr-foreign-toplevel-management")]
    WlrForeignToplevelManagement(foreign_toplevel_management::CommandEvent),
    /// export dmabuf messages.
    #[cfg(feature = "wlr-export-dmabuf")]
    WlrExportDmabuf(export_dmabuf::CommandEvent),
    /// xdg output messages.
    #[cfg(feature = "ext-xdg-output")]
    XdgOutput(xdg_output::CommandEvent),
    /// xdg foreign messages.
    #[cfg(feature = "ext-xdg-foreign")]
    XdgForeign(xdg_foreign::CommandEvent),
    /// xdg decoration messages.
    #[cfg(feature = "ext-xdg-decoration")]
    XdgDecoration(xdg_decoration::CommandEvent),
    /// text input messages.
    #[cfg(feature = "ext-text-input")]
    TextInput(text_input::CommandEvent),
    /// tablet unstable messages.
    #[cfg(feature = "ext-tablet-unstable")]
    TabletUnstable(tablet_unstable::CommandEvent),
    /// relative pointer messages.
    #[cfg(feature = "ext-relative-pointer")]
    RelativePointer(relative_pointer::CommandEvent),
    /// primary selection messages.
    #[cfg(feature = "ext-primary-selection")]
    PrimarySelection(primary_selection::CommandEvent),
    /// pointer gestures messages.
    #[cfg(feature = "ext-pointer-gestures")]
    PointerGestures(pointer_gestures::CommandEvent),
    /// pointer constraints messages.
    #[cfg(feature = "ext-pointer-constraints")]
    PointerConstraints(pointer_constraints::CommandEvent),
    /// linux explicit synchronization messages.
    #[cfg(feature = "ext-linux-explicit-synchronization")]
    LinuxExplicitSynchronization(linux_explicit_synchronization::CommandEvent),
    /// keyboard shortcuts inhibit messages.
    #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
    KeyboardShortcutsInhibit(keyboard_shortcuts_inhibit::CommandEvent),
    /// input timestamps messages.
    #[cfg(feature = "ext-input-timestamps")]
    InputTimestamps(input_timestamps::CommandEvent),
    /// input method messages.
    #[cfg(feature = "ext-input-method")]
    InputMethod(input_method::CommandEvent),
    /// fullscreen shell messages.
    #[cfg(feature = "ext-fullscreen-shell")]
    FullscreenShell(fullscreen_shell::CommandEvent),
    /// Xdg toplevel icon extension commands.
    #[cfg(feature = "ext-xdg-toplevel-icon")]
    XdgToplevelIcon(xdg_toplevel_icon::CommandEvent),
    /// Xdg session management extension commands.
    #[cfg(feature = "ext-xdg-session-management")]
    XdgSessionManagement(xdg_session_management::CommandEvent),
    /// Xdg activation extension commands.
    #[cfg(feature = "ext-xdg-activation")]
    XdgActivation(xdg_activation::CommandEvent),
    /// Fractional scale extension commands.
    #[cfg(feature = "ext-fractional-scale")]
    FractionalScale(fractional_scale::CommandEvent),
    /// Workspace extension commands.
    #[cfg(feature = "ext-workspace")]
    Workspace(workspace::CommandEvent),
    /// Transient seat extension commands.
    #[cfg(feature = "ext-transient-seat")]
    TransientSeat(transient_seat::CommandEvent),
    /// Session lock extension commands.
    #[cfg(feature = "ext-session-lock")]
    SessionLock(session_lock::CommandEvent),
    /// Image copy capture extension commands.
    #[cfg(feature = "ext-image-copy-capture")]
    ImageCopyCapture(image_copy_capture::CommandEvent),
    /// Idle notify extension commands.
    #[cfg(feature = "ext-idle-notify")]
    IdleNotify(idle_notify::CommandEvent),
    /// Foreign toplevel list extension commands.
    #[cfg(feature = "ext-foreign-toplevel-list")]
    ForeignToplevelList(foreign_toplevel_list::CommandEvent),
    /// Color representation extension commands.
    #[cfg(feature = "ext-color-representation")]
    ColorRepresentation(color_representation::CommandEvent),
    /// Color management extension commands.
    #[cfg(feature = "ext-color-management")]
    ColorManagement(color_management::CommandEvent),
    /// Data control extension commands.
    #[cfg(feature = "ext-data-control")]
    DataControl(ext_data_control::CommandEvent),
    /// Background effect extension commands.
    #[cfg(feature = "ext-background-effect")]
    BackgroundEffect(background_effect::CommandEvent),
    /// DRM lease extension commands.
    #[cfg(feature = "ext-drm-lease")]
    DrmLease(drm_lease::CommandEvent),
    /// WLR screencopy extension commands.
    #[cfg(feature = "wlr-screencopy")]
    WlrScreencopy(screencopy::CommandEvent),
    /// WLR data control extension commands.
    #[cfg(feature = "wlr-data-control")]
    WlrDataControl(data_control::CommandEvent),
    /// WLR layer shell extension commands.
    #[cfg(feature = "wlr-layer-shell")]
    WlrLayerShell(layer_shell::CommandEvent),
    /// Tablet extension commands.
    #[cfg(feature = "ext-tablet")]
    Tablet(tablet::CommandEvent),
    /// XDG shell extension commands.
    #[cfg(feature = "ext-xdg-shell")]
    XdgShell(xdg_shell::CommandEvent),
    /// Linux DMA-BUF extension commands.
    #[cfg(feature = "ext-dmabuf")]
    Dmabuf(dmabuf::CommandEvent),
    /// Presentation time extension commands.
    #[cfg(feature = "ext-presentation-time")]
    PresentationTime(presentation_time::CommandEvent),
}

impl CommandReader<Self> for CommandEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for CommandEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface {
            #[cfg(feature = "wlr-input-inhibitor")]
            ObjectInteface::WlrInputInhibitor(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xwayland-keyboard-grab")]
            ObjectInteface::XwaylandKeyboardGrab(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-idle-inhibit")]
            ObjectInteface::IdleInhibit(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xwayland-shell")]
            ObjectInteface::XwaylandShell(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            ObjectInteface::XdgToplevelTag(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            ObjectInteface::XdgToplevelDrag(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xdg-system-bell")]
            ObjectInteface::XdgSystemBell(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-xdg-dialog")]
            ObjectInteface::XdgDialog(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-tearing-control")]
            ObjectInteface::TearingControl(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-single-pixel-buffer")]
            ObjectInteface::SinglePixelBuffer(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-security-context")]
            ObjectInteface::SecurityContext(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-pointer-warp")]
            ObjectInteface::PointerWarp(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            ObjectInteface::LinuxDrmSyncobj(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-fifo")]
            ObjectInteface::Fifo(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-image-capture-source")]
            ObjectInteface::ImageCaptureSource(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-cursor-shape")]
            ObjectInteface::CursorShape(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-content-type")]
            ObjectInteface::ContentType(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-commit-timing")]
            ObjectInteface::CommitTiming(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-alpha-modifier")]
            ObjectInteface::AlphaModifier(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "wlr-virtual-pointer")]
            ObjectInteface::WlrVirtualPointer(_) => Err(FdReaderError::InvalidProtocol),
            #[cfg(feature = "ext-dmabuf")]
            ObjectInteface::Dmabuf(_) => Ok(Self::Dmabuf(
                dmabuf::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-shell")]
            ObjectInteface::XdgShell(_) => Ok(Self::XdgShell(
                xdg_shell::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-tablet")]
            ObjectInteface::Tablet(_) => Ok(Self::Tablet(
                tablet::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-layer-shell")]
            ObjectInteface::WlrLayerShell(_) => Ok(Self::WlrLayerShell(
                layer_shell::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-data-control")]
            ObjectInteface::WlrDataControl(_) => Ok(Self::WlrDataControl(
                data_control::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-screencopy")]
            ObjectInteface::WlrScreencopy(_) => Ok(Self::WlrScreencopy(
                screencopy::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-drm-lease")]
            ObjectInteface::DrmLease(_) => Ok(Self::DrmLease(
                drm_lease::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-background-effect")]
            ObjectInteface::BackgroundEffect(_) => Ok(Self::BackgroundEffect(
                background_effect::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-data-control")]
            ObjectInteface::DataControl(_) => Ok(Self::DataControl(
                ext_data_control::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-color-management")]
            ObjectInteface::ColorManagement(_) => Ok(Self::ColorManagement(
                color_management::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-color-representation")]
            ObjectInteface::ColorRepresentation(_) => Ok(Self::ColorRepresentation(
                color_representation::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            ObjectInteface::ForeignToplevelList(_) => Ok(Self::ForeignToplevelList(
                foreign_toplevel_list::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-idle-notify")]
            ObjectInteface::IdleNotify(_) => Ok(Self::IdleNotify(
                idle_notify::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-image-copy-capture")]
            ObjectInteface::ImageCopyCapture(_) => Ok(Self::ImageCopyCapture(
                image_copy_capture::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-session-lock")]
            ObjectInteface::SessionLock(_) => Ok(Self::SessionLock(
                session_lock::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-transient-seat")]
            ObjectInteface::TransientSeat(_) => Ok(Self::TransientSeat(
                transient_seat::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-workspace")]
            ObjectInteface::Workspace(_) => Ok(Self::Workspace(
                workspace::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-fractional-scale")]
            ObjectInteface::FractionalScale(_) => Ok(Self::FractionalScale(
                fractional_scale::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-activation")]
            ObjectInteface::XdgActivation(_) => Ok(Self::XdgActivation(
                xdg_activation::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-session-management")]
            ObjectInteface::XdgSessionManagement(_) => Ok(Self::XdgSessionManagement(
                xdg_session_management::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            ObjectInteface::XdgToplevelIcon(_) => Ok(Self::XdgToplevelIcon(
                xdg_toplevel_icon::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-fullscreen-shell")]
            ObjectInteface::FullscreenShell(_) => Ok(Self::FullscreenShell(
                fullscreen_shell::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-input-method")]
            ObjectInteface::InputMethod(_) => Ok(Self::InputMethod(
                input_method::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-input-timestamps")]
            ObjectInteface::InputTimestamps(_) => Ok(Self::InputTimestamps(
                input_timestamps::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            ObjectInteface::KeyboardShortcutsInhibit(_) => Ok(Self::KeyboardShortcutsInhibit(
                keyboard_shortcuts_inhibit::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            ObjectInteface::LinuxExplicitSynchronization(_) => {
                Ok(Self::LinuxExplicitSynchronization(
                    linux_explicit_synchronization::CommandEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                ))
            }
            #[cfg(feature = "ext-pointer-constraints")]
            ObjectInteface::PointerConstraints(_) => Ok(Self::PointerConstraints(
                pointer_constraints::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-pointer-gestures")]
            ObjectInteface::PointerGestures(_) => Ok(Self::PointerGestures(
                pointer_gestures::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-primary-selection")]
            ObjectInteface::PrimarySelection(_) => Ok(Self::PrimarySelection(
                primary_selection::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-relative-pointer")]
            ObjectInteface::RelativePointer(_) => Ok(Self::RelativePointer(
                relative_pointer::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-tablet-unstable")]
            ObjectInteface::TabletUnstable(_) => Ok(Self::TabletUnstable(
                tablet_unstable::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-text-input")]
            ObjectInteface::TextInput(_) => Ok(Self::TextInput(
                text_input::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-decoration")]
            ObjectInteface::XdgDecoration(_) => Ok(Self::XdgDecoration(
                xdg_decoration::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-foreign")]
            ObjectInteface::XdgForeign(_) => Ok(Self::XdgForeign(
                xdg_foreign::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-output")]
            ObjectInteface::XdgOutput(_) => Ok(Self::XdgOutput(
                xdg_output::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-export-dmabuf")]
            ObjectInteface::WlrExportDmabuf(_) => Ok(Self::WlrExportDmabuf(
                export_dmabuf::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            ObjectInteface::WlrForeignToplevelManagement(_) => {
                Ok(Self::WlrForeignToplevelManagement(
                    foreign_toplevel_management::CommandEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                ))
            }
            #[cfg(feature = "wlr-gamma-control")]
            ObjectInteface::WlrGammaControl(_) => Ok(Self::WlrGammaControl(
                gamma_control::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-output-management")]
            ObjectInteface::WlrOutputManagement(_) => Ok(Self::WlrOutputManagement(
                output_management::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-output-power-management")]
            ObjectInteface::WlrOutputPowerManagement(_) => Ok(Self::WlrOutputPowerManagement(
                output_power_management::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            ObjectInteface::Core(_) => Ok(Self::Core(
                core::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-presentation-time")]
            ObjectInteface::PresentationTime(_) => Ok(Self::PresentationTime(
                presentation_time::CommandEvent::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-viewporter")]
            ObjectInteface::Viewporter(_) => Err(FdReaderError::InvalidProtocol),
        }
    }
}

impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::Core(command) => command.object_id(),
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(command) => command.object_id(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(command) => command.object_id(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(command) => command.object_id(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(command) => command.object_id(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(command) => command.object_id(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(command) => command.object_id(),
            #[cfg(feature = "ext-tablet-unstable")]
            Self::TabletUnstable(command) => command.object_id(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(command) => command.object_id(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(command) => command.object_id(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(command) => command.object_id(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(command) => command.object_id(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(command) => command.object_id(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(command) => command.object_id(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(command) => command.object_id(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(command) => command.object_id(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.object_id(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.object_id(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.object_id(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.object_id(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.object_id(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.object_id(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(command) => command.object_id(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(command) => command.object_id(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(command) => command.object_id(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(command) => command.object_id(),
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(command) => command.object_id(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(command) => command.object_id(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(command) => command.object_id(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.object_id(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(command) => command.object_id(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(command) => command.object_id(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(command) => command.object_id(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(command) => command.object_id(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::Core(command) => command.opcode(),
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(command) => command.opcode(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(command) => command.opcode(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(command) => command.opcode(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(command) => command.opcode(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(command) => command.opcode(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(command) => command.opcode(),
            #[cfg(feature = "ext-tablet-unstable")]
            Self::TabletUnstable(command) => command.opcode(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(command) => command.opcode(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(command) => command.opcode(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(command) => command.opcode(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(command) => command.opcode(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(command) => command.opcode(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(command) => command.opcode(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(command) => command.opcode(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(command) => command.opcode(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.opcode(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.opcode(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.opcode(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.opcode(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.opcode(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.opcode(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(command) => command.opcode(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(command) => command.opcode(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(command) => command.opcode(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(command) => command.opcode(),
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(command) => command.opcode(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(command) => command.opcode(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(command) => command.opcode(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.opcode(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(command) => command.opcode(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(command) => command.opcode(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(command) => command.opcode(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(command) => command.opcode(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::Core(command) => command.payload_size(),
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(command) => command.payload_size(),
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(command) => command.payload_size(),
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(command) => command.payload_size(),
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(command) => command.payload_size(),
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(command) => command.payload_size(),
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(command) => command.payload_size(),
            #[cfg(feature = "ext-tablet-unstable")]
            Self::TabletUnstable(command) => command.payload_size(),
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(command) => command.payload_size(),
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(command) => command.payload_size(),
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(command) => command.payload_size(),
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(command) => command.payload_size(),
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(command) => command.payload_size(),
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(command) => command.payload_size(),
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(command) => command.payload_size(),
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(command) => command.payload_size(),
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.payload_size(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.payload_size(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.payload_size(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.payload_size(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.payload_size(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.payload_size(),
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(command) => command.payload_size(),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(command) => command.payload_size(),
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(command) => command.payload_size(),
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(command) => command.payload_size(),
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(command) => command.payload_size(),
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(command) => command.payload_size(),
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(command) => command.payload_size(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.payload_size(),
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(command) => command.payload_size(),
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(command) => command.payload_size(),
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(command) => command.payload_size(),
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(command) => command.payload_size(),
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::Core(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-output-power-management")]
            Self::WlrOutputPowerManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-output-management")]
            Self::WlrOutputManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-gamma-control")]
            Self::WlrGammaControl(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-foreign-toplevel-management")]
            Self::WlrForeignToplevelManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-export-dmabuf")]
            Self::WlrExportDmabuf(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-output")]
            Self::XdgOutput(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-foreign")]
            Self::XdgForeign(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-decoration")]
            Self::XdgDecoration(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-text-input")]
            Self::TextInput(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-tablet-unstable")]
            Self::TabletUnstable(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-relative-pointer")]
            Self::RelativePointer(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-primary-selection")]
            Self::PrimarySelection(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-pointer-gestures")]
            Self::PointerGestures(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-pointer-constraints")]
            Self::PointerConstraints(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-linux-explicit-synchronization")]
            Self::LinuxExplicitSynchronization(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-keyboard-shortcuts-inhibit")]
            Self::KeyboardShortcutsInhibit(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-input-timestamps")]
            Self::InputTimestamps(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-input-method")]
            Self::InputMethod(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-fullscreen-shell")]
            Self::FullscreenShell(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-idle-notify")]
            Self::IdleNotify(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-foreign-toplevel-list")]
            Self::ForeignToplevelList(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-color-representation")]
            Self::ColorRepresentation(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-color-management")]
            Self::ColorManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-data-control")]
            Self::DataControl(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-background-effect")]
            Self::BackgroundEffect(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-drm-lease")]
            Self::DrmLease(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-data-control")]
            Self::WlrDataControl(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-layer-shell")]
            Self::WlrLayerShell(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-tablet")]
            Self::Tablet(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-shell")]
            Self::XdgShell(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-dmabuf")]
            Self::Dmabuf(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-presentation-time")]
            Self::PresentationTime(command) => command.write(writer, state).await,
        }
    }
}
