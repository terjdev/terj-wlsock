use super::{ExtTransientSeatV1DeniedEvent, ExtTransientSeatV1ReadyEvent};
use crate::protocol::ext::transient_seat::ObjectInteface as TransientSeatObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Transient seat events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtTransientSeatV1ReadyEvent].
    ExtTransientSeatV1Ready(ExtTransientSeatV1ReadyEvent),
    /// Message [ExtTransientSeatV1DeniedEvent].
    ExtTransientSeatV1Denied(ExtTransientSeatV1DeniedEvent),
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
            ObjectInteface::TransientSeat(TransientSeatObjectInteface::ExtTransientSeatV1 {
                ..
            }) => match opcode {
                ExtTransientSeatV1ReadyEvent::OPCODE => Ok(Self::ExtTransientSeatV1Ready(
                    ExtTransientSeatV1ReadyEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ExtTransientSeatV1DeniedEvent::OPCODE => Ok(Self::ExtTransientSeatV1Denied(
                    ExtTransientSeatV1DeniedEvent::read_with_interface(
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
            Self::ExtTransientSeatV1Ready(message) => message.object_id(),
            Self::ExtTransientSeatV1Denied(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtTransientSeatV1Ready(message) => message.opcode(),
            Self::ExtTransientSeatV1Denied(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtTransientSeatV1Ready(message) => message.payload_size(),
            Self::ExtTransientSeatV1Denied(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtTransientSeatV1Ready(message) => message.write(writer, state).await,
            Self::ExtTransientSeatV1Denied(message) => message.write(writer, state).await,
        }
    }
}
