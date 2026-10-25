use super::{
    ExtForeignToplevelHandleV1DestroyRequest, ExtForeignToplevelListV1DestroyRequest,
    ExtForeignToplevelListV1StopRequest,
};
use crate::protocol::ext::foreign_toplevel_list::ObjectInteface as ForeignToplevelListObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Foreign toplevel list requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtForeignToplevelListV1StopRequest].
    ExtForeignToplevelListV1Stop(ExtForeignToplevelListV1StopRequest),
    /// Message [ExtForeignToplevelListV1DestroyRequest].
    ExtForeignToplevelListV1Destroy(ExtForeignToplevelListV1DestroyRequest),
    /// Message [ExtForeignToplevelHandleV1DestroyRequest].
    ExtForeignToplevelHandleV1Destroy(ExtForeignToplevelHandleV1DestroyRequest),
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
            ObjectInteface::ForeignToplevelList(
                ForeignToplevelListObjectInteface::ExtForeignToplevelListV1 { .. },
            ) => match opcode {
                ExtForeignToplevelListV1StopRequest::OPCODE => {
                    Ok(Self::ExtForeignToplevelListV1Stop(
                        ExtForeignToplevelListV1StopRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelListV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtForeignToplevelListV1Destroy(
                        ExtForeignToplevelListV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::ForeignToplevelList(
                ForeignToplevelListObjectInteface::ExtForeignToplevelHandleV1 { .. },
            ) => match opcode {
                ExtForeignToplevelHandleV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1Destroy(
                        ExtForeignToplevelHandleV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtForeignToplevelListV1Stop(message) => message.object_id(),
            Self::ExtForeignToplevelListV1Destroy(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtForeignToplevelListV1Stop(message) => message.opcode(),
            Self::ExtForeignToplevelListV1Destroy(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtForeignToplevelListV1Stop(message) => message.payload_size(),
            Self::ExtForeignToplevelListV1Destroy(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtForeignToplevelListV1Stop(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelListV1Destroy(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
