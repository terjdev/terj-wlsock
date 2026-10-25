use super::ObjectInteface as WlrExportDmabufObjectInteface;
use super::{
    ZwlrExportDmabufFrameV1CancelEvent, ZwlrExportDmabufFrameV1FrameEvent,
    ZwlrExportDmabufFrameV1ObjectEvent, ZwlrExportDmabufFrameV1ReadyEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches export dmabuf events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrExportDmabufFrameV1FrameEvent].
    ZwlrExportDmabufFrameV1Frame(ZwlrExportDmabufFrameV1FrameEvent),
    /// Message [ZwlrExportDmabufFrameV1ObjectEvent].
    ZwlrExportDmabufFrameV1Object(ZwlrExportDmabufFrameV1ObjectEvent),
    /// Message [ZwlrExportDmabufFrameV1ReadyEvent].
    ZwlrExportDmabufFrameV1Ready(ZwlrExportDmabufFrameV1ReadyEvent),
    /// Message [ZwlrExportDmabufFrameV1CancelEvent].
    ZwlrExportDmabufFrameV1Cancel(ZwlrExportDmabufFrameV1CancelEvent),
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
            ObjectInteface::WlrExportDmabuf(
                WlrExportDmabufObjectInteface::ZwlrExportDmabufFrameV1 { .. },
            ) => match opcode {
                ZwlrExportDmabufFrameV1FrameEvent::OPCODE => {
                    Ok(Self::ZwlrExportDmabufFrameV1Frame(
                        ZwlrExportDmabufFrameV1FrameEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrExportDmabufFrameV1ObjectEvent::OPCODE => {
                    Ok(Self::ZwlrExportDmabufFrameV1Object(
                        ZwlrExportDmabufFrameV1ObjectEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrExportDmabufFrameV1ReadyEvent::OPCODE => {
                    Ok(Self::ZwlrExportDmabufFrameV1Ready(
                        ZwlrExportDmabufFrameV1ReadyEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrExportDmabufFrameV1CancelEvent::OPCODE => {
                    Ok(Self::ZwlrExportDmabufFrameV1Cancel(
                        ZwlrExportDmabufFrameV1CancelEvent::read_with_interface(
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
            Self::ZwlrExportDmabufFrameV1Frame(m) => m.object_id(),
            Self::ZwlrExportDmabufFrameV1Object(m) => m.object_id(),
            Self::ZwlrExportDmabufFrameV1Ready(m) => m.object_id(),
            Self::ZwlrExportDmabufFrameV1Cancel(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrExportDmabufFrameV1Frame(m) => m.opcode(),
            Self::ZwlrExportDmabufFrameV1Object(m) => m.opcode(),
            Self::ZwlrExportDmabufFrameV1Ready(m) => m.opcode(),
            Self::ZwlrExportDmabufFrameV1Cancel(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrExportDmabufFrameV1Frame(m) => m.payload_size(),
            Self::ZwlrExportDmabufFrameV1Object(m) => m.payload_size(),
            Self::ZwlrExportDmabufFrameV1Ready(m) => m.payload_size(),
            Self::ZwlrExportDmabufFrameV1Cancel(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrExportDmabufFrameV1Frame(m) => m.write(writer, state).await,
            Self::ZwlrExportDmabufFrameV1Object(m) => m.write(writer, state).await,
            Self::ZwlrExportDmabufFrameV1Ready(m) => m.write(writer, state).await,
            Self::ZwlrExportDmabufFrameV1Cancel(m) => m.write(writer, state).await,
        }
    }
}
