use super::ObjectInteface as WlrInputInhibitorObjectInteface;
use super::{ZwlrInputInhibitManagerV1GetInhibitorRequest, ZwlrInputInhibitorV1DestroyRequest};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches input inhibitor requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrInputInhibitManagerV1GetInhibitorRequest].
    ZwlrInputInhibitManagerV1GetInhibitor(ZwlrInputInhibitManagerV1GetInhibitorRequest),
    /// Message [ZwlrInputInhibitorV1DestroyRequest].
    ZwlrInputInhibitorV1Destroy(ZwlrInputInhibitorV1DestroyRequest),
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
            ObjectInteface::WlrInputInhibitor(
                WlrInputInhibitorObjectInteface::ZwlrInputInhibitManagerV1 { .. },
            ) => match opcode {
                ZwlrInputInhibitManagerV1GetInhibitorRequest::OPCODE => {
                    Ok(Self::ZwlrInputInhibitManagerV1GetInhibitor(
                        ZwlrInputInhibitManagerV1GetInhibitorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrInputInhibitor(
                WlrInputInhibitorObjectInteface::ZwlrInputInhibitorV1 { .. },
            ) => match opcode {
                ZwlrInputInhibitorV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrInputInhibitorV1Destroy(
                        ZwlrInputInhibitorV1DestroyRequest::read_with_interface(
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
            Self::ZwlrInputInhibitManagerV1GetInhibitor(m) => m.object_id(),
            Self::ZwlrInputInhibitorV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrInputInhibitManagerV1GetInhibitor(m) => m.opcode(),
            Self::ZwlrInputInhibitorV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrInputInhibitManagerV1GetInhibitor(m) => m.payload_size(),
            Self::ZwlrInputInhibitorV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrInputInhibitManagerV1GetInhibitor(m) => m.write(writer, state).await,
            Self::ZwlrInputInhibitorV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
