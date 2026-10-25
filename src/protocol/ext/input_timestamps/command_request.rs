use super::ObjectInteface as InputTimestampsObjectInteface;
use super::{
    ZwpInputTimestampsManagerV1DestroyRequest,
    ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest,
    ZwpInputTimestampsManagerV1GetPointerTimestampsRequest,
    ZwpInputTimestampsManagerV1GetTouchTimestampsRequest, ZwpInputTimestampsV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches input timestamps requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpInputTimestampsManagerV1DestroyRequest].
    ZwpInputTimestampsManagerV1Destroy(ZwpInputTimestampsManagerV1DestroyRequest),
    /// Message [ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest].
    ZwpInputTimestampsManagerV1GetKeyboardTimestamps(
        ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest,
    ),
    /// Message [ZwpInputTimestampsManagerV1GetPointerTimestampsRequest].
    ZwpInputTimestampsManagerV1GetPointerTimestamps(
        ZwpInputTimestampsManagerV1GetPointerTimestampsRequest,
    ),
    /// Message [ZwpInputTimestampsManagerV1GetTouchTimestampsRequest].
    ZwpInputTimestampsManagerV1GetTouchTimestamps(
        ZwpInputTimestampsManagerV1GetTouchTimestampsRequest,
    ),
    /// Message [ZwpInputTimestampsV1DestroyRequest].
    ZwpInputTimestampsV1Destroy(ZwpInputTimestampsV1DestroyRequest),
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
        match interface{
ObjectInteface::InputTimestamps(InputTimestampsObjectInteface::ZwpInputTimestampsManagerV1{..})=>match opcode{ZwpInputTimestampsManagerV1DestroyRequest::OPCODE=>Ok(Self::ZwpInputTimestampsManagerV1Destroy(ZwpInputTimestampsManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest::OPCODE=>Ok(Self::ZwpInputTimestampsManagerV1GetKeyboardTimestamps(ZwpInputTimestampsManagerV1GetKeyboardTimestampsRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpInputTimestampsManagerV1GetPointerTimestampsRequest::OPCODE=>Ok(Self::ZwpInputTimestampsManagerV1GetPointerTimestamps(ZwpInputTimestampsManagerV1GetPointerTimestampsRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpInputTimestampsManagerV1GetTouchTimestampsRequest::OPCODE=>Ok(Self::ZwpInputTimestampsManagerV1GetTouchTimestamps(ZwpInputTimestampsManagerV1GetTouchTimestampsRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::InputTimestamps(InputTimestampsObjectInteface::ZwpInputTimestampsV1{..})=>match opcode{ZwpInputTimestampsV1DestroyRequest::OPCODE=>Ok(Self::ZwpInputTimestampsV1Destroy(ZwpInputTimestampsV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpInputTimestampsManagerV1Destroy(m) => m.object_id(),
            Self::ZwpInputTimestampsManagerV1GetKeyboardTimestamps(m) => m.object_id(),
            Self::ZwpInputTimestampsManagerV1GetPointerTimestamps(m) => m.object_id(),
            Self::ZwpInputTimestampsManagerV1GetTouchTimestamps(m) => m.object_id(),
            Self::ZwpInputTimestampsV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpInputTimestampsManagerV1Destroy(m) => m.opcode(),
            Self::ZwpInputTimestampsManagerV1GetKeyboardTimestamps(m) => m.opcode(),
            Self::ZwpInputTimestampsManagerV1GetPointerTimestamps(m) => m.opcode(),
            Self::ZwpInputTimestampsManagerV1GetTouchTimestamps(m) => m.opcode(),
            Self::ZwpInputTimestampsV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpInputTimestampsManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpInputTimestampsManagerV1GetKeyboardTimestamps(m) => m.payload_size(),
            Self::ZwpInputTimestampsManagerV1GetPointerTimestamps(m) => m.payload_size(),
            Self::ZwpInputTimestampsManagerV1GetTouchTimestamps(m) => m.payload_size(),
            Self::ZwpInputTimestampsV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpInputTimestampsManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpInputTimestampsManagerV1GetKeyboardTimestamps(m) => {
                m.write(writer, state).await
            }
            Self::ZwpInputTimestampsManagerV1GetPointerTimestamps(m) => {
                m.write(writer, state).await
            }
            Self::ZwpInputTimestampsManagerV1GetTouchTimestamps(m) => m.write(writer, state).await,
            Self::ZwpInputTimestampsV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
