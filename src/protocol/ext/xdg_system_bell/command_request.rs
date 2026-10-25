use super::{XdgSystemBellV1DestroyRequest, XdgSystemBellV1RingRequest};
use crate::protocol::ext::xdg_system_bell::ObjectInteface as XdgSystemBellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg system bell requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XdgSystemBellV1DestroyRequest].
    XdgSystemBellV1Destroy(XdgSystemBellV1DestroyRequest),
    /// Message [XdgSystemBellV1RingRequest].
    XdgSystemBellV1Ring(XdgSystemBellV1RingRequest),
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
            ObjectInteface::XdgSystemBell(XdgSystemBellObjectInteface::XdgSystemBellV1 {
                ..
            }) => match opcode {
                XdgSystemBellV1DestroyRequest::OPCODE => Ok(Self::XdgSystemBellV1Destroy(
                    XdgSystemBellV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSystemBellV1RingRequest::OPCODE => Ok(Self::XdgSystemBellV1Ring(
                    XdgSystemBellV1RingRequest::read_with_interface(
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
            Self::XdgSystemBellV1Destroy(message) => message.object_id(),
            Self::XdgSystemBellV1Ring(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgSystemBellV1Destroy(message) => message.opcode(),
            Self::XdgSystemBellV1Ring(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgSystemBellV1Destroy(message) => message.payload_size(),
            Self::XdgSystemBellV1Ring(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgSystemBellV1Destroy(message) => message.write(writer, state).await,
            Self::XdgSystemBellV1Ring(message) => message.write(writer, state).await,
        }
    }
}
