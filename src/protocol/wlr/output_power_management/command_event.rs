use super::ObjectInteface as WlrOutputPowerManagementObjectInteface;
use super::{ZwlrOutputPowerV1FailedEvent, ZwlrOutputPowerV1ModeEvent};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches output power management events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrOutputPowerV1ModeEvent].
    ZwlrOutputPowerV1Mode(ZwlrOutputPowerV1ModeEvent),
    /// Message [ZwlrOutputPowerV1FailedEvent].
    ZwlrOutputPowerV1Failed(ZwlrOutputPowerV1FailedEvent),
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
            ObjectInteface::WlrOutputPowerManagement(
                WlrOutputPowerManagementObjectInteface::ZwlrOutputPowerV1 { .. },
            ) => match opcode {
                ZwlrOutputPowerV1ModeEvent::OPCODE => Ok(Self::ZwlrOutputPowerV1Mode(
                    ZwlrOutputPowerV1ModeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrOutputPowerV1FailedEvent::OPCODE => Ok(Self::ZwlrOutputPowerV1Failed(
                    ZwlrOutputPowerV1FailedEvent::read_with_interface(
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
            Self::ZwlrOutputPowerV1Mode(m) => m.object_id(),
            Self::ZwlrOutputPowerV1Failed(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrOutputPowerV1Mode(m) => m.opcode(),
            Self::ZwlrOutputPowerV1Failed(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrOutputPowerV1Mode(m) => m.payload_size(),
            Self::ZwlrOutputPowerV1Failed(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrOutputPowerV1Mode(m) => m.write(writer, state).await,
            Self::ZwlrOutputPowerV1Failed(m) => m.write(writer, state).await,
        }
    }
}
