use super::{
    XdgSessionV1CreatedEvent, XdgSessionV1ReplacedEvent, XdgSessionV1RestoredEvent,
    XdgToplevelSessionV1RestoredEvent,
};
use crate::protocol::ext::xdg_session_management::ObjectInteface as XdgSessionManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xdg session management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XdgSessionV1CreatedEvent].
    XdgSessionV1Created(XdgSessionV1CreatedEvent),
    /// Message [XdgSessionV1RestoredEvent].
    XdgSessionV1Restored(XdgSessionV1RestoredEvent),
    /// Message [XdgSessionV1ReplacedEvent].
    XdgSessionV1Replaced(XdgSessionV1ReplacedEvent),
    /// Message [XdgToplevelSessionV1RestoredEvent].
    XdgToplevelSessionV1Restored(XdgToplevelSessionV1RestoredEvent),
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
            ObjectInteface::XdgSessionManagement(
                XdgSessionManagementObjectInteface::XdgSessionV1 { .. },
            ) => match opcode {
                XdgSessionV1CreatedEvent::OPCODE => Ok(Self::XdgSessionV1Created(
                    XdgSessionV1CreatedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionV1RestoredEvent::OPCODE => Ok(Self::XdgSessionV1Restored(
                    XdgSessionV1RestoredEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XdgSessionV1ReplacedEvent::OPCODE => Ok(Self::XdgSessionV1Replaced(
                    XdgSessionV1ReplacedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XdgSessionManagement(
                XdgSessionManagementObjectInteface::XdgToplevelSessionV1 { .. },
            ) => match opcode {
                XdgToplevelSessionV1RestoredEvent::OPCODE => {
                    Ok(Self::XdgToplevelSessionV1Restored(
                        XdgToplevelSessionV1RestoredEvent::read_with_interface(
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
            Self::XdgSessionV1Created(message) => message.object_id(),
            Self::XdgSessionV1Restored(message) => message.object_id(),
            Self::XdgSessionV1Replaced(message) => message.object_id(),
            Self::XdgToplevelSessionV1Restored(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XdgSessionV1Created(message) => message.opcode(),
            Self::XdgSessionV1Restored(message) => message.opcode(),
            Self::XdgSessionV1Replaced(message) => message.opcode(),
            Self::XdgToplevelSessionV1Restored(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XdgSessionV1Created(message) => message.payload_size(),
            Self::XdgSessionV1Restored(message) => message.payload_size(),
            Self::XdgSessionV1Replaced(message) => message.payload_size(),
            Self::XdgToplevelSessionV1Restored(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XdgSessionV1Created(message) => message.write(writer, state).await,
            Self::XdgSessionV1Restored(message) => message.write(writer, state).await,
            Self::XdgSessionV1Replaced(message) => message.write(writer, state).await,
            Self::XdgToplevelSessionV1Restored(message) => message.write(writer, state).await,
        }
    }
}
