use super::{
    ExtForeignToplevelHandleV1AppIdEvent, ExtForeignToplevelHandleV1ClosedEvent,
    ExtForeignToplevelHandleV1DoneEvent, ExtForeignToplevelHandleV1IdentifierEvent,
    ExtForeignToplevelHandleV1TitleEvent, ExtForeignToplevelListV1FinishedEvent,
    ExtForeignToplevelListV1ToplevelEvent,
};
use crate::protocol::ext::foreign_toplevel_list::ObjectInteface as ForeignToplevelListObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Foreign toplevel list events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtForeignToplevelListV1ToplevelEvent].
    ExtForeignToplevelListV1Toplevel(ExtForeignToplevelListV1ToplevelEvent),
    /// Message [ExtForeignToplevelListV1FinishedEvent].
    ExtForeignToplevelListV1Finished(ExtForeignToplevelListV1FinishedEvent),
    /// Message [ExtForeignToplevelHandleV1ClosedEvent].
    ExtForeignToplevelHandleV1Closed(ExtForeignToplevelHandleV1ClosedEvent),
    /// Message [ExtForeignToplevelHandleV1DoneEvent].
    ExtForeignToplevelHandleV1Done(ExtForeignToplevelHandleV1DoneEvent),
    /// Message [ExtForeignToplevelHandleV1TitleEvent].
    ExtForeignToplevelHandleV1Title(ExtForeignToplevelHandleV1TitleEvent),
    /// Message [ExtForeignToplevelHandleV1AppIdEvent].
    ExtForeignToplevelHandleV1AppId(ExtForeignToplevelHandleV1AppIdEvent),
    /// Message [ExtForeignToplevelHandleV1IdentifierEvent].
    ExtForeignToplevelHandleV1Identifier(ExtForeignToplevelHandleV1IdentifierEvent),
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
            ObjectInteface::ForeignToplevelList(
                ForeignToplevelListObjectInteface::ExtForeignToplevelListV1 { .. },
            ) => match opcode {
                ExtForeignToplevelListV1ToplevelEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelListV1Toplevel(
                        ExtForeignToplevelListV1ToplevelEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelListV1FinishedEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelListV1Finished(
                        ExtForeignToplevelListV1FinishedEvent::read_with_interface(
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
                ExtForeignToplevelHandleV1ClosedEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1Closed(
                        ExtForeignToplevelHandleV1ClosedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelHandleV1DoneEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1Done(
                        ExtForeignToplevelHandleV1DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelHandleV1TitleEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1Title(
                        ExtForeignToplevelHandleV1TitleEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelHandleV1AppIdEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1AppId(
                        ExtForeignToplevelHandleV1AppIdEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtForeignToplevelHandleV1IdentifierEvent::OPCODE => {
                    Ok(Self::ExtForeignToplevelHandleV1Identifier(
                        ExtForeignToplevelHandleV1IdentifierEvent::read_with_interface(
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
            Self::ExtForeignToplevelListV1Toplevel(message) => message.object_id(),
            Self::ExtForeignToplevelListV1Finished(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1Closed(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1Done(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1Title(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1AppId(message) => message.object_id(),
            Self::ExtForeignToplevelHandleV1Identifier(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtForeignToplevelListV1Toplevel(message) => message.opcode(),
            Self::ExtForeignToplevelListV1Finished(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1Closed(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1Done(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1Title(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1AppId(message) => message.opcode(),
            Self::ExtForeignToplevelHandleV1Identifier(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtForeignToplevelListV1Toplevel(message) => message.payload_size(),
            Self::ExtForeignToplevelListV1Finished(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1Closed(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1Done(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1Title(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1AppId(message) => message.payload_size(),
            Self::ExtForeignToplevelHandleV1Identifier(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtForeignToplevelListV1Toplevel(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelListV1Finished(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1Closed(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1Done(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1Title(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1AppId(message) => message.write(writer, state).await,
            Self::ExtForeignToplevelHandleV1Identifier(message) => {
                message.write(writer, state).await
            }
        }
    }
}
