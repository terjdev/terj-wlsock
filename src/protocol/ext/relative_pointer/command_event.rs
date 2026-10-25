use super::ObjectInteface as RelativePointerObjectInteface;
use super::ZwpRelativePointerV1RelativeMotionEvent;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches relative pointer events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpRelativePointerV1RelativeMotionEvent].
    ZwpRelativePointerV1RelativeMotion(ZwpRelativePointerV1RelativeMotionEvent),
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
            ObjectInteface::RelativePointer(
                RelativePointerObjectInteface::ZwpRelativePointerV1 { .. },
            ) => match opcode {
                ZwpRelativePointerV1RelativeMotionEvent::OPCODE => {
                    Ok(Self::ZwpRelativePointerV1RelativeMotion(
                        ZwpRelativePointerV1RelativeMotionEvent::read_with_interface(
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
            Self::ZwpRelativePointerV1RelativeMotion(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpRelativePointerV1RelativeMotion(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpRelativePointerV1RelativeMotion(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpRelativePointerV1RelativeMotion(m) => m.write(writer, state).await,
        }
    }
}
