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
#[cfg(feature = "ext-idle-notify")]
use crate::protocol::ext::idle_notify;
#[cfg(feature = "ext-image-capture-source")]
use crate::protocol::ext::image_capture_source;
#[cfg(feature = "ext-image-copy-capture")]
use crate::protocol::ext::image_copy_capture;
#[cfg(feature = "ext-linux-drm-syncobj")]
use crate::protocol::ext::linux_drm_syncobj;
#[cfg(feature = "ext-pointer-warp")]
use crate::protocol::ext::pointer_warp;
#[cfg(feature = "ext-presentation-time")]
use crate::protocol::ext::presentation_time;
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
#[cfg(feature = "ext-transient-seat")]
use crate::protocol::ext::transient_seat;
#[cfg(feature = "ext-viewporter")]
use crate::protocol::ext::viewporter;
#[cfg(feature = "ext-workspace")]
use crate::protocol::ext::workspace;
#[cfg(feature = "ext-xdg-activation")]
use crate::protocol::ext::xdg_activation;
#[cfg(feature = "ext-xdg-dialog")]
use crate::protocol::ext::xdg_dialog;
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
#[cfg(feature = "ext-xwayland-shell")]
use crate::protocol::ext::xwayland_shell;
#[cfg(feature = "wlr-data-control")]
use crate::protocol::wlr::data_control;
#[cfg(feature = "wlr-layer-shell")]
use crate::protocol::wlr::layer_shell;
#[cfg(feature = "wlr-screencopy")]
use crate::protocol::wlr::screencopy;
#[cfg(feature = "wlr-virtual-pointer")]
use crate::protocol::wlr::virtual_pointer;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Groups enabled protocol requests by their defining protocol.
#[derive(Debug)]
pub enum CommandRequest {
    /// Core Wayland protocol commands.
    Core(core::CommandRequest),
    /// Xwayland shell extension commands.
    #[cfg(feature = "ext-xwayland-shell")]
    XwaylandShell(xwayland_shell::CommandRequest),
    /// Xdg toplevel tag extension commands.
    #[cfg(feature = "ext-xdg-toplevel-tag")]
    XdgToplevelTag(xdg_toplevel_tag::CommandRequest),
    /// Xdg toplevel icon extension commands.
    #[cfg(feature = "ext-xdg-toplevel-icon")]
    XdgToplevelIcon(xdg_toplevel_icon::CommandRequest),
    /// Xdg toplevel drag extension commands.
    #[cfg(feature = "ext-xdg-toplevel-drag")]
    XdgToplevelDrag(xdg_toplevel_drag::CommandRequest),
    /// Xdg system bell extension commands.
    #[cfg(feature = "ext-xdg-system-bell")]
    XdgSystemBell(xdg_system_bell::CommandRequest),
    /// Xdg session management extension commands.
    #[cfg(feature = "ext-xdg-session-management")]
    XdgSessionManagement(xdg_session_management::CommandRequest),
    /// Xdg dialog extension commands.
    #[cfg(feature = "ext-xdg-dialog")]
    XdgDialog(xdg_dialog::CommandRequest),
    /// Xdg activation extension commands.
    #[cfg(feature = "ext-xdg-activation")]
    XdgActivation(xdg_activation::CommandRequest),
    /// Tearing control extension commands.
    #[cfg(feature = "ext-tearing-control")]
    TearingControl(tearing_control::CommandRequest),
    /// Single pixel buffer extension commands.
    #[cfg(feature = "ext-single-pixel-buffer")]
    SinglePixelBuffer(single_pixel_buffer::CommandRequest),
    /// Security context extension commands.
    #[cfg(feature = "ext-security-context")]
    SecurityContext(security_context::CommandRequest),
    /// Pointer warp extension commands.
    #[cfg(feature = "ext-pointer-warp")]
    PointerWarp(pointer_warp::CommandRequest),
    /// Linux drm syncobj extension commands.
    #[cfg(feature = "ext-linux-drm-syncobj")]
    LinuxDrmSyncobj(linux_drm_syncobj::CommandRequest),
    /// Fractional scale extension commands.
    #[cfg(feature = "ext-fractional-scale")]
    FractionalScale(fractional_scale::CommandRequest),
    /// Fifo extension commands.
    #[cfg(feature = "ext-fifo")]
    Fifo(fifo::CommandRequest),
    /// Workspace extension commands.
    #[cfg(feature = "ext-workspace")]
    Workspace(workspace::CommandRequest),
    /// Transient seat extension commands.
    #[cfg(feature = "ext-transient-seat")]
    TransientSeat(transient_seat::CommandRequest),
    /// Session lock extension commands.
    #[cfg(feature = "ext-session-lock")]
    SessionLock(session_lock::CommandRequest),
    /// Image copy capture extension commands.
    #[cfg(feature = "ext-image-copy-capture")]
    ImageCopyCapture(image_copy_capture::CommandRequest),
    /// Image capture source extension commands.
    #[cfg(feature = "ext-image-capture-source")]
    ImageCaptureSource(image_capture_source::CommandRequest),
    /// Idle notify extension commands.
    #[cfg(feature = "ext-idle-notify")]
    IdleNotify(idle_notify::CommandRequest),
    /// Foreign toplevel list extension commands.
    #[cfg(feature = "ext-foreign-toplevel-list")]
    ForeignToplevelList(foreign_toplevel_list::CommandRequest),
    /// Color representation extension commands.
    #[cfg(feature = "ext-color-representation")]
    ColorRepresentation(color_representation::CommandRequest),
    /// Color management extension commands.
    #[cfg(feature = "ext-color-management")]
    ColorManagement(color_management::CommandRequest),
    /// Data control extension commands.
    #[cfg(feature = "ext-data-control")]
    DataControl(ext_data_control::CommandRequest),
    /// Background effect extension commands.
    #[cfg(feature = "ext-background-effect")]
    BackgroundEffect(background_effect::CommandRequest),
    /// DRM lease extension commands.
    #[cfg(feature = "ext-drm-lease")]
    DrmLease(drm_lease::CommandRequest),
    /// Cursor shape extension commands.
    #[cfg(feature = "ext-cursor-shape")]
    CursorShape(cursor_shape::CommandRequest),
    /// Content type extension commands.
    #[cfg(feature = "ext-content-type")]
    ContentType(content_type::CommandRequest),
    /// Commit timing extension commands.
    #[cfg(feature = "ext-commit-timing")]
    CommitTiming(commit_timing::CommandRequest),
    /// Alpha modifier extension commands.
    #[cfg(feature = "ext-alpha-modifier")]
    AlphaModifier(alpha_modifier::CommandRequest),
    /// WLR screencopy extension commands.
    #[cfg(feature = "wlr-screencopy")]
    WlrScreencopy(screencopy::CommandRequest),
    /// WLR virtual pointer extension commands.
    #[cfg(feature = "wlr-virtual-pointer")]
    WlrVirtualPointer(virtual_pointer::CommandRequest),
    /// WLR data control extension commands.
    #[cfg(feature = "wlr-data-control")]
    WlrDataControl(data_control::CommandRequest),
    /// WLR layer shell extension commands.
    #[cfg(feature = "wlr-layer-shell")]
    WlrLayerShell(layer_shell::CommandRequest),
    /// Tablet extension commands.
    #[cfg(feature = "ext-tablet")]
    Tablet(tablet::CommandRequest),
    /// XDG shell extension commands.
    #[cfg(feature = "ext-xdg-shell")]
    XdgShell(xdg_shell::CommandRequest),
    /// Linux DMA-BUF extension commands.
    #[cfg(feature = "ext-dmabuf")]
    Dmabuf(dmabuf::CommandRequest),
    /// Presentation time extension commands.
    #[cfg(feature = "ext-presentation-time")]
    PresentationTime(presentation_time::CommandRequest),
    /// Viewporter extension commands.
    #[cfg(feature = "ext-viewporter")]
    Viewporter(viewporter::CommandRequest),
}

