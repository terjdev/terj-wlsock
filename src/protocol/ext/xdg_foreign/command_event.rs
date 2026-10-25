use super::ObjectInteface as XdgForeignObjectInteface;
use super::{
    ZxdgExportedV1HandleEvent, ZxdgExportedV2HandleEvent, ZxdgImportedV1DestroyedEvent,
    ZxdgImportedV2DestroyedEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xdg foreign events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZxdgExportedV1HandleEvent].
    ZxdgExportedV1Handle(ZxdgExportedV1HandleEvent),
    /// Message [ZxdgImportedV1DestroyedEvent].
    ZxdgImportedV1Destroyed(ZxdgImportedV1DestroyedEvent),
    /// Message [ZxdgExportedV2HandleEvent].
    ZxdgExportedV2Handle(ZxdgExportedV2HandleEvent),
    /// Message [ZxdgImportedV2DestroyedEvent].
    ZxdgImportedV2Destroyed(ZxdgImportedV2DestroyedEvent),
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
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV1 { .. }) => {
                match opcode {
                    ZxdgExportedV1HandleEvent::OPCODE => Ok(Self::ZxdgExportedV1Handle(
                        ZxdgExportedV1HandleEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV1 { .. }) => {
                match opcode {
                    ZxdgImportedV1DestroyedEvent::OPCODE => Ok(Self::ZxdgImportedV1Destroyed(
                        ZxdgImportedV1DestroyedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgExportedV2 { .. }) => {
                match opcode {
                    ZxdgExportedV2HandleEvent::OPCODE => Ok(Self::ZxdgExportedV2Handle(
                        ZxdgExportedV2HandleEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XdgForeign(XdgForeignObjectInteface::ZxdgImportedV2 { .. }) => {
                match opcode {
                    ZxdgImportedV2DestroyedEvent::OPCODE => Ok(Self::ZxdgImportedV2Destroyed(
                        ZxdgImportedV2DestroyedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZxdgExportedV1Handle(m) => m.object_id(),
            Self::ZxdgImportedV1Destroyed(m) => m.object_id(),
            Self::ZxdgExportedV2Handle(m) => m.object_id(),
            Self::ZxdgImportedV2Destroyed(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZxdgExportedV1Handle(m) => m.opcode(),
            Self::ZxdgImportedV1Destroyed(m) => m.opcode(),
            Self::ZxdgExportedV2Handle(m) => m.opcode(),
            Self::ZxdgImportedV2Destroyed(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZxdgExportedV1Handle(m) => m.payload_size(),
            Self::ZxdgImportedV1Destroyed(m) => m.payload_size(),
            Self::ZxdgExportedV2Handle(m) => m.payload_size(),
            Self::ZxdgImportedV2Destroyed(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZxdgExportedV1Handle(m) => m.write(writer, state).await,
            Self::ZxdgImportedV1Destroyed(m) => m.write(writer, state).await,
            Self::ZxdgExportedV2Handle(m) => m.write(writer, state).await,
            Self::ZxdgImportedV2Destroyed(m) => m.write(writer, state).await,
        }
    }
}
