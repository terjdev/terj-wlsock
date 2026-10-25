use super::ObjectInteface as TextInputObjectInteface;
use super::{
    ZwpTextInputManagerV1CreateTextInputRequest, ZwpTextInputManagerV3DestroyRequest,
    ZwpTextInputManagerV3GetTextInputRequest, ZwpTextInputV1ActivateRequest,
    ZwpTextInputV1CommitStateRequest, ZwpTextInputV1DeactivateRequest,
    ZwpTextInputV1HideInputPanelRequest, ZwpTextInputV1InvokeActionRequest,
    ZwpTextInputV1ResetRequest, ZwpTextInputV1SetContentTypeRequest,
    ZwpTextInputV1SetCursorRectangleRequest, ZwpTextInputV1SetPreferredLanguageRequest,
    ZwpTextInputV1SetSurroundingTextRequest, ZwpTextInputV1ShowInputPanelRequest,
    ZwpTextInputV3CommitRequest, ZwpTextInputV3DestroyRequest, ZwpTextInputV3DisableRequest,
    ZwpTextInputV3EnableRequest, ZwpTextInputV3HideInputPanelRequest,
    ZwpTextInputV3SetAvailableActionsRequest, ZwpTextInputV3SetContentTypeRequest,
    ZwpTextInputV3SetCursorRectangleRequest, ZwpTextInputV3SetSurroundingTextRequest,
    ZwpTextInputV3SetTextChangeCauseRequest, ZwpTextInputV3ShowInputPanelRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches text input requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpTextInputV1ActivateRequest].
    ZwpTextInputV1Activate(ZwpTextInputV1ActivateRequest),
    /// Message [ZwpTextInputV1DeactivateRequest].
    ZwpTextInputV1Deactivate(ZwpTextInputV1DeactivateRequest),
    /// Message [ZwpTextInputV1ShowInputPanelRequest].
    ZwpTextInputV1ShowInputPanel(ZwpTextInputV1ShowInputPanelRequest),
    /// Message [ZwpTextInputV1HideInputPanelRequest].
    ZwpTextInputV1HideInputPanel(ZwpTextInputV1HideInputPanelRequest),
    /// Message [ZwpTextInputV1ResetRequest].
    ZwpTextInputV1Reset(ZwpTextInputV1ResetRequest),
    /// Message [ZwpTextInputV1SetSurroundingTextRequest].
    ZwpTextInputV1SetSurroundingText(ZwpTextInputV1SetSurroundingTextRequest),
    /// Message [ZwpTextInputV1SetContentTypeRequest].
    ZwpTextInputV1SetContentType(ZwpTextInputV1SetContentTypeRequest),
    /// Message [ZwpTextInputV1SetCursorRectangleRequest].
    ZwpTextInputV1SetCursorRectangle(ZwpTextInputV1SetCursorRectangleRequest),
    /// Message [ZwpTextInputV1SetPreferredLanguageRequest].
    ZwpTextInputV1SetPreferredLanguage(ZwpTextInputV1SetPreferredLanguageRequest),
    /// Message [ZwpTextInputV1CommitStateRequest].
    ZwpTextInputV1CommitState(ZwpTextInputV1CommitStateRequest),
    /// Message [ZwpTextInputV1InvokeActionRequest].
    ZwpTextInputV1InvokeAction(ZwpTextInputV1InvokeActionRequest),
    /// Message [ZwpTextInputManagerV1CreateTextInputRequest].
    ZwpTextInputManagerV1CreateTextInput(ZwpTextInputManagerV1CreateTextInputRequest),
    /// Message [ZwpTextInputV3DestroyRequest].
    ZwpTextInputV3Destroy(ZwpTextInputV3DestroyRequest),
    /// Message [ZwpTextInputV3EnableRequest].
    ZwpTextInputV3Enable(ZwpTextInputV3EnableRequest),
    /// Message [ZwpTextInputV3DisableRequest].
    ZwpTextInputV3Disable(ZwpTextInputV3DisableRequest),
    /// Message [ZwpTextInputV3SetSurroundingTextRequest].
    ZwpTextInputV3SetSurroundingText(ZwpTextInputV3SetSurroundingTextRequest),
    /// Message [ZwpTextInputV3SetTextChangeCauseRequest].
    ZwpTextInputV3SetTextChangeCause(ZwpTextInputV3SetTextChangeCauseRequest),
    /// Message [ZwpTextInputV3SetContentTypeRequest].
    ZwpTextInputV3SetContentType(ZwpTextInputV3SetContentTypeRequest),
    /// Message [ZwpTextInputV3SetCursorRectangleRequest].
    ZwpTextInputV3SetCursorRectangle(ZwpTextInputV3SetCursorRectangleRequest),
    /// Message [ZwpTextInputV3CommitRequest].
    ZwpTextInputV3Commit(ZwpTextInputV3CommitRequest),
    /// Message [ZwpTextInputV3SetAvailableActionsRequest].
    ZwpTextInputV3SetAvailableActions(ZwpTextInputV3SetAvailableActionsRequest),
    /// Message [ZwpTextInputV3ShowInputPanelRequest].
    ZwpTextInputV3ShowInputPanel(ZwpTextInputV3ShowInputPanelRequest),
    /// Message [ZwpTextInputV3HideInputPanelRequest].
    ZwpTextInputV3HideInputPanel(ZwpTextInputV3HideInputPanelRequest),
    /// Message [ZwpTextInputManagerV3DestroyRequest].
    ZwpTextInputManagerV3Destroy(ZwpTextInputManagerV3DestroyRequest),
    /// Message [ZwpTextInputManagerV3GetTextInputRequest].
    ZwpTextInputManagerV3GetTextInput(ZwpTextInputManagerV3GetTextInputRequest),
}
impl CommandReader<Self> for CommandRequest {
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
impl InterfaceReader for CommandRequest {
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
                    ZwpTextInputV1ActivateRequest::OPCODE => Ok(Self::ZwpTextInputV1Activate(
                        ZwpTextInputV1ActivateRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1DeactivateRequest::OPCODE => Ok(Self::ZwpTextInputV1Deactivate(
                        ZwpTextInputV1DeactivateRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1ShowInputPanelRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1ShowInputPanel(
                            ZwpTextInputV1ShowInputPanelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1HideInputPanelRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1HideInputPanel(
                            ZwpTextInputV1HideInputPanelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1ResetRequest::OPCODE => Ok(Self::ZwpTextInputV1Reset(
                        ZwpTextInputV1ResetRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV1SetSurroundingTextRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1SetSurroundingText(
                            ZwpTextInputV1SetSurroundingTextRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1SetContentTypeRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1SetContentType(
                            ZwpTextInputV1SetContentTypeRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1SetCursorRectangleRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1SetCursorRectangle(
                            ZwpTextInputV1SetCursorRectangleRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1SetPreferredLanguageRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1SetPreferredLanguage(
                            ZwpTextInputV1SetPreferredLanguageRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1CommitStateRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1CommitState(
                            ZwpTextInputV1CommitStateRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV1InvokeActionRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV1InvokeAction(
                            ZwpTextInputV1InvokeActionRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputManagerV1 {
                ..
            }) => match opcode {
                ZwpTextInputManagerV1CreateTextInputRequest::OPCODE => {
                    Ok(Self::ZwpTextInputManagerV1CreateTextInput(
                        ZwpTextInputManagerV1CreateTextInputRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputV3 { .. }) => {
                match opcode {
                    ZwpTextInputV3DestroyRequest::OPCODE => Ok(Self::ZwpTextInputV3Destroy(
                        ZwpTextInputV3DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3EnableRequest::OPCODE => Ok(Self::ZwpTextInputV3Enable(
                        ZwpTextInputV3EnableRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3DisableRequest::OPCODE => Ok(Self::ZwpTextInputV3Disable(
                        ZwpTextInputV3DisableRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3SetSurroundingTextRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3SetSurroundingText(
                            ZwpTextInputV3SetSurroundingTextRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3SetTextChangeCauseRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3SetTextChangeCause(
                            ZwpTextInputV3SetTextChangeCauseRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3SetContentTypeRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3SetContentType(
                            ZwpTextInputV3SetContentTypeRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3SetCursorRectangleRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3SetCursorRectangle(
                            ZwpTextInputV3SetCursorRectangleRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3CommitRequest::OPCODE => Ok(Self::ZwpTextInputV3Commit(
                        ZwpTextInputV3CommitRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpTextInputV3SetAvailableActionsRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3SetAvailableActions(
                            ZwpTextInputV3SetAvailableActionsRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3ShowInputPanelRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3ShowInputPanel(
                            ZwpTextInputV3ShowInputPanelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    ZwpTextInputV3HideInputPanelRequest::OPCODE => {
                        Ok(Self::ZwpTextInputV3HideInputPanel(
                            ZwpTextInputV3HideInputPanelRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::TextInput(TextInputObjectInteface::ZwpTextInputManagerV3 {
                ..
            }) => match opcode {
                ZwpTextInputManagerV3DestroyRequest::OPCODE => {
                    Ok(Self::ZwpTextInputManagerV3Destroy(
                        ZwpTextInputManagerV3DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpTextInputManagerV3GetTextInputRequest::OPCODE => {
                    Ok(Self::ZwpTextInputManagerV3GetTextInput(
                        ZwpTextInputManagerV3GetTextInputRequest::read_with_interface(
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
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwpTextInputV1Activate(m) => m.object_id(),
            Self::ZwpTextInputV1Deactivate(m) => m.object_id(),
            Self::ZwpTextInputV1ShowInputPanel(m) => m.object_id(),
            Self::ZwpTextInputV1HideInputPanel(m) => m.object_id(),
            Self::ZwpTextInputV1Reset(m) => m.object_id(),
            Self::ZwpTextInputV1SetSurroundingText(m) => m.object_id(),
            Self::ZwpTextInputV1SetContentType(m) => m.object_id(),
            Self::ZwpTextInputV1SetCursorRectangle(m) => m.object_id(),
            Self::ZwpTextInputV1SetPreferredLanguage(m) => m.object_id(),
            Self::ZwpTextInputV1CommitState(m) => m.object_id(),
            Self::ZwpTextInputV1InvokeAction(m) => m.object_id(),
            Self::ZwpTextInputManagerV1CreateTextInput(m) => m.object_id(),
            Self::ZwpTextInputV3Destroy(m) => m.object_id(),
            Self::ZwpTextInputV3Enable(m) => m.object_id(),
            Self::ZwpTextInputV3Disable(m) => m.object_id(),
            Self::ZwpTextInputV3SetSurroundingText(m) => m.object_id(),
            Self::ZwpTextInputV3SetTextChangeCause(m) => m.object_id(),
            Self::ZwpTextInputV3SetContentType(m) => m.object_id(),
            Self::ZwpTextInputV3SetCursorRectangle(m) => m.object_id(),
            Self::ZwpTextInputV3Commit(m) => m.object_id(),
            Self::ZwpTextInputV3SetAvailableActions(m) => m.object_id(),
            Self::ZwpTextInputV3ShowInputPanel(m) => m.object_id(),
            Self::ZwpTextInputV3HideInputPanel(m) => m.object_id(),
            Self::ZwpTextInputManagerV3Destroy(m) => m.object_id(),
            Self::ZwpTextInputManagerV3GetTextInput(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpTextInputV1Activate(m) => m.opcode(),
            Self::ZwpTextInputV1Deactivate(m) => m.opcode(),
            Self::ZwpTextInputV1ShowInputPanel(m) => m.opcode(),
            Self::ZwpTextInputV1HideInputPanel(m) => m.opcode(),
            Self::ZwpTextInputV1Reset(m) => m.opcode(),
            Self::ZwpTextInputV1SetSurroundingText(m) => m.opcode(),
            Self::ZwpTextInputV1SetContentType(m) => m.opcode(),
            Self::ZwpTextInputV1SetCursorRectangle(m) => m.opcode(),
            Self::ZwpTextInputV1SetPreferredLanguage(m) => m.opcode(),
            Self::ZwpTextInputV1CommitState(m) => m.opcode(),
            Self::ZwpTextInputV1InvokeAction(m) => m.opcode(),
            Self::ZwpTextInputManagerV1CreateTextInput(m) => m.opcode(),
            Self::ZwpTextInputV3Destroy(m) => m.opcode(),
            Self::ZwpTextInputV3Enable(m) => m.opcode(),
            Self::ZwpTextInputV3Disable(m) => m.opcode(),
            Self::ZwpTextInputV3SetSurroundingText(m) => m.opcode(),
            Self::ZwpTextInputV3SetTextChangeCause(m) => m.opcode(),
            Self::ZwpTextInputV3SetContentType(m) => m.opcode(),
            Self::ZwpTextInputV3SetCursorRectangle(m) => m.opcode(),
            Self::ZwpTextInputV3Commit(m) => m.opcode(),
            Self::ZwpTextInputV3SetAvailableActions(m) => m.opcode(),
            Self::ZwpTextInputV3ShowInputPanel(m) => m.opcode(),
            Self::ZwpTextInputV3HideInputPanel(m) => m.opcode(),
            Self::ZwpTextInputManagerV3Destroy(m) => m.opcode(),
            Self::ZwpTextInputManagerV3GetTextInput(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpTextInputV1Activate(m) => m.payload_size(),
            Self::ZwpTextInputV1Deactivate(m) => m.payload_size(),
            Self::ZwpTextInputV1ShowInputPanel(m) => m.payload_size(),
            Self::ZwpTextInputV1HideInputPanel(m) => m.payload_size(),
            Self::ZwpTextInputV1Reset(m) => m.payload_size(),
            Self::ZwpTextInputV1SetSurroundingText(m) => m.payload_size(),
            Self::ZwpTextInputV1SetContentType(m) => m.payload_size(),
            Self::ZwpTextInputV1SetCursorRectangle(m) => m.payload_size(),
            Self::ZwpTextInputV1SetPreferredLanguage(m) => m.payload_size(),
            Self::ZwpTextInputV1CommitState(m) => m.payload_size(),
            Self::ZwpTextInputV1InvokeAction(m) => m.payload_size(),
            Self::ZwpTextInputManagerV1CreateTextInput(m) => m.payload_size(),
            Self::ZwpTextInputV3Destroy(m) => m.payload_size(),
            Self::ZwpTextInputV3Enable(m) => m.payload_size(),
            Self::ZwpTextInputV3Disable(m) => m.payload_size(),
            Self::ZwpTextInputV3SetSurroundingText(m) => m.payload_size(),
            Self::ZwpTextInputV3SetTextChangeCause(m) => m.payload_size(),
            Self::ZwpTextInputV3SetContentType(m) => m.payload_size(),
            Self::ZwpTextInputV3SetCursorRectangle(m) => m.payload_size(),
            Self::ZwpTextInputV3Commit(m) => m.payload_size(),
            Self::ZwpTextInputV3SetAvailableActions(m) => m.payload_size(),
            Self::ZwpTextInputV3ShowInputPanel(m) => m.payload_size(),
            Self::ZwpTextInputV3HideInputPanel(m) => m.payload_size(),
            Self::ZwpTextInputManagerV3Destroy(m) => m.payload_size(),
            Self::ZwpTextInputManagerV3GetTextInput(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpTextInputV1Activate(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1Deactivate(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1ShowInputPanel(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1HideInputPanel(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1Reset(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1SetSurroundingText(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1SetContentType(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1SetCursorRectangle(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1SetPreferredLanguage(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1CommitState(m) => m.write(writer, state).await,
            Self::ZwpTextInputV1InvokeAction(m) => m.write(writer, state).await,
            Self::ZwpTextInputManagerV1CreateTextInput(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Destroy(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Enable(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Disable(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3SetSurroundingText(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3SetTextChangeCause(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3SetContentType(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3SetCursorRectangle(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3Commit(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3SetAvailableActions(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3ShowInputPanel(m) => m.write(writer, state).await,
            Self::ZwpTextInputV3HideInputPanel(m) => m.write(writer, state).await,
            Self::ZwpTextInputManagerV3Destroy(m) => m.write(writer, state).await,
            Self::ZwpTextInputManagerV3GetTextInput(m) => m.write(writer, state).await,
        }
    }
}
