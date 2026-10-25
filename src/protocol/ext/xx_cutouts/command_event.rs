use super::ObjectInteface as XxCutoutsObjectInteface;
use super::{XxCutoutsV1ConfigureEvent, XxCutoutsV1CutoutBoxEvent, XxCutoutsV1CutoutCornerEvent};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches cutouts events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxCutoutsV1CutoutBoxEvent].
    XxCutoutsV1CutoutBox(XxCutoutsV1CutoutBoxEvent),
    /// Message [XxCutoutsV1CutoutCornerEvent].
    XxCutoutsV1CutoutCorner(XxCutoutsV1CutoutCornerEvent),
    /// Message [XxCutoutsV1ConfigureEvent].
    XxCutoutsV1Configure(XxCutoutsV1ConfigureEvent),
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
            ObjectInteface::XxCutouts(XxCutoutsObjectInteface::XxCutoutsV1 { .. }) => {
                match opcode {
                    XxCutoutsV1CutoutBoxEvent::OPCODE => Ok(Self::XxCutoutsV1CutoutBox(
                        XxCutoutsV1CutoutBoxEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxCutoutsV1CutoutCornerEvent::OPCODE => Ok(Self::XxCutoutsV1CutoutCorner(
                        XxCutoutsV1CutoutCornerEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxCutoutsV1ConfigureEvent::OPCODE => Ok(Self::XxCutoutsV1Configure(
                        XxCutoutsV1ConfigureEvent::read_with_interface(
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
            Self::XxCutoutsV1CutoutBox(m) => m.object_id(),
            Self::XxCutoutsV1CutoutCorner(m) => m.object_id(),
            Self::XxCutoutsV1Configure(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxCutoutsV1CutoutBox(m) => m.opcode(),
            Self::XxCutoutsV1CutoutCorner(m) => m.opcode(),
            Self::XxCutoutsV1Configure(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxCutoutsV1CutoutBox(m) => m.payload_size(),
            Self::XxCutoutsV1CutoutCorner(m) => m.payload_size(),
            Self::XxCutoutsV1Configure(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxCutoutsV1CutoutBox(m) => m.write(writer, state).await,
            Self::XxCutoutsV1CutoutCorner(m) => m.write(writer, state).await,
            Self::XxCutoutsV1Configure(m) => m.write(writer, state).await,
        }
    }
}
