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
#[cfg(feature = "ext-idle-notify")]
use crate::protocol::ext::idle_notify;
#[cfg(feature = "ext-image-copy-capture")]
use crate::protocol::ext::image_copy_capture;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
#[cfg(feature = "ext-session-lock")]
use crate::protocol::ext::session_lock;
#[cfg(feature = "ext-tablet")]
use crate::protocol::ext::tablet;
#[cfg(feature = "ext-transient-seat")]
use crate::protocol::ext::transient_seat;
#[cfg(feature = "ext-workspace")]
use crate::protocol::ext::workspace;
#[cfg(feature = "ext-xdg-activation")]
use crate::protocol::ext::xdg_activation;
#[cfg(feature = "ext-xdg-session-management")]
use crate::protocol::ext::xdg_session_management;
#[cfg(feature = "ext-xdg-shell")]
use crate::protocol::ext::xdg_shell;
#[cfg(feature = "ext-xdg-toplevel-icon")]
use crate::protocol::ext::xdg_toplevel_icon;
#[cfg(feature = "wlr-data-control")]
use crate::protocol::wlr::data_control;
#[cfg(feature = "wlr-layer-shell")]
use crate::protocol::wlr::layer_shell;
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