impl CommandReader<Self> for CommandRequest {
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

impl InterfaceReader for CommandRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        match interface {
            #[cfg(feature = "ext-dmabuf")]
            ObjectInteface::Dmabuf(_) => Ok(Self::Dmabuf(
                dmabuf::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-shell")]
            ObjectInteface::XdgShell(_) => Ok(Self::XdgShell(
                xdg_shell::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-tablet")]
            ObjectInteface::Tablet(_) => Ok(Self::Tablet(
                tablet::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-layer-shell")]
            ObjectInteface::WlrLayerShell(_) => Ok(Self::WlrLayerShell(
                layer_shell::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-data-control")]
            ObjectInteface::WlrDataControl(_) => Ok(Self::WlrDataControl(
                data_control::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-virtual-pointer")]
            ObjectInteface::WlrVirtualPointer(_) => Ok(Self::WlrVirtualPointer(
                virtual_pointer::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "wlr-screencopy")]
            ObjectInteface::WlrScreencopy(_) => Ok(Self::WlrScreencopy(
                screencopy::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-alpha-modifier")]
            ObjectInteface::AlphaModifier(_) => Ok(Self::AlphaModifier(
                alpha_modifier::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-commit-timing")]
            ObjectInteface::CommitTiming(_) => Ok(Self::CommitTiming(
                commit_timing::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-content-type")]
            ObjectInteface::ContentType(_) => Ok(Self::ContentType(
                content_type::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-cursor-shape")]
            ObjectInteface::CursorShape(_) => Ok(Self::CursorShape(
                cursor_shape::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-drm-lease")]
            ObjectInteface::DrmLease(_) => Ok(Self::DrmLease(
                drm_lease::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-background-effect")]
            ObjectInteface::BackgroundEffect(_) => Ok(Self::BackgroundEffect(
                background_effect::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-data-control")]
            ObjectInteface::DataControl(_) => Ok(Self::DataControl(
                ext_data_control::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-color-management")]
            ObjectInteface::ColorManagement(_) => Ok(Self::ColorManagement(
                color_management::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-color-representation")]
            ObjectInteface::ColorRepresentation(_) => Ok(Self::ColorRepresentation(
                color_representation::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-foreign-toplevel-list")]
            ObjectInteface::ForeignToplevelList(_) => Ok(Self::ForeignToplevelList(
                foreign_toplevel_list::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-idle-notify")]
            ObjectInteface::IdleNotify(_) => Ok(Self::IdleNotify(
                idle_notify::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-image-capture-source")]
            ObjectInteface::ImageCaptureSource(_) => Ok(Self::ImageCaptureSource(
                image_capture_source::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-image-copy-capture")]
            ObjectInteface::ImageCopyCapture(_) => Ok(Self::ImageCopyCapture(
                image_copy_capture::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-session-lock")]
            ObjectInteface::SessionLock(_) => Ok(Self::SessionLock(
                session_lock::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-transient-seat")]
            ObjectInteface::TransientSeat(_) => Ok(Self::TransientSeat(
                transient_seat::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-workspace")]
            ObjectInteface::Workspace(_) => Ok(Self::Workspace(
                workspace::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-fifo")]
            ObjectInteface::Fifo(_) => Ok(Self::Fifo(
                fifo::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-fractional-scale")]
            ObjectInteface::FractionalScale(_) => Ok(Self::FractionalScale(
                fractional_scale::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            ObjectInteface::LinuxDrmSyncobj(_) => Ok(Self::LinuxDrmSyncobj(
                linux_drm_syncobj::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-pointer-warp")]
            ObjectInteface::PointerWarp(_) => Ok(Self::PointerWarp(
                pointer_warp::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-security-context")]
            ObjectInteface::SecurityContext(_) => Ok(Self::SecurityContext(
                security_context::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-single-pixel-buffer")]
            ObjectInteface::SinglePixelBuffer(_) => Ok(Self::SinglePixelBuffer(
                single_pixel_buffer::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-tearing-control")]
            ObjectInteface::TearingControl(_) => Ok(Self::TearingControl(
                tearing_control::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-activation")]
            ObjectInteface::XdgActivation(_) => Ok(Self::XdgActivation(
                xdg_activation::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-dialog")]
            ObjectInteface::XdgDialog(_) => Ok(Self::XdgDialog(
                xdg_dialog::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-session-management")]
            ObjectInteface::XdgSessionManagement(_) => Ok(Self::XdgSessionManagement(
                xdg_session_management::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-system-bell")]
            ObjectInteface::XdgSystemBell(_) => Ok(Self::XdgSystemBell(
                xdg_system_bell::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            ObjectInteface::XdgToplevelDrag(_) => Ok(Self::XdgToplevelDrag(
                xdg_toplevel_drag::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            ObjectInteface::XdgToplevelIcon(_) => Ok(Self::XdgToplevelIcon(
                xdg_toplevel_icon::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            ObjectInteface::XdgToplevelTag(_) => Ok(Self::XdgToplevelTag(
                xdg_toplevel_tag::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-xwayland-shell")]
            ObjectInteface::XwaylandShell(_) => Ok(Self::XwaylandShell(
                xwayland_shell::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            ObjectInteface::Core(_) => Ok(Self::Core(
                core::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-presentation-time")]
            ObjectInteface::PresentationTime(_) => Ok(Self::PresentationTime(
                presentation_time::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
            #[cfg(feature = "ext-viewporter")]
            ObjectInteface::Viewporter(_) => Ok(Self::Viewporter(
                viewporter::CommandRequest::read_with_interface(
                    object_id, opcode, interface, reader, state,
                )
                .await?,
            )),
        }
    }
}

impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::Core(command) => command.object_id(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(command) => command.object_id(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.object_id(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(command) => command.object_id(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(command) => command.object_id(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(command) => command.object_id(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(command) => command.object_id(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(command) => command.object_id(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.object_id(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(command) => command.object_id(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.object_id(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.object_id(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.object_id(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.object_id(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(command) => command.object_id(),
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
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(command) => command.object_id(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(command) => command.object_id(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(command) => command.object_id(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(command) => command.object_id(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.object_id(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(command) => command.object_id(),
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
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(command) => command.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::Core(command) => command.opcode(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(command) => command.opcode(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.opcode(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(command) => command.opcode(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(command) => command.opcode(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(command) => command.opcode(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(command) => command.opcode(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(command) => command.opcode(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.opcode(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(command) => command.opcode(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.opcode(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.opcode(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.opcode(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.opcode(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(command) => command.opcode(),
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
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(command) => command.opcode(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(command) => command.opcode(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(command) => command.opcode(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(command) => command.opcode(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.opcode(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(command) => command.opcode(),
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
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(command) => command.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::Core(command) => command.payload_size(),
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(command) => command.payload_size(),
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.payload_size(),
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(command) => command.payload_size(),
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(command) => command.payload_size(),
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(command) => command.payload_size(),
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(command) => command.payload_size(),
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(command) => command.payload_size(),
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.payload_size(),
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(command) => command.payload_size(),
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.payload_size(),
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.payload_size(),
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.payload_size(),
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.payload_size(),
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(command) => command.payload_size(),
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
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(command) => command.payload_size(),
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(command) => command.payload_size(),
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(command) => command.payload_size(),
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(command) => command.payload_size(),
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.payload_size(),
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(command) => command.payload_size(),
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
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(command) => command.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::Core(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xwayland-shell")]
            Self::XwaylandShell(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-toplevel-tag")]
            Self::XdgToplevelTag(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-toplevel-icon")]
            Self::XdgToplevelIcon(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-toplevel-drag")]
            Self::XdgToplevelDrag(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-system-bell")]
            Self::XdgSystemBell(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-session-management")]
            Self::XdgSessionManagement(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-dialog")]
            Self::XdgDialog(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-xdg-activation")]
            Self::XdgActivation(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-tearing-control")]
            Self::TearingControl(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-single-pixel-buffer")]
            Self::SinglePixelBuffer(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-security-context")]
            Self::SecurityContext(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-pointer-warp")]
            Self::PointerWarp(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-linux-drm-syncobj")]
            Self::LinuxDrmSyncobj(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-fractional-scale")]
            Self::FractionalScale(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-fifo")]
            Self::Fifo(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-workspace")]
            Self::Workspace(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-transient-seat")]
            Self::TransientSeat(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-session-lock")]
            Self::SessionLock(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-image-copy-capture")]
            Self::ImageCopyCapture(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-image-capture-source")]
            Self::ImageCaptureSource(command) => command.write(writer, state).await,
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
            #[cfg(feature = "ext-cursor-shape")]
            Self::CursorShape(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-content-type")]
            Self::ContentType(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-commit-timing")]
            Self::CommitTiming(command) => command.write(writer, state).await,
            #[cfg(feature = "ext-alpha-modifier")]
            Self::AlphaModifier(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-screencopy")]
            Self::WlrScreencopy(command) => command.write(writer, state).await,
            #[cfg(feature = "wlr-virtual-pointer")]
            Self::WlrVirtualPointer(command) => command.write(writer, state).await,
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
            #[cfg(feature = "ext-viewporter")]
            Self::Viewporter(command) => command.write(writer, state).await,
        }
    }
}
