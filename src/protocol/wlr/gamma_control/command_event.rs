use super::ObjectInteface as WlrGammaControlObjectInteface;
use super::{ZwlrGammaControlV1FailedEvent, ZwlrGammaControlV1GammaSizeEvent};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches gamma control events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrGammaControlV1GammaSizeEvent].
    ZwlrGammaControlV1GammaSize(ZwlrGammaControlV1GammaSizeEvent),
    /// Message [ZwlrGammaControlV1FailedEvent].
    ZwlrGammaControlV1Failed(ZwlrGammaControlV1FailedEvent),
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
            ObjectInteface::WlrGammaControl(
                WlrGammaControlObjectInteface::ZwlrGammaControlV1 { .. },
            ) => match opcode {
                ZwlrGammaControlV1GammaSizeEvent::OPCODE => Ok(Self::ZwlrGammaControlV1GammaSize(
                    ZwlrGammaControlV1GammaSizeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrGammaControlV1FailedEvent::OPCODE => Ok(Self::ZwlrGammaControlV1Failed(
                    ZwlrGammaControlV1FailedEvent::read_with_interface(
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
            Self::ZwlrGammaControlV1GammaSize(m) => m.object_id(),
            Self::ZwlrGammaControlV1Failed(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrGammaControlV1GammaSize(m) => m.opcode(),
            Self::ZwlrGammaControlV1Failed(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrGammaControlV1GammaSize(m) => m.payload_size(),
            Self::ZwlrGammaControlV1Failed(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrGammaControlV1GammaSize(m) => m.write(writer, state).await,
            Self::ZwlrGammaControlV1Failed(m) => m.write(writer, state).await,
        }
    }
}
