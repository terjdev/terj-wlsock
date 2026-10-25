use crate::protocol::core;
#[cfg(feature = "ext-background-effect")]
use crate::protocol::ext::background_effect;
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
#[cfg(feature = "ext-xdg-shell")]
use crate::protocol::ext::xdg_shell;
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
