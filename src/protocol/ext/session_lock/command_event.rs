use super::{
    ExtSessionLockSurfaceV1ConfigureEvent, ExtSessionLockV1FinishedEvent,
    ExtSessionLockV1LockedEvent,
};
use crate::protocol::ext::session_lock::ObjectInteface as SessionLockObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Session lock events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtSessionLockV1LockedEvent].
    ExtSessionLockV1Locked(ExtSessionLockV1LockedEvent),
    /// Message [ExtSessionLockV1FinishedEvent].
    ExtSessionLockV1Finished(ExtSessionLockV1FinishedEvent),
    /// Message [ExtSessionLockSurfaceV1ConfigureEvent].
    ExtSessionLockSurfaceV1Configure(ExtSessionLockSurfaceV1ConfigureEvent),
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
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockV1 { .. }) => {
                match opcode {
                    ExtSessionLockV1LockedEvent::OPCODE => Ok(Self::ExtSessionLockV1Locked(
                        ExtSessionLockV1LockedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ExtSessionLockV1FinishedEvent::OPCODE => Ok(Self::ExtSessionLockV1Finished(
                        ExtSessionLockV1FinishedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockSurfaceV1 {
                ..
            }) => match opcode {
                ExtSessionLockSurfaceV1ConfigureEvent::OPCODE => {
                    Ok(Self::ExtSessionLockSurfaceV1Configure(
                        ExtSessionLockSurfaceV1ConfigureEvent::read_with_interface(
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
            Self::ExtSessionLockV1Locked(message) => message.object_id(),
            Self::ExtSessionLockV1Finished(message) => message.object_id(),
            Self::ExtSessionLockSurfaceV1Configure(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtSessionLockV1Locked(message) => message.opcode(),
            Self::ExtSessionLockV1Finished(message) => message.opcode(),
            Self::ExtSessionLockSurfaceV1Configure(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtSessionLockV1Locked(message) => message.payload_size(),
            Self::ExtSessionLockV1Finished(message) => message.payload_size(),
            Self::ExtSessionLockSurfaceV1Configure(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtSessionLockV1Locked(message) => message.write(writer, state).await,
            Self::ExtSessionLockV1Finished(message) => message.write(writer, state).await,
            Self::ExtSessionLockSurfaceV1Configure(message) => message.write(writer, state).await,
        }
    }
}
