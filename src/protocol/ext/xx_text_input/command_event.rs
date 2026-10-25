use super::ObjectInteface as XxTextInputObjectInteface;
use super::{
    XxTextInputV3CommitStringEvent, XxTextInputV3DeleteSurroundingTextEvent,
    XxTextInputV3DoneEvent, XxTextInputV3EnterEvent, XxTextInputV3LeaveEvent,
    XxTextInputV3MoveCursorEvent, XxTextInputV3PerformActionEvent, XxTextInputV3PreeditStringEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xx text input events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxTextInputV3EnterEvent].
    XxTextInputV3Enter(XxTextInputV3EnterEvent),
    /// Message [XxTextInputV3LeaveEvent].
    XxTextInputV3Leave(XxTextInputV3LeaveEvent),
    /// Message [XxTextInputV3PreeditStringEvent].
    XxTextInputV3PreeditString(XxTextInputV3PreeditStringEvent),
    /// Message [XxTextInputV3CommitStringEvent].
    XxTextInputV3CommitString(XxTextInputV3CommitStringEvent),
    /// Message [XxTextInputV3DeleteSurroundingTextEvent].
    XxTextInputV3DeleteSurroundingText(XxTextInputV3DeleteSurroundingTextEvent),
    /// Message [XxTextInputV3MoveCursorEvent].
    XxTextInputV3MoveCursor(XxTextInputV3MoveCursorEvent),
    /// Message [XxTextInputV3DoneEvent].
    XxTextInputV3Done(XxTextInputV3DoneEvent),
    /// Message [XxTextInputV3PerformActionEvent].
    XxTextInputV3PerformAction(XxTextInputV3PerformActionEvent),
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
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. }) => {
                match opcode {
                    XxTextInputV3EnterEvent::OPCODE => Ok(Self::XxTextInputV3Enter(
                        XxTextInputV3EnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3LeaveEvent::OPCODE => Ok(Self::XxTextInputV3Leave(
                        XxTextInputV3LeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3PreeditStringEvent::OPCODE => {
                        Ok(Self::XxTextInputV3PreeditString(
                            XxTextInputV3PreeditStringEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3CommitStringEvent::OPCODE => Ok(Self::XxTextInputV3CommitString(
                        XxTextInputV3CommitStringEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3DeleteSurroundingTextEvent::OPCODE => {
                        Ok(Self::XxTextInputV3DeleteSurroundingText(
                            XxTextInputV3DeleteSurroundingTextEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3MoveCursorEvent::OPCODE => Ok(Self::XxTextInputV3MoveCursor(
                        XxTextInputV3MoveCursorEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3DoneEvent::OPCODE => Ok(Self::XxTextInputV3Done(
                        XxTextInputV3DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3PerformActionEvent::OPCODE => {
                        Ok(Self::XxTextInputV3PerformAction(
                            XxTextInputV3PerformActionEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
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
            Self::XxTextInputV3Enter(m) => m.object_id(),
            Self::XxTextInputV3Leave(m) => m.object_id(),
            Self::XxTextInputV3PreeditString(m) => m.object_id(),
            Self::XxTextInputV3CommitString(m) => m.object_id(),
            Self::XxTextInputV3DeleteSurroundingText(m) => m.object_id(),
            Self::XxTextInputV3MoveCursor(m) => m.object_id(),
            Self::XxTextInputV3Done(m) => m.object_id(),
            Self::XxTextInputV3PerformAction(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxTextInputV3Enter(m) => m.opcode(),
            Self::XxTextInputV3Leave(m) => m.opcode(),
            Self::XxTextInputV3PreeditString(m) => m.opcode(),
            Self::XxTextInputV3CommitString(m) => m.opcode(),
            Self::XxTextInputV3DeleteSurroundingText(m) => m.opcode(),
            Self::XxTextInputV3MoveCursor(m) => m.opcode(),
            Self::XxTextInputV3Done(m) => m.opcode(),
            Self::XxTextInputV3PerformAction(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxTextInputV3Enter(m) => m.payload_size(),
            Self::XxTextInputV3Leave(m) => m.payload_size(),
            Self::XxTextInputV3PreeditString(m) => m.payload_size(),
            Self::XxTextInputV3CommitString(m) => m.payload_size(),
            Self::XxTextInputV3DeleteSurroundingText(m) => m.payload_size(),
            Self::XxTextInputV3MoveCursor(m) => m.payload_size(),
            Self::XxTextInputV3Done(m) => m.payload_size(),
            Self::XxTextInputV3PerformAction(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxTextInputV3Enter(m) => m.write(writer, state).await,
            Self::XxTextInputV3Leave(m) => m.write(writer, state).await,
            Self::XxTextInputV3PreeditString(m) => m.write(writer, state).await,
            Self::XxTextInputV3CommitString(m) => m.write(writer, state).await,
            Self::XxTextInputV3DeleteSurroundingText(m) => m.write(writer, state).await,
            Self::XxTextInputV3MoveCursor(m) => m.write(writer, state).await,
            Self::XxTextInputV3Done(m) => m.write(writer, state).await,
            Self::XxTextInputV3PerformAction(m) => m.write(writer, state).await,
        }
    }
}
