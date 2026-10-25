use super::{ZwlrLayerSurfaceV1ClosedEvent, ZwlrLayerSurfaceV1ConfigureEvent};
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR layer shell events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrLayerSurfaceV1ConfigureEvent].
    ZwlrLayerSurfaceV1Configure(ZwlrLayerSurfaceV1ConfigureEvent),
    /// Message [ZwlrLayerSurfaceV1ClosedEvent].
    ZwlrLayerSurfaceV1Closed(ZwlrLayerSurfaceV1ClosedEvent),
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
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerSurfaceV1 {
                ..
            }) => match opcode {
                ZwlrLayerSurfaceV1ConfigureEvent::OPCODE => Ok(Self::ZwlrLayerSurfaceV1Configure(
                    ZwlrLayerSurfaceV1ConfigureEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrLayerSurfaceV1ClosedEvent::OPCODE => Ok(Self::ZwlrLayerSurfaceV1Closed(
                    ZwlrLayerSurfaceV1ClosedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwlrLayerSurfaceV1Configure(message) => message.object_id(),
            Self::ZwlrLayerSurfaceV1Closed(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrLayerSurfaceV1Configure(message) => message.opcode(),
            Self::ZwlrLayerSurfaceV1Closed(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrLayerSurfaceV1Configure(message) => message.payload_size(),
            Self::ZwlrLayerSurfaceV1Closed(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrLayerSurfaceV1Configure(message) => message.write(writer, state).await,
            Self::ZwlrLayerSurfaceV1Closed(message) => message.write(writer, state).await,
        }
    }
}
