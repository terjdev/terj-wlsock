use super::ObjectInteface as KeyboardShortcutsInhibitObjectInteface;
use super::{
    ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest,
    ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest,
    ZwpKeyboardShortcutsInhibitorV1DestroyRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches keyboard shortcuts inhibit requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest].
    ZwpKeyboardShortcutsInhibitManagerV1Destroy(ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest),
    /// Message [ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest].
    ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(
        ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest,
    ),
    /// Message [ZwpKeyboardShortcutsInhibitorV1DestroyRequest].
    ZwpKeyboardShortcutsInhibitorV1Destroy(ZwpKeyboardShortcutsInhibitorV1DestroyRequest),
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
ObjectInteface::KeyboardShortcutsInhibit(KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitManagerV1{..})=>match opcode{ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest::OPCODE=>Ok(Self::ZwpKeyboardShortcutsInhibitManagerV1Destroy(ZwpKeyboardShortcutsInhibitManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest::OPCODE=>Ok(Self::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcutsRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::KeyboardShortcutsInhibit(KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1{..})=>match opcode{ZwpKeyboardShortcutsInhibitorV1DestroyRequest::OPCODE=>Ok(Self::ZwpKeyboardShortcutsInhibitorV1Destroy(ZwpKeyboardShortcutsInhibitorV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1Destroy(m) => m.object_id(),
            Self::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(m) => m.object_id(),
            Self::ZwpKeyboardShortcutsInhibitorV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1Destroy(m) => m.opcode(),
            Self::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(m) => m.opcode(),
            Self::ZwpKeyboardShortcutsInhibitorV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(m) => m.payload_size(),
            Self::ZwpKeyboardShortcutsInhibitorV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpKeyboardShortcutsInhibitManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpKeyboardShortcutsInhibitManagerV1InhibitShortcuts(m) => {
                m.write(writer, state).await
            }
            Self::ZwpKeyboardShortcutsInhibitorV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
