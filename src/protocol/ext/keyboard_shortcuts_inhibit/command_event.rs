use super::ObjectInteface as KeyboardShortcutsInhibitObjectInteface;
use super::{
    ZwpKeyboardShortcutsInhibitorV1ActiveEvent, ZwpKeyboardShortcutsInhibitorV1InactiveEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches keyboard shortcuts inhibit events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpKeyboardShortcutsInhibitorV1ActiveEvent].
    ZwpKeyboardShortcutsInhibitorV1Active(ZwpKeyboardShortcutsInhibitorV1ActiveEvent),
    /// Message [ZwpKeyboardShortcutsInhibitorV1InactiveEvent].
    ZwpKeyboardShortcutsInhibitorV1Inactive(ZwpKeyboardShortcutsInhibitorV1InactiveEvent),
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
            ObjectInteface::KeyboardShortcutsInhibit(
                KeyboardShortcutsInhibitObjectInteface::ZwpKeyboardShortcutsInhibitorV1 { .. },
            ) => match opcode {
                ZwpKeyboardShortcutsInhibitorV1ActiveEvent::OPCODE => {
                    Ok(Self::ZwpKeyboardShortcutsInhibitorV1Active(
                        ZwpKeyboardShortcutsInhibitorV1ActiveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpKeyboardShortcutsInhibitorV1InactiveEvent::OPCODE => {
                    Ok(Self::ZwpKeyboardShortcutsInhibitorV1Inactive(
                        ZwpKeyboardShortcutsInhibitorV1InactiveEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitorV1Active(m) => m.object_id(),
            Self::ZwpKeyboardShortcutsInhibitorV1Inactive(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpKeyboardShortcutsInhibitorV1Active(m) => m.opcode(),
            Self::ZwpKeyboardShortcutsInhibitorV1Inactive(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpKeyboardShortcutsInhibitorV1Active(m) => m.payload_size(),
            Self::ZwpKeyboardShortcutsInhibitorV1Inactive(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpKeyboardShortcutsInhibitorV1Active(m) => m.write(writer, state).await,
            Self::ZwpKeyboardShortcutsInhibitorV1Inactive(m) => m.write(writer, state).await,
        }
    }
}
