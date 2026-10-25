use super::ObjectInteface as TextInputObjectInteface;
use super::{
    ZwpTextInputV1CommitStringEvent, ZwpTextInputV1CursorPositionEvent,
    ZwpTextInputV1DeleteSurroundingTextEvent, ZwpTextInputV1EnterEvent,
    ZwpTextInputV1InputPanelStateEvent, ZwpTextInputV1KeysymEvent, ZwpTextInputV1LanguageEvent,
    ZwpTextInputV1LeaveEvent, ZwpTextInputV1ModifiersMapEvent, ZwpTextInputV1PreeditCursorEvent,
    ZwpTextInputV1PreeditStringEvent, ZwpTextInputV1PreeditStylingEvent,
    ZwpTextInputV1TextDirectionEvent, ZwpTextInputV3ActionEvent, ZwpTextInputV3CommitStringEvent,
    ZwpTextInputV3DeleteSurroundingTextEvent, ZwpTextInputV3DoneEvent, ZwpTextInputV3EnterEvent,
    ZwpTextInputV3LanguageEvent, ZwpTextInputV3LeaveEvent, ZwpTextInputV3PreeditHintEvent,
    ZwpTextInputV3PreeditStringEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches text input events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpTextInputV1EnterEvent].
    ZwpTextInputV1Enter(ZwpTextInputV1EnterEvent),
    /// Message [ZwpTextInputV1LeaveEvent].
    ZwpTextInputV1Leave(ZwpTextInputV1LeaveEvent),
    /// Message [ZwpTextInputV1ModifiersMapEvent].
    ZwpTextInputV1ModifiersMap(ZwpTextInputV1ModifiersMapEvent),
    /// Message [ZwpTextInputV1InputPanelStateEvent].
    ZwpTextInputV1InputPanelState(ZwpTextInputV1InputPanelStateEvent),
    /// Message [ZwpTextInputV1PreeditStringEvent].
    ZwpTextInputV1PreeditString(ZwpTextInputV1PreeditStringEvent),
    /// Message [ZwpTextInputV1PreeditStylingEvent].
    ZwpTextInputV1PreeditStyling(ZwpTextInputV1PreeditStylingEvent),
    /// Message [ZwpTextInputV1PreeditCursorEvent].
    ZwpTextInputV1PreeditCursor(ZwpTextInputV1PreeditCursorEvent),
    /// Message [ZwpTextInputV1CommitStringEvent].
    ZwpTextInputV1CommitString(ZwpTextInputV1CommitStringEvent),
    /// Message [ZwpTextInputV1CursorPositionEvent].
    ZwpTextInputV1CursorPosition(ZwpTextInputV1CursorPositionEvent),
    /// Message [ZwpTextInputV1DeleteSurroundingTextEvent].
    ZwpTextInputV1DeleteSurroundingText(ZwpTextInputV1DeleteSurroundingTextEvent),
    /// Message [ZwpTextInputV1KeysymEvent].
    ZwpTextInputV1Keysym(ZwpTextInputV1KeysymEvent),
    /// Message [ZwpTextInputV1LanguageEvent].
    ZwpTextInputV1Language(ZwpTextInputV1LanguageEvent),
    /// Message [ZwpTextInputV1TextDirectionEvent].
    ZwpTextInputV1TextDirection(ZwpTextInputV1TextDirectionEvent),
    /// Message [ZwpTextInputV3EnterEvent].
    ZwpTextInputV3Enter(ZwpTextInputV3EnterEvent),
    /// Message [ZwpTextInputV3LeaveEvent].
    ZwpTextInputV3Leave(ZwpTextInputV3LeaveEvent),
    /// Message [ZwpTextInputV3PreeditStringEvent].
    ZwpTextInputV3PreeditString(ZwpTextInputV3PreeditStringEvent),
    /// Message [ZwpTextInputV3CommitStringEvent].
    ZwpTextInputV3CommitString(ZwpTextInputV3CommitStringEvent),
    /// Message [ZwpTextInputV3DeleteSurroundingTextEvent].
    ZwpTextInputV3DeleteSurroundingText(ZwpTextInputV3DeleteSurroundingTextEvent),
    /// Message [ZwpTextInputV3DoneEvent].
    ZwpTextInputV3Done(ZwpTextInputV3DoneEvent),
    /// Message [ZwpTextInputV3ActionEvent].
    ZwpTextInputV3Action(ZwpTextInputV3ActionEvent),
    /// Message [ZwpTextInputV3LanguageEvent].
    ZwpTextInputV3Language(ZwpTextInputV3LanguageEvent),
    /// Message [ZwpTextInputV3PreeditHintEvent].
    ZwpTextInputV3PreeditHint(ZwpTextInputV3PreeditHintEvent),
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
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV1 { .. }) => {
                match opcode {
                    ZwpTextInputV1EnterEvent::OPCODE => Ok(Self::ZwpTextInputV1Enter(
                        ZwpTextInputV1EnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1LeaveEvent::OPCODE => Ok(Self::ZwpTextInputV1Leave(
                        ZwpTextInputV1LeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1ModifiersMapEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1ModifiersMap(
                            ZwpTextInputV1ModifiersMapEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1InputPanelStateEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1InputPanelState(
                            ZwpTextInputV1InputPanelStateEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1PreeditStringEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1PreeditString(
                            ZwpTextInputV1PreeditStringEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1PreeditStylingEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1PreeditStyling(
                            ZwpTextInputV1PreeditStylingEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1PreeditCursorEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1PreeditCursor(
                            ZwpTextInputV1PreeditCursorEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1CommitStringEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1CommitString(
                            ZwpTextInputV1CommitStringEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1CursorPositionEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1CursorPosition(
                            ZwpTextInputV1CursorPositionEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1DeleteSurroundingTextEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1DeleteSurroundingText(
                            ZwpTextInputV1DeleteSurroundingTextEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1KeysymEvent::OPCODE => Ok(Self::ZwpTextInputV1Keysym(
                        ZwpTextInputV1KeysymEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1LanguageEvent::OPCODE => Ok(Self::ZwpTextInputV1Language(
                        ZwpTextInputV1LanguageEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1TextDirectionEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV1TextDirection(
                            ZwpTextInputV1TextDirectionEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. }) => {
                match opcode {
                    ZwpTextInputV3EnterEvent::OPCODE => Ok(Self::ZwpTextInputV3Enter(
                        ZwpTextInputV3EnterEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3LeaveEvent::OPCODE => Ok(Self::ZwpTextInputV3Leave(
                        ZwpTextInputV3LeaveEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3PreeditStringEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV3PreeditString(
                            ZwpTextInputV3PreeditStringEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3CommitStringEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV3CommitString(
                            ZwpTextInputV3CommitStringEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3DeleteSurroundingTextEvent::OPCODE => {
                        Ok(Self::ZwpTextInputV3DeleteSurroundingText(
                            ZwpTextInputV3DeleteSurroundingTextEvent::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3DoneEvent::OPCODE => Ok(Self::ZwpTextInputV3Done(
                        ZwpTextInputV3DoneEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3ActionEvent::OPCODE => Ok(Self::ZwpTextInputV3Action(
                        ZwpTextInputV3ActionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3LanguageEvent::OPCODE => Ok(Self::ZwpTextInputV3Language(
                        ZwpTextInputV3LanguageEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3PreeditHintEvent::OPCODE => Ok(Self::ZwpTextInputV3PreeditHint(
                        ZwpTextInputV3PreeditHintEvent::read_with_interface(
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
            Self::ZwpTextInputV1Enter(m) => m.object_id(),
            Self::ZwpTextInputV1Leave(m) => m.object_id(),
            Self::ZwpTextInputV1ModifiersMap(m) => m.object_id(),
            Self::ZwpTextInputV1InputPanelState(m) => m.object_id(),
            Self::ZwpTextInputV1PreeditString(m) => m.object_id(),
            Self::ZwpTextInputV1PreeditStyling(m) => m.object_id(),
            Self::ZwpTextInputV1PreeditCursor(m) => m.object_id(),
            Self::ZwpTextInputV1CommitString(m) => m.object_id(),
            Self::ZwpTextInputV1CursorPosition(m) => m.object_id(),
            Self::ZwpTextInputV1DeleteSurroundingText(m) => m.object_id(),
            Self::ZwpTextInputV1Keysym(m) => m.object_id(),
            Self::ZwpTextInputV1Language(m) => m.object_id(),
            Self::ZwpTextInputV1TextDirection(m) => m.object_id(),
            Self::ZwpTextInputV3Enter(m) => m.object_id(),
            Self::ZwpTextInputV3Leave(m) => m.object_id(),
            Self::ZwpTextInputV3PreeditString(m) => m.object_id(),
            Self::ZwpTextInputV3CommitString(m) => m.object_id(),
            Self::ZwpTextInputV3DeleteSurroundingText(m) => m.object_id(),
            Self::ZwpTextInputV3Done(m) => m.object_id(),
            Self::ZwpTextInputV3Action(m) => m.object_id(),
            Self::ZwpTextInputV3Language(m) => m.object_id(),
            Self::ZwpTextInputV3PreeditHint(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTextInputV1Enter(m) => m.opcode(),
            Self::ZwpTextInputV1Leave(m) => m.opcode(),
            Self::ZwpTextInputV1ModifiersMap(m) => m.opcode(),
            Self::ZwpTextInputV1InputPanelState(m) => m.opcode(),
            Self::ZwpTextInputV1PreeditString(m) => m.opcode(),
            Self::ZwpTextInputV1PreeditStyling(m) => m.opcode(),
            Self::ZwpTextInputV1PreeditCursor(m) => m.opcode(),
            Self::ZwpTextInputV1CommitString(m) => m.opcode(),
            Self::ZwpTextInputV1CursorPosition(m) => m.opcode(),
            Self::ZwpTextInputV1DeleteSurroundingText(m) => m.opcode(),
            Self::ZwpTextInputV1Keysym(m) => m.opcode(),
            Self::ZwpTextInputV1Language(m) => m.opcode(),
            Self::ZwpTextInputV1TextDirection(m) => m.opcode(),
            Self::ZwpTextInputV3Enter(m) => m.opcode(),
            Self::ZwpTextInputV3Leave(m) => m.opcode(),
            Self::ZwpTextInputV3PreeditString(m) => m.opcode(),
            Self::ZwpTextInputV3CommitString(m) => m.opcode(),
            Self::ZwpTextInputV3DeleteSurroundingText(m) => m.opcode(),
            Self::ZwpTextInputV3Done(m) => m.opcode(),
            Self::ZwpTextInputV3Action(m) => m.opcode(),
            Self::ZwpTextInputV3Language(m) => m.opcode(),
            Self::ZwpTextInputV3PreeditHint(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTextInputV1Enter(m) => m.payload_size(),
            Self::ZwpTextInputV1Leave(m) => m.payload_size(),
            Self::ZwpTextInputV1ModifiersMap(m) => m.payload_size(),
            Self::ZwpTextInputV1InputPanelState(m) => m.payload_size(),
            Self::ZwpTextInputV1PreeditString(m) => m.payload_size(),
            Self::ZwpTextInputV1PreeditStyling(m) => m.payload_size(),
            Self::ZwpTextInputV1PreeditCursor(m) => m.payload_size(),
            Self::ZwpTextInputV1CommitString(m) => m.payload_size(),
            Self::ZwpTextInputV1CursorPosition(m) => m.payload_size(),
            Self::ZwpTextInputV1DeleteSurroundingText(m) => m.payload_size(),
            Self::ZwpTextInputV1Keysym(m) => m.payload_size(),
            Self::ZwpTextInputV1Language(m) => m.payload_size(),
            Self::ZwpTextInputV1TextDirection(m) => m.payload_size(),
            Self::ZwpTextInputV3Enter(m) => m.payload_size(),
            Self::ZwpTextInputV3Leave(m) => m.payload_size(),
            Self::ZwpTextInputV3PreeditString(m) => m.payload_size(),
            Self::ZwpTextInputV3CommitString(m) => m.payload_size(),
            Self::ZwpTextInputV3DeleteSurroundingText(m) => m.payload_size(),
            Self::ZwpTextInputV3Done(m) => m.payload_size(),
            Self::ZwpTextInputV3Action(m) => m.payload_size(),
            Self::ZwpTextInputV3Language(m) => m.payload_size(),
            Self::ZwpTextInputV3PreeditHint(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTextInputV1Enter(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1Leave(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1ModifiersMap(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1InputPanelState(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1PreeditString(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1PreeditStyling(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1PreeditCursor(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1CommitString(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1CursorPosition(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1DeleteSurroundingText(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1Keysym(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1Language(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1TextDirection(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Enter(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Leave(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3PreeditString(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3CommitString(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3DeleteSurroundingText(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Done(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Action(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Language(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3PreeditHint(m) => m.write(writer, state).await,
        }
    }
}
