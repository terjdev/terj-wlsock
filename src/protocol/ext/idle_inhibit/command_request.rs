use super::ObjectInteface as IdleInhibitObjectInteface;
use super::{
    ZwpIdleInhibitManagerV1CreateInhibitorRequest, ZwpIdleInhibitManagerV1DestroyRequest,
    ZwpIdleInhibitorV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches idle inhibit requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpIdleInhibitManagerV1DestroyRequest].
    ZwpIdleInhibitManagerV1Destroy(ZwpIdleInhibitManagerV1DestroyRequest),
    /// Message [ZwpIdleInhibitManagerV1CreateInhibitorRequest].
    ZwpIdleInhibitManagerV1CreateInhibitor(ZwpIdleInhibitManagerV1CreateInhibitorRequest),
    /// Message [ZwpIdleInhibitorV1DestroyRequest].
    ZwpIdleInhibitorV1Destroy(ZwpIdleInhibitorV1DestroyRequest),
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
            ObjectInteface::IdleInhibit(IdleInhibitObjectInteface::ZwpIdleInhibitManagerV1 {
                ..
            }) => match opcode {
                ZwpIdleInhibitManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpIdleInhibitManagerV1Destroy(
                        ZwpIdleInhibitManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpIdleInhibitManagerV1CreateInhibitorRequest::OPCODE => {
                    Ok(Self::ZwpIdleInhibitManagerV1CreateInhibitor(
                        ZwpIdleInhibitManagerV1CreateInhibitorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::IdleInhibit(IdleInhibitObjectInteface::ZwpIdleInhibitorV1 {
                ..
            }) => match opcode {
                ZwpIdleInhibitorV1DestroyRequest::OPCODE => Ok(Self::ZwpIdleInhibitorV1Destroy(
                    ZwpIdleInhibitorV1DestroyRequest::read_with_interface(
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
            Self::ZwpIdleInhibitManagerV1Destroy(m) => m.object_id(),
            Self::ZwpIdleInhibitManagerV1CreateInhibitor(m) => m.object_id(),
            Self::ZwpIdleInhibitorV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpIdleInhibitManagerV1Destroy(m) => m.opcode(),
            Self::ZwpIdleInhibitManagerV1CreateInhibitor(m) => m.opcode(),
            Self::ZwpIdleInhibitorV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpIdleInhibitManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpIdleInhibitManagerV1CreateInhibitor(m) => m.payload_size(),
            Self::ZwpIdleInhibitorV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpIdleInhibitManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpIdleInhibitManagerV1CreateInhibitor(m) => m.write(writer, state).await,
            Self::ZwpIdleInhibitorV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
