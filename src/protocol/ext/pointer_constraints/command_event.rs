use super::ObjectInteface as PointerConstraintsObjectInteface;
use super::{
    ZwpConfinedPointerV1ConfinedEvent, ZwpConfinedPointerV1UnconfinedEvent,
    ZwpLockedPointerV1LockedEvent, ZwpLockedPointerV1UnlockedEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches pointer constraints events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpLockedPointerV1LockedEvent].
    ZwpLockedPointerV1Locked(ZwpLockedPointerV1LockedEvent),
    /// Message [ZwpLockedPointerV1UnlockedEvent].
    ZwpLockedPointerV1Unlocked(ZwpLockedPointerV1UnlockedEvent),
    /// Message [ZwpConfinedPointerV1ConfinedEvent].
    ZwpConfinedPointerV1Confined(ZwpConfinedPointerV1ConfinedEvent),
    /// Message [ZwpConfinedPointerV1UnconfinedEvent].
    ZwpConfinedPointerV1Unconfined(ZwpConfinedPointerV1UnconfinedEvent),
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
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpLockedPointerV1 { .. },
            ) => match opcode {
                ZwpLockedPointerV1LockedEvent::OPCODE => Ok(Self::ZwpLockedPointerV1Locked(
                    ZwpLockedPointerV1LockedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpLockedPointerV1UnlockedEvent::OPCODE => Ok(Self::ZwpLockedPointerV1Unlocked(
                    ZwpLockedPointerV1UnlockedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. },
            ) => match opcode {
                ZwpConfinedPointerV1ConfinedEvent::OPCODE => {
                    Ok(Self::ZwpConfinedPointerV1Confined(
                        ZwpConfinedPointerV1ConfinedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpConfinedPointerV1UnconfinedEvent::OPCODE => {
                    Ok(Self::ZwpConfinedPointerV1Unconfined(
                        ZwpConfinedPointerV1UnconfinedEvent::read_with_interface(
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
            Self::ZwpLockedPointerV1Locked(m) => m.object_id(),
            Self::ZwpLockedPointerV1Unlocked(m) => m.object_id(),
            Self::ZwpConfinedPointerV1Confined(m) => m.object_id(),
            Self::ZwpConfinedPointerV1Unconfined(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpLockedPointerV1Locked(m) => m.opcode(),
            Self::ZwpLockedPointerV1Unlocked(m) => m.opcode(),
            Self::ZwpConfinedPointerV1Confined(m) => m.opcode(),
            Self::ZwpConfinedPointerV1Unconfined(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpLockedPointerV1Locked(m) => m.payload_size(),
            Self::ZwpLockedPointerV1Unlocked(m) => m.payload_size(),
            Self::ZwpConfinedPointerV1Confined(m) => m.payload_size(),
            Self::ZwpConfinedPointerV1Unconfined(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpLockedPointerV1Locked(m) => m.write(writer, state).await,
            Self::ZwpLockedPointerV1Unlocked(m) => m.write(writer, state).await,
            Self::ZwpConfinedPointerV1Confined(m) => m.write(writer, state).await,
            Self::ZwpConfinedPointerV1Unconfined(m) => m.write(writer, state).await,
        }
    }
}
