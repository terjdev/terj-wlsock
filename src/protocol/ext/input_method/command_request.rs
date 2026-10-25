use super::ObjectInteface as InputMethodObjectInteface;
use super::{
    ZwpInputMethodContextV1CommitStringRequest, ZwpInputMethodContextV1CursorPositionRequest,
    ZwpInputMethodContextV1DeleteSurroundingTextRequest, ZwpInputMethodContextV1DestroyRequest,
    ZwpInputMethodContextV1GrabKeyboardRequest, ZwpInputMethodContextV1KeyRequest,
    ZwpInputMethodContextV1KeysymRequest, ZwpInputMethodContextV1LanguageRequest,
    ZwpInputMethodContextV1ModifiersMapRequest, ZwpInputMethodContextV1ModifiersRequest,
    ZwpInputMethodContextV1PreeditCursorRequest, ZwpInputMethodContextV1PreeditStringRequest,
    ZwpInputMethodContextV1PreeditStylingRequest, ZwpInputMethodContextV1TextDirectionRequest,
    ZwpInputPanelSurfaceV1SetOverlayPanelRequest, ZwpInputPanelSurfaceV1SetToplevelRequest,
    ZwpInputPanelV1GetInputPanelSurfaceRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches input method requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpInputMethodContextV1DestroyRequest].
    ZwpInputMethodContextV1Destroy(ZwpInputMethodContextV1DestroyRequest),
    /// Message [ZwpInputMethodContextV1CommitStringRequest].
    ZwpInputMethodContextV1CommitString(ZwpInputMethodContextV1CommitStringRequest),
    /// Message [ZwpInputMethodContextV1PreeditStringRequest].
    ZwpInputMethodContextV1PreeditString(ZwpInputMethodContextV1PreeditStringRequest),
    /// Message [ZwpInputMethodContextV1PreeditStylingRequest].
    ZwpInputMethodContextV1PreeditStyling(ZwpInputMethodContextV1PreeditStylingRequest),
    /// Message [ZwpInputMethodContextV1PreeditCursorRequest].
    ZwpInputMethodContextV1PreeditCursor(ZwpInputMethodContextV1PreeditCursorRequest),
    /// Message [ZwpInputMethodContextV1DeleteSurroundingTextRequest].
    ZwpInputMethodContextV1DeleteSurroundingText(
        ZwpInputMethodContextV1DeleteSurroundingTextRequest,
    ),
    /// Message [ZwpInputMethodContextV1CursorPositionRequest].
    ZwpInputMethodContextV1CursorPosition(ZwpInputMethodContextV1CursorPositionRequest),
    /// Message [ZwpInputMethodContextV1ModifiersMapRequest].
    ZwpInputMethodContextV1ModifiersMap(ZwpInputMethodContextV1ModifiersMapRequest),
    /// Message [ZwpInputMethodContextV1KeysymRequest].
    ZwpInputMethodContextV1Keysym(ZwpInputMethodContextV1KeysymRequest),
    /// Message [ZwpInputMethodContextV1GrabKeyboardRequest].
    ZwpInputMethodContextV1GrabKeyboard(ZwpInputMethodContextV1GrabKeyboardRequest),
    /// Message [ZwpInputMethodContextV1KeyRequest].
    ZwpInputMethodContextV1Key(ZwpInputMethodContextV1KeyRequest),
    /// Message [ZwpInputMethodContextV1ModifiersRequest].
    ZwpInputMethodContextV1Modifiers(ZwpInputMethodContextV1ModifiersRequest),
    /// Message [ZwpInputMethodContextV1LanguageRequest].
    ZwpInputMethodContextV1Language(ZwpInputMethodContextV1LanguageRequest),
    /// Message [ZwpInputMethodContextV1TextDirectionRequest].
    ZwpInputMethodContextV1TextDirection(ZwpInputMethodContextV1TextDirectionRequest),
    /// Message [ZwpInputPanelV1GetInputPanelSurfaceRequest].
    ZwpInputPanelV1GetInputPanelSurface(ZwpInputPanelV1GetInputPanelSurfaceRequest),
    /// Message [ZwpInputPanelSurfaceV1SetToplevelRequest].
    ZwpInputPanelSurfaceV1SetToplevel(ZwpInputPanelSurfaceV1SetToplevelRequest),
    /// Message [ZwpInputPanelSurfaceV1SetOverlayPanelRequest].
    ZwpInputPanelSurfaceV1SetOverlayPanel(ZwpInputPanelSurfaceV1SetOverlayPanelRequest),
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
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 {
                ..
            }) => match opcode {
                ZwpInputMethodContextV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1Destroy(
                        ZwpInputMethodContextV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1CommitStringRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1CommitString(
                        ZwpInputMethodContextV1CommitStringRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1PreeditStringRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1PreeditString(
                        ZwpInputMethodContextV1PreeditStringRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1PreeditStylingRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1PreeditStyling(
                        ZwpInputMethodContextV1PreeditStylingRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1PreeditCursorRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1PreeditCursor(
                        ZwpInputMethodContextV1PreeditCursorRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1DeleteSurroundingTextRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1DeleteSurroundingText(
                        ZwpInputMethodContextV1DeleteSurroundingTextRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1CursorPositionRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1CursorPosition(
                        ZwpInputMethodContextV1CursorPositionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1ModifiersMapRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1ModifiersMap(
                        ZwpInputMethodContextV1ModifiersMapRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1KeysymRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1Keysym(
                        ZwpInputMethodContextV1KeysymRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1GrabKeyboardRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1GrabKeyboard(
                        ZwpInputMethodContextV1GrabKeyboardRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1KeyRequest::OPCODE => Ok(Self::ZwpInputMethodContextV1Key(
                    ZwpInputMethodContextV1KeyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwpInputMethodContextV1ModifiersRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1Modifiers(
                        ZwpInputMethodContextV1ModifiersRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1LanguageRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1Language(
                        ZwpInputMethodContextV1LanguageRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1TextDirectionRequest::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1TextDirection(
                        ZwpInputMethodContextV1TextDirectionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputPanelV1 { .. }) => {
                match opcode {
                    ZwpInputPanelV1GetInputPanelSurfaceRequest::OPCODE => {
                        Ok(Self::ZwpInputPanelV1GetInputPanelSurface(
                            ZwpInputPanelV1GetInputPanelSurfaceRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputPanelSurfaceV1 {
                ..
            }) => match opcode {
                ZwpInputPanelSurfaceV1SetToplevelRequest::OPCODE => {
                    Ok(Self::ZwpInputPanelSurfaceV1SetToplevel(
                        ZwpInputPanelSurfaceV1SetToplevelRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputPanelSurfaceV1SetOverlayPanelRequest::OPCODE => {
                    Ok(Self::ZwpInputPanelSurfaceV1SetOverlayPanel(
                        ZwpInputPanelSurfaceV1SetOverlayPanelRequest::read_with_interface(
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
            Self::ZwpInputMethodContextV1Destroy(m) => m.object_id(),
            Self::ZwpInputMethodContextV1CommitString(m) => m.object_id(),
            Self::ZwpInputMethodContextV1PreeditString(m) => m.object_id(),
            Self::ZwpInputMethodContextV1PreeditStyling(m) => m.object_id(),
            Self::ZwpInputMethodContextV1PreeditCursor(m) => m.object_id(),
            Self::ZwpInputMethodContextV1DeleteSurroundingText(m) => m.object_id(),
            Self::ZwpInputMethodContextV1CursorPosition(m) => m.object_id(),
            Self::ZwpInputMethodContextV1ModifiersMap(m) => m.object_id(),
            Self::ZwpInputMethodContextV1Keysym(m) => m.object_id(),
            Self::ZwpInputMethodContextV1GrabKeyboard(m) => m.object_id(),
            Self::ZwpInputMethodContextV1Key(m) => m.object_id(),
            Self::ZwpInputMethodContextV1Modifiers(m) => m.object_id(),
            Self::ZwpInputMethodContextV1Language(m) => m.object_id(),
            Self::ZwpInputMethodContextV1TextDirection(m) => m.object_id(),
            Self::ZwpInputPanelV1GetInputPanelSurface(m) => m.object_id(),
            Self::ZwpInputPanelSurfaceV1SetToplevel(m) => m.object_id(),
            Self::ZwpInputPanelSurfaceV1SetOverlayPanel(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpInputMethodContextV1Destroy(m) => m.opcode(),
            Self::ZwpInputMethodContextV1CommitString(m) => m.opcode(),
            Self::ZwpInputMethodContextV1PreeditString(m) => m.opcode(),
            Self::ZwpInputMethodContextV1PreeditStyling(m) => m.opcode(),
            Self::ZwpInputMethodContextV1PreeditCursor(m) => m.opcode(),
            Self::ZwpInputMethodContextV1DeleteSurroundingText(m) => m.opcode(),
            Self::ZwpInputMethodContextV1CursorPosition(m) => m.opcode(),
            Self::ZwpInputMethodContextV1ModifiersMap(m) => m.opcode(),
            Self::ZwpInputMethodContextV1Keysym(m) => m.opcode(),
            Self::ZwpInputMethodContextV1GrabKeyboard(m) => m.opcode(),
            Self::ZwpInputMethodContextV1Key(m) => m.opcode(),
            Self::ZwpInputMethodContextV1Modifiers(m) => m.opcode(),
            Self::ZwpInputMethodContextV1Language(m) => m.opcode(),
            Self::ZwpInputMethodContextV1TextDirection(m) => m.opcode(),
            Self::ZwpInputPanelV1GetInputPanelSurface(m) => m.opcode(),
            Self::ZwpInputPanelSurfaceV1SetToplevel(m) => m.opcode(),
            Self::ZwpInputPanelSurfaceV1SetOverlayPanel(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpInputMethodContextV1Destroy(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1CommitString(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1PreeditString(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1PreeditStyling(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1PreeditCursor(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1DeleteSurroundingText(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1CursorPosition(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1ModifiersMap(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1Keysym(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1GrabKeyboard(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1Key(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1Modifiers(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1Language(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1TextDirection(m) => m.payload_size(),
            Self::ZwpInputPanelV1GetInputPanelSurface(m) => m.payload_size(),
            Self::ZwpInputPanelSurfaceV1SetToplevel(m) => m.payload_size(),
            Self::ZwpInputPanelSurfaceV1SetOverlayPanel(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpInputMethodContextV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1CommitString(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1PreeditString(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1PreeditStyling(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1PreeditCursor(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1DeleteSurroundingText(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1CursorPosition(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1ModifiersMap(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1Keysym(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1GrabKeyboard(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1Key(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1Modifiers(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1Language(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1TextDirection(m) => m.write(writer, state).await,
            Self::ZwpInputPanelV1GetInputPanelSurface(m) => m.write(writer, state).await,
            Self::ZwpInputPanelSurfaceV1SetToplevel(m) => m.write(writer, state).await,
            Self::ZwpInputPanelSurfaceV1SetOverlayPanel(m) => m.write(writer, state).await,
        }
    }
}
