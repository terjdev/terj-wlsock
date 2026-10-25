use super::ObjectInteface as XdgOutputObjectInteface;
use super::{
    ZxdgOutputV1DescriptionEvent, ZxdgOutputV1DoneEvent, ZxdgOutputV1LogicalPositionEvent,
    ZxdgOutputV1LogicalSizeEvent, ZxdgOutputV1NameEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xdg output events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZxdgOutputV1LogicalPositionEvent].
    ZxdgOutputV1LogicalPosition(ZxdgOutputV1LogicalPositionEvent),
    /// Message [ZxdgOutputV1LogicalSizeEvent].
    ZxdgOutputV1LogicalSize(ZxdgOutputV1LogicalSizeEvent),
    /// Message [ZxdgOutputV1DoneEvent].
    ZxdgOutputV1Done(ZxdgOutputV1DoneEvent),
    /// Message [ZxdgOutputV1NameEvent].
    ZxdgOutputV1Name(ZxdgOutputV1NameEvent),
    /// Message [ZxdgOutputV1DescriptionEvent].
    ZxdgOutputV1Description(ZxdgOutputV1DescriptionEvent),
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
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. }) => match opcode
            {
                ZxdgOutputV1LogicalPositionEvent::OPCODE => Ok(Self::ZxdgOutputV1LogicalPosition(
                    ZxdgOutputV1LogicalPositionEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZxdgOutputV1LogicalSizeEvent::OPCODE => Ok(Self::ZxdgOutputV1LogicalSize(
                    ZxdgOutputV1LogicalSizeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZxdgOutputV1DoneEvent::OPCODE => Ok(Self::ZxdgOutputV1Done(
                    ZxdgOutputV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZxdgOutputV1NameEvent::OPCODE => Ok(Self::ZxdgOutputV1Name(
                    ZxdgOutputV1NameEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZxdgOutputV1DescriptionEvent::OPCODE => Ok(Self::ZxdgOutputV1Description(
                    ZxdgOutputV1DescriptionEvent::read_with_interface(
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
            Self::ZxdgOutputV1LogicalPosition(m) => m.object_id(),
            Self::ZxdgOutputV1LogicalSize(m) => m.object_id(),
            Self::ZxdgOutputV1Done(m) => m.object_id(),
            Self::ZxdgOutputV1Name(m) => m.object_id(),
            Self::ZxdgOutputV1Description(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZxdgOutputV1LogicalPosition(m) => m.opcode(),
            Self::ZxdgOutputV1LogicalSize(m) => m.opcode(),
            Self::ZxdgOutputV1Done(m) => m.opcode(),
            Self::ZxdgOutputV1Name(m) => m.opcode(),
            Self::ZxdgOutputV1Description(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZxdgOutputV1LogicalPosition(m) => m.payload_size(),
            Self::ZxdgOutputV1LogicalSize(m) => m.payload_size(),
            Self::ZxdgOutputV1Done(m) => m.payload_size(),
            Self::ZxdgOutputV1Name(m) => m.payload_size(),
            Self::ZxdgOutputV1Description(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZxdgOutputV1LogicalPosition(m) => m.write(writer, state).await,
            Self::ZxdgOutputV1LogicalSize(m) => m.write(writer, state).await,
            Self::ZxdgOutputV1Done(m) => m.write(writer, state).await,
            Self::ZxdgOutputV1Name(m) => m.write(writer, state).await,
            Self::ZxdgOutputV1Description(m) => m.write(writer, state).await,
        }
    }
}
