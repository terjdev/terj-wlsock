use super::ObjectInteface as LinuxExplicitSynchronizationObjectInteface;
use super::{
    ZwpLinuxBufferReleaseV1FencedReleaseEvent, ZwpLinuxBufferReleaseV1ImmediateReleaseEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches linux explicit synchronization events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpLinuxBufferReleaseV1FencedReleaseEvent].
    ZwpLinuxBufferReleaseV1FencedRelease(ZwpLinuxBufferReleaseV1FencedReleaseEvent),
    /// Message [ZwpLinuxBufferReleaseV1ImmediateReleaseEvent].
    ZwpLinuxBufferReleaseV1ImmediateRelease(ZwpLinuxBufferReleaseV1ImmediateReleaseEvent),
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
            ObjectInteface::LinuxExplicitSynchronization(
                LinuxExplicitSynchronizationObjectInteface::ZwpLinuxBufferReleaseV1 { .. },
            ) => match opcode {
                ZwpLinuxBufferReleaseV1FencedReleaseEvent::OPCODE => {
                    Ok(Self::ZwpLinuxBufferReleaseV1FencedRelease(
                        ZwpLinuxBufferReleaseV1FencedReleaseEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpLinuxBufferReleaseV1ImmediateReleaseEvent::OPCODE => {
                    Ok(Self::ZwpLinuxBufferReleaseV1ImmediateRelease(
                        ZwpLinuxBufferReleaseV1ImmediateReleaseEvent::read_with_interface(
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
            Self::ZwpLinuxBufferReleaseV1FencedRelease(m) => m.object_id(),
            Self::ZwpLinuxBufferReleaseV1ImmediateRelease(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpLinuxBufferReleaseV1FencedRelease(m) => m.opcode(),
            Self::ZwpLinuxBufferReleaseV1ImmediateRelease(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpLinuxBufferReleaseV1FencedRelease(m) => m.payload_size(),
            Self::ZwpLinuxBufferReleaseV1ImmediateRelease(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpLinuxBufferReleaseV1FencedRelease(m) => m.write(writer, state).await,
            Self::ZwpLinuxBufferReleaseV1ImmediateRelease(m) => m.write(writer, state).await,
        }
    }
}
