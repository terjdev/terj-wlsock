use super::ObjectInteface as XxHotkeyObjectInteface;
use super::{
    XxHotkeyV1BoundEvent, XxHotkeyV1DeniedEvent, XxHotkeyV1ReleasedEvent, XxHotkeyV1RevokedEvent,
    XxHotkeyV1TriggeredEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches hotkey events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxHotkeyV1BoundEvent].
    XxHotkeyV1Bound(XxHotkeyV1BoundEvent),
    /// Message [XxHotkeyV1DeniedEvent].
    XxHotkeyV1Denied(XxHotkeyV1DeniedEvent),
    /// Message [XxHotkeyV1RevokedEvent].
    XxHotkeyV1Revoked(XxHotkeyV1RevokedEvent),
    /// Message [XxHotkeyV1TriggeredEvent].
    XxHotkeyV1Triggered(XxHotkeyV1TriggeredEvent),
    /// Message [XxHotkeyV1ReleasedEvent].
    XxHotkeyV1Released(XxHotkeyV1ReleasedEvent),
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
            ObjectInteface::XxHotkey(XxHotkeyObjectInteface::XxHotkeyV1 { .. }) => match opcode {
                XxHotkeyV1BoundEvent::OPCODE => Ok(Self::XxHotkeyV1Bound(
                    XxHotkeyV1BoundEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1DeniedEvent::OPCODE => Ok(Self::XxHotkeyV1Denied(
                    XxHotkeyV1DeniedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1RevokedEvent::OPCODE => Ok(Self::XxHotkeyV1Revoked(
                    XxHotkeyV1RevokedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1TriggeredEvent::OPCODE => Ok(Self::XxHotkeyV1Triggered(
                    XxHotkeyV1TriggeredEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxHotkeyV1ReleasedEvent::OPCODE => Ok(Self::XxHotkeyV1Released(
                    XxHotkeyV1ReleasedEvent::read_with_interface(
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
            Self::XxHotkeyV1Bound(m) => m.object_id(),
            Self::XxHotkeyV1Denied(m) => m.object_id(),
            Self::XxHotkeyV1Revoked(m) => m.object_id(),
            Self::XxHotkeyV1Triggered(m) => m.object_id(),
            Self::XxHotkeyV1Released(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxHotkeyV1Bound(m) => m.opcode(),
            Self::XxHotkeyV1Denied(m) => m.opcode(),
            Self::XxHotkeyV1Revoked(m) => m.opcode(),
            Self::XxHotkeyV1Triggered(m) => m.opcode(),
            Self::XxHotkeyV1Released(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxHotkeyV1Bound(m) => m.payload_size(),
            Self::XxHotkeyV1Denied(m) => m.payload_size(),
            Self::XxHotkeyV1Revoked(m) => m.payload_size(),
            Self::XxHotkeyV1Triggered(m) => m.payload_size(),
            Self::XxHotkeyV1Released(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxHotkeyV1Bound(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Denied(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Revoked(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Triggered(m) => m.write(writer, state).await,
            Self::XxHotkeyV1Released(m) => m.write(writer, state).await,
        }
    }
}
