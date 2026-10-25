use super::{ExtIdleNotificationV1IdledEvent, ExtIdleNotificationV1ResumedEvent};
use crate::protocol::ext::idle_notify::ObjectInteface as IdleNotifyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Idle notify events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtIdleNotificationV1IdledEvent].
    ExtIdleNotificationV1Idled(ExtIdleNotificationV1IdledEvent),
    /// Message [ExtIdleNotificationV1ResumedEvent].
    ExtIdleNotificationV1Resumed(ExtIdleNotificationV1ResumedEvent),
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
            ObjectInteface::IdleNotify(IdleNotifyObjectInteface::ExtIdleNotificationV1 {
                ..
            }) => match opcode {
                ExtIdleNotificationV1IdledEvent::OPCODE => Ok(Self::ExtIdleNotificationV1Idled(
                    ExtIdleNotificationV1IdledEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ExtIdleNotificationV1ResumedEvent::OPCODE => {
                    Ok(Self::ExtIdleNotificationV1Resumed(
                        ExtIdleNotificationV1ResumedEvent::read_with_interface(
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
            Self::ExtIdleNotificationV1Idled(message) => message.object_id(),
            Self::ExtIdleNotificationV1Resumed(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtIdleNotificationV1Idled(message) => message.opcode(),
            Self::ExtIdleNotificationV1Resumed(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtIdleNotificationV1Idled(message) => message.payload_size(),
            Self::ExtIdleNotificationV1Resumed(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtIdleNotificationV1Idled(message) => message.write(writer, state).await,
            Self::ExtIdleNotificationV1Resumed(message) => message.write(writer, state).await,
        }
    }
}
