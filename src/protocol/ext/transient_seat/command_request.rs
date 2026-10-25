use super::{
    ExtTransientSeatManagerV1CreateRequest, ExtTransientSeatManagerV1DestroyRequest,
    ExtTransientSeatV1DestroyRequest,
};
use crate::protocol::ext::transient_seat::ObjectInteface as TransientSeatObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Transient seat requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtTransientSeatManagerV1CreateRequest].
    ExtTransientSeatManagerV1Create(ExtTransientSeatManagerV1CreateRequest),
    /// Message [ExtTransientSeatManagerV1DestroyRequest].
    ExtTransientSeatManagerV1Destroy(ExtTransientSeatManagerV1DestroyRequest),
    /// Message [ExtTransientSeatV1DestroyRequest].
    ExtTransientSeatV1Destroy(ExtTransientSeatV1DestroyRequest),
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
            ObjectInteface::TransientSeat(
                TransientSeatObjectInteface::ExtTransientSeatManagerV1 { .. },
            ) => match opcode {
                ExtTransientSeatManagerV1CreateRequest::OPCODE => {
                    Ok(Self::ExtTransientSeatManagerV1Create(
                        ExtTransientSeatManagerV1CreateRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtTransientSeatManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtTransientSeatManagerV1Destroy(
                        ExtTransientSeatManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TransientSeat(TransientSeatObjectInteface::ExtTransientSeatV1 {
                ..
            }) => match opcode {
                ExtTransientSeatV1DestroyRequest::OPCODE => Ok(Self::ExtTransientSeatV1Destroy(
                    ExtTransientSeatV1DestroyRequest::read_with_interface(
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
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtTransientSeatManagerV1Create(message) => message.object_id(),
            Self::ExtTransientSeatManagerV1Destroy(message) => message.object_id(),
            Self::ExtTransientSeatV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtTransientSeatManagerV1Create(message) => message.opcode(),
            Self::ExtTransientSeatManagerV1Destroy(message) => message.opcode(),
            Self::ExtTransientSeatV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtTransientSeatManagerV1Create(message) => message.payload_size(),
            Self::ExtTransientSeatManagerV1Destroy(message) => message.payload_size(),
            Self::ExtTransientSeatV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtTransientSeatManagerV1Create(message) => message.write(writer, state).await,
            Self::ExtTransientSeatManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ExtTransientSeatV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
