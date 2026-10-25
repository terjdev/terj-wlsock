use crate::protocol::core;
#[cfg(feature = "ext-alpha-modifier")]
use crate::protocol::ext::alpha_modifier;
#[cfg(feature = "ext-commit-timing")]
use crate::protocol::ext::commit_timing;
#[cfg(feature = "ext-content-type")]
use crate::protocol::ext::content_type;
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
