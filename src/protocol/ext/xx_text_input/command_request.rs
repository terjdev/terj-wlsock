use super::ObjectInteface as XxTextInputObjectInteface;
use super::{
    XxTextInputManagerV3DestroyRequest, XxTextInputManagerV3GetTextInputRequest,
    XxTextInputV3AnnounceSupportedFeaturesRequest, XxTextInputV3CommitRequest,
    XxTextInputV3DestroyRequest, XxTextInputV3DisableRequest, XxTextInputV3EnableRequest,
    XxTextInputV3SetAvailableActionsRequest, XxTextInputV3SetContentTypeRequest,
    XxTextInputV3SetCursorRectangleRequest, XxTextInputV3SetSurroundingTextRequest,
    XxTextInputV3SetTextChangeCauseRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xx text input requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxTextInputV3DestroyRequest].
    XxTextInputV3Destroy(XxTextInputV3DestroyRequest),
    /// Message [XxTextInputV3EnableRequest].
    XxTextInputV3Enable(XxTextInputV3EnableRequest),
    /// Message [XxTextInputV3DisableRequest].
    XxTextInputV3Disable(XxTextInputV3DisableRequest),
    /// Message [XxTextInputV3SetSurroundingTextRequest].
    XxTextInputV3SetSurroundingText(XxTextInputV3SetSurroundingTextRequest),
    /// Message [XxTextInputV3SetTextChangeCauseRequest].
    XxTextInputV3SetTextChangeCause(XxTextInputV3SetTextChangeCauseRequest),
    /// Message [XxTextInputV3SetContentTypeRequest].
    XxTextInputV3SetContentType(XxTextInputV3SetContentTypeRequest),
    /// Message [XxTextInputV3SetCursorRectangleRequest].
    XxTextInputV3SetCursorRectangle(XxTextInputV3SetCursorRectangleRequest),
    /// Message [XxTextInputV3CommitRequest].
    XxTextInputV3Commit(XxTextInputV3CommitRequest),
    /// Message [XxTextInputV3SetAvailableActionsRequest].
    XxTextInputV3SetAvailableActions(XxTextInputV3SetAvailableActionsRequest),
    /// Message [XxTextInputV3AnnounceSupportedFeaturesRequest].
    XxTextInputV3AnnounceSupportedFeatures(XxTextInputV3AnnounceSupportedFeaturesRequest),
    /// Message [XxTextInputManagerV3DestroyRequest].
    XxTextInputManagerV3Destroy(XxTextInputManagerV3DestroyRequest),
    /// Message [XxTextInputManagerV3GetTextInputRequest].
    XxTextInputManagerV3GetTextInput(XxTextInputManagerV3GetTextInputRequest),
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
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputV3 { .. }) => {
                match opcode {
                    XxTextInputV3DestroyRequest::OPCODE => Ok(Self::XxTextInputV3Destroy(
                        XxTextInputV3DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3EnableRequest::OPCODE => Ok(Self::XxTextInputV3Enable(
                        XxTextInputV3EnableRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3DisableRequest::OPCODE => Ok(Self::XxTextInputV3Disable(
                        XxTextInputV3DisableRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3SetSurroundingTextRequest::OPCODE => {
                        Ok(Self::XxTextInputV3SetSurroundingText(
                            XxTextInputV3SetSurroundingTextRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3SetTextChangeCauseRequest::OPCODE => {
                        Ok(Self::XxTextInputV3SetTextChangeCause(
                            XxTextInputV3SetTextChangeCauseRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3SetContentTypeRequest::OPCODE => {
                        Ok(Self::XxTextInputV3SetContentType(
                            XxTextInputV3SetContentTypeRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3SetCursorRectangleRequest::OPCODE => {
                        Ok(Self::XxTextInputV3SetCursorRectangle(
                            XxTextInputV3SetCursorRectangleRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3CommitRequest::OPCODE => Ok(Self::XxTextInputV3Commit(
                        XxTextInputV3CommitRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxTextInputV3SetAvailableActionsRequest::OPCODE => {
                        Ok(Self::XxTextInputV3SetAvailableActions(
                            XxTextInputV3SetAvailableActionsRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxTextInputV3AnnounceSupportedFeaturesRequest::OPCODE => {
                        Ok(Self::XxTextInputV3AnnounceSupportedFeatures(
                            XxTextInputV3AnnounceSupportedFeaturesRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XxTextInput(XxTextInputObjectInteface::XxTextInputManagerV3 {
                ..
            }) => match opcode {
                XxTextInputManagerV3DestroyRequest::OPCODE => {
                    Ok(Self::XxTextInputManagerV3Destroy(
                        XxTextInputManagerV3DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxTextInputManagerV3GetTextInputRequest::OPCODE => {
                    Ok(Self::XxTextInputManagerV3GetTextInput(
                        XxTextInputManagerV3GetTextInputRequest::read_with_interface(
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
            Self::XxTextInputV3Destroy(m) => m.object_id(),
            Self::XxTextInputV3Enable(m) => m.object_id(),
            Self::XxTextInputV3Disable(m) => m.object_id(),
            Self::XxTextInputV3SetSurroundingText(m) => m.object_id(),
            Self::XxTextInputV3SetTextChangeCause(m) => m.object_id(),
            Self::XxTextInputV3SetContentType(m) => m.object_id(),
            Self::XxTextInputV3SetCursorRectangle(m) => m.object_id(),
            Self::XxTextInputV3Commit(m) => m.object_id(),
            Self::XxTextInputV3SetAvailableActions(m) => m.object_id(),
            Self::XxTextInputV3AnnounceSupportedFeatures(m) => m.object_id(),
            Self::XxTextInputManagerV3Destroy(m) => m.object_id(),
            Self::XxTextInputManagerV3GetTextInput(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxTextInputV3Destroy(m) => m.opcode(),
            Self::XxTextInputV3Enable(m) => m.opcode(),
            Self::XxTextInputV3Disable(m) => m.opcode(),
            Self::XxTextInputV3SetSurroundingText(m) => m.opcode(),
            Self::XxTextInputV3SetTextChangeCause(m) => m.opcode(),
            Self::XxTextInputV3SetContentType(m) => m.opcode(),
            Self::XxTextInputV3SetCursorRectangle(m) => m.opcode(),
            Self::XxTextInputV3Commit(m) => m.opcode(),
            Self::XxTextInputV3SetAvailableActions(m) => m.opcode(),
            Self::XxTextInputV3AnnounceSupportedFeatures(m) => m.opcode(),
            Self::XxTextInputManagerV3Destroy(m) => m.opcode(),
            Self::XxTextInputManagerV3GetTextInput(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxTextInputV3Destroy(m) => m.payload_size(),
            Self::XxTextInputV3Enable(m) => m.payload_size(),
            Self::XxTextInputV3Disable(m) => m.payload_size(),
            Self::XxTextInputV3SetSurroundingText(m) => m.payload_size(),
            Self::XxTextInputV3SetTextChangeCause(m) => m.payload_size(),
            Self::XxTextInputV3SetContentType(m) => m.payload_size(),
            Self::XxTextInputV3SetCursorRectangle(m) => m.payload_size(),
            Self::XxTextInputV3Commit(m) => m.payload_size(),
            Self::XxTextInputV3SetAvailableActions(m) => m.payload_size(),
            Self::XxTextInputV3AnnounceSupportedFeatures(m) => m.payload_size(),
            Self::XxTextInputManagerV3Destroy(m) => m.payload_size(),
            Self::XxTextInputManagerV3GetTextInput(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxTextInputV3Destroy(m) => m.write(writer, state).await,
            Self::XxTextInputV3Enable(m) => m.write(writer, state).await,
            Self::XxTextInputV3Disable(m) => m.write(writer, state).await,
            Self::XxTextInputV3SetSurroundingText(m) => m.write(writer, state).await,
            Self::XxTextInputV3SetTextChangeCause(m) => m.write(writer, state).await,
            Self::XxTextInputV3SetContentType(m) => m.write(writer, state).await,
            Self::XxTextInputV3SetCursorRectangle(m) => m.write(writer, state).await,
            Self::XxTextInputV3Commit(m) => m.write(writer, state).await,
            Self::XxTextInputV3SetAvailableActions(m) => m.write(writer, state).await,
            Self::XxTextInputV3AnnounceSupportedFeatures(m) => m.write(writer, state).await,
            Self::XxTextInputManagerV3Destroy(m) => m.write(writer, state).await,
            Self::XxTextInputManagerV3GetTextInput(m) => m.write(writer, state).await,
        }
    }
}
