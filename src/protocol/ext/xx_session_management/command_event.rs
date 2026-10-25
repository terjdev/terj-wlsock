use super::ObjectInteface as XxSessionManagementObjectInteface;
use super::{
    XxSessionV1CreatedEvent, XxSessionV1ReplacedEvent, XxSessionV1RestoredEvent,
    XxToplevelSessionV1RestoredEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches session management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxSessionV1CreatedEvent].
    XxSessionV1Created(XxSessionV1CreatedEvent),
    /// Message [XxSessionV1RestoredEvent].
    XxSessionV1Restored(XxSessionV1RestoredEvent),
    /// Message [XxSessionV1ReplacedEvent].
    XxSessionV1Replaced(XxSessionV1ReplacedEvent),
    /// Message [XxToplevelSessionV1RestoredEvent].
    XxToplevelSessionV1Restored(XxToplevelSessionV1RestoredEvent),
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
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 { .. },
            ) => match opcode {
                XxSessionV1CreatedEvent::OPCODE => Ok(Self::XxSessionV1Created(
                    XxSessionV1CreatedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionV1RestoredEvent::OPCODE => Ok(Self::XxSessionV1Restored(
                    XxSessionV1RestoredEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxSessionV1ReplacedEvent::OPCODE => Ok(Self::XxSessionV1Replaced(
                    XxSessionV1ReplacedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 { .. },
            ) => match opcode {
                XxToplevelSessionV1RestoredEvent::OPCODE => Ok(Self::XxToplevelSessionV1Restored(
                    XxToplevelSessionV1RestoredEvent::read_with_interface(
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
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::XxSessionV1Created(m) => m.object_id(),
            Self::XxSessionV1Restored(m) => m.object_id(),
            Self::XxSessionV1Replaced(m) => m.object_id(),
            Self::XxToplevelSessionV1Restored(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxSessionV1Created(m) => m.opcode(),
            Self::XxSessionV1Restored(m) => m.opcode(),
            Self::XxSessionV1Replaced(m) => m.opcode(),
            Self::XxToplevelSessionV1Restored(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxSessionV1Created(m) => m.payload_size(),
            Self::XxSessionV1Restored(m) => m.payload_size(),
            Self::XxSessionV1Replaced(m) => m.payload_size(),
            Self::XxToplevelSessionV1Restored(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxSessionV1Created(m) => m.write(writer, state).await,
            Self::XxSessionV1Restored(m) => m.write(writer, state).await,
            Self::XxSessionV1Replaced(m) => m.write(writer, state).await,
            Self::XxToplevelSessionV1Restored(m) => m.write(writer, state).await,
        }
    }
}
