use super::ObjectInteface as XdgOutputObjectInteface;
use super::{
    ZxdgOutputManagerV1DestroyRequest, ZxdgOutputManagerV1GetXdgOutputRequest,
    ZxdgOutputV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xdg output requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZxdgOutputManagerV1DestroyRequest].
    ZxdgOutputManagerV1Destroy(ZxdgOutputManagerV1DestroyRequest),
    /// Message [ZxdgOutputManagerV1GetXdgOutputRequest].
    ZxdgOutputManagerV1GetXdgOutput(ZxdgOutputManagerV1GetXdgOutputRequest),
    /// Message [ZxdgOutputV1DestroyRequest].
    ZxdgOutputV1Destroy(ZxdgOutputV1DestroyRequest),
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
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputManagerV1 { .. }) => {
                match opcode {
                    ZxdgOutputManagerV1DestroyRequest::OPCODE => {
                        Ok(Self::ZxdgOutputManagerV1Destroy(
                            ZxdgOutputManagerV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZxdgOutputManagerV1GetXdgOutputRequest::OPCODE => {
                        Ok(Self::ZxdgOutputManagerV1GetXdgOutput(
                            ZxdgOutputManagerV1GetXdgOutputRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgOutput(XdgOutputObjectInteface::ZxdgOutputV1 { .. }) => match opcode
            {
                ZxdgOutputV1DestroyRequest::OPCODE => Ok(Self::ZxdgOutputV1Destroy(
                    ZxdgOutputV1DestroyRequest::read_with_interface(
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
            Self::ZxdgOutputManagerV1Destroy(m) => m.object_id(),
            Self::ZxdgOutputManagerV1GetXdgOutput(m) => m.object_id(),
            Self::ZxdgOutputV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZxdgOutputManagerV1Destroy(m) => m.opcode(),
            Self::ZxdgOutputManagerV1GetXdgOutput(m) => m.opcode(),
            Self::ZxdgOutputV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZxdgOutputManagerV1Destroy(m) => m.payload_size(),
            Self::ZxdgOutputManagerV1GetXdgOutput(m) => m.payload_size(),
            Self::ZxdgOutputV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZxdgOutputManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZxdgOutputManagerV1GetXdgOutput(m) => m.write(writer, state).await,
            Self::ZxdgOutputV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
