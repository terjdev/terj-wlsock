use super::{
    ZwlrScreencopyFrameV1BufferDoneEvent, ZwlrScreencopyFrameV1BufferEvent,
    ZwlrScreencopyFrameV1DamageEvent, ZwlrScreencopyFrameV1FailedEvent,
    ZwlrScreencopyFrameV1FlagsEvent, ZwlrScreencopyFrameV1LinuxDmabufEvent,
    ZwlrScreencopyFrameV1ReadyEvent,
};
use crate::protocol::wlr::screencopy::ObjectInteface as WlrScreencopyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR screencopy events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrScreencopyFrameV1BufferEvent].
    ZwlrScreencopyFrameV1Buffer(ZwlrScreencopyFrameV1BufferEvent),
    /// Message [ZwlrScreencopyFrameV1FlagsEvent].
    ZwlrScreencopyFrameV1Flags(ZwlrScreencopyFrameV1FlagsEvent),
    /// Message [ZwlrScreencopyFrameV1ReadyEvent].
    ZwlrScreencopyFrameV1Ready(ZwlrScreencopyFrameV1ReadyEvent),
    /// Message [ZwlrScreencopyFrameV1FailedEvent].
    ZwlrScreencopyFrameV1Failed(ZwlrScreencopyFrameV1FailedEvent),
    /// Message [ZwlrScreencopyFrameV1DamageEvent].
    ZwlrScreencopyFrameV1Damage(ZwlrScreencopyFrameV1DamageEvent),
    /// Message [ZwlrScreencopyFrameV1LinuxDmabufEvent].
    ZwlrScreencopyFrameV1LinuxDmabuf(ZwlrScreencopyFrameV1LinuxDmabufEvent),
    /// Message [ZwlrScreencopyFrameV1BufferDoneEvent].
    ZwlrScreencopyFrameV1BufferDone(ZwlrScreencopyFrameV1BufferDoneEvent),
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
            ObjectInteface::WlrScreencopy(WlrScreencopyObjectInteface::ZwlrScreencopyFrameV1 {
                ..
            }) => match opcode {
                ZwlrScreencopyFrameV1BufferEvent::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Buffer(
                    ZwlrScreencopyFrameV1BufferEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1FlagsEvent::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Flags(
                    ZwlrScreencopyFrameV1FlagsEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1ReadyEvent::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Ready(
                    ZwlrScreencopyFrameV1ReadyEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1FailedEvent::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Failed(
                    ZwlrScreencopyFrameV1FailedEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1DamageEvent::OPCODE => Ok(Self::ZwlrScreencopyFrameV1Damage(
                    ZwlrScreencopyFrameV1DamageEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrScreencopyFrameV1LinuxDmabufEvent::OPCODE => {
                    Ok(Self::ZwlrScreencopyFrameV1LinuxDmabuf(
                        ZwlrScreencopyFrameV1LinuxDmabufEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrScreencopyFrameV1BufferDoneEvent::OPCODE => {
                    Ok(Self::ZwlrScreencopyFrameV1BufferDone(
                        ZwlrScreencopyFrameV1BufferDoneEvent::read_with_interface(
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
            Self::ZwlrScreencopyFrameV1Buffer(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Flags(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Ready(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Failed(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1Damage(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1LinuxDmabuf(message) => message.object_id(),
            Self::ZwlrScreencopyFrameV1BufferDone(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrScreencopyFrameV1Buffer(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Flags(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Ready(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Failed(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1Damage(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1LinuxDmabuf(message) => message.opcode(),
            Self::ZwlrScreencopyFrameV1BufferDone(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrScreencopyFrameV1Buffer(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Flags(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Ready(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Failed(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1Damage(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1LinuxDmabuf(message) => message.payload_size(),
            Self::ZwlrScreencopyFrameV1BufferDone(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrScreencopyFrameV1Buffer(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Flags(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Ready(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Failed(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1Damage(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1LinuxDmabuf(message) => message.write(writer, state).await,
            Self::ZwlrScreencopyFrameV1BufferDone(message) => message.write(writer, state).await,
        }
    }
}
