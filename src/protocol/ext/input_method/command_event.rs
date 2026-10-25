use super::ObjectInteface as InputMethodObjectInteface;
use super::{
    ZwpInputMethodContextV1CommitStateEvent, ZwpInputMethodContextV1ContentTypeEvent,
    ZwpInputMethodContextV1InvokeActionEvent, ZwpInputMethodContextV1PreferredLanguageEvent,
    ZwpInputMethodContextV1ResetEvent, ZwpInputMethodContextV1SurroundingTextEvent,
    ZwpInputMethodV1ActivateEvent, ZwpInputMethodV1DeactivateEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches input method events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpInputMethodContextV1SurroundingTextEvent].
    ZwpInputMethodContextV1SurroundingText(ZwpInputMethodContextV1SurroundingTextEvent),
    /// Message [ZwpInputMethodContextV1ResetEvent].
    ZwpInputMethodContextV1Reset(ZwpInputMethodContextV1ResetEvent),
    /// Message [ZwpInputMethodContextV1ContentTypeEvent].
    ZwpInputMethodContextV1ContentType(ZwpInputMethodContextV1ContentTypeEvent),
    /// Message [ZwpInputMethodContextV1InvokeActionEvent].
    ZwpInputMethodContextV1InvokeAction(ZwpInputMethodContextV1InvokeActionEvent),
    /// Message [ZwpInputMethodContextV1CommitStateEvent].
    ZwpInputMethodContextV1CommitState(ZwpInputMethodContextV1CommitStateEvent),
    /// Message [ZwpInputMethodContextV1PreferredLanguageEvent].
    ZwpInputMethodContextV1PreferredLanguage(ZwpInputMethodContextV1PreferredLanguageEvent),
    /// Message [ZwpInputMethodV1ActivateEvent].
    ZwpInputMethodV1Activate(ZwpInputMethodV1ActivateEvent),
    /// Message [ZwpInputMethodV1DeactivateEvent].
    ZwpInputMethodV1Deactivate(ZwpInputMethodV1DeactivateEvent),
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
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodContextV1 {
                ..
            }) => match opcode {
                ZwpInputMethodContextV1SurroundingTextEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1SurroundingText(
                        ZwpInputMethodContextV1SurroundingTextEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1ResetEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1Reset(
                        ZwpInputMethodContextV1ResetEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1ContentTypeEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1ContentType(
                        ZwpInputMethodContextV1ContentTypeEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1InvokeActionEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1InvokeAction(
                        ZwpInputMethodContextV1InvokeActionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1CommitStateEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1CommitState(
                        ZwpInputMethodContextV1CommitStateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpInputMethodContextV1PreferredLanguageEvent::OPCODE => {
                    Ok(Self::ZwpInputMethodContextV1PreferredLanguage(
                        ZwpInputMethodContextV1PreferredLanguageEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::InputMethod(InputMethodObjectInteface::ZwpInputMethodV1 { .. }) => {
                match opcode {
                    ZwpInputMethodV1ActivateEvent::OPCODE => Ok(Self::ZwpInputMethodV1Activate(
                        ZwpInputMethodV1ActivateEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    ZwpInputMethodV1DeactivateEvent::OPCODE => {
                        Ok(Self::ZwpInputMethodV1Deactivate(
                            ZwpInputMethodV1DeactivateEvent::read_with_interface(
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
            Self::ZwpInputMethodContextV1SurroundingText(m) => m.object_id(),
            Self::ZwpInputMethodContextV1Reset(m) => m.object_id(),
            Self::ZwpInputMethodContextV1ContentType(m) => m.object_id(),
            Self::ZwpInputMethodContextV1InvokeAction(m) => m.object_id(),
            Self::ZwpInputMethodContextV1CommitState(m) => m.object_id(),
            Self::ZwpInputMethodContextV1PreferredLanguage(m) => m.object_id(),
            Self::ZwpInputMethodV1Activate(m) => m.object_id(),
            Self::ZwpInputMethodV1Deactivate(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpInputMethodContextV1SurroundingText(m) => m.opcode(),
            Self::ZwpInputMethodContextV1Reset(m) => m.opcode(),
            Self::ZwpInputMethodContextV1ContentType(m) => m.opcode(),
            Self::ZwpInputMethodContextV1InvokeAction(m) => m.opcode(),
            Self::ZwpInputMethodContextV1CommitState(m) => m.opcode(),
            Self::ZwpInputMethodContextV1PreferredLanguage(m) => m.opcode(),
            Self::ZwpInputMethodV1Activate(m) => m.opcode(),
            Self::ZwpInputMethodV1Deactivate(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpInputMethodContextV1SurroundingText(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1Reset(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1ContentType(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1InvokeAction(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1CommitState(m) => m.payload_size(),
            Self::ZwpInputMethodContextV1PreferredLanguage(m) => m.payload_size(),
            Self::ZwpInputMethodV1Activate(m) => m.payload_size(),
            Self::ZwpInputMethodV1Deactivate(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpInputMethodContextV1SurroundingText(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1Reset(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1ContentType(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1InvokeAction(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1CommitState(m) => m.write(writer, state).await,
            Self::ZwpInputMethodContextV1PreferredLanguage(m) => m.write(writer, state).await,
            Self::ZwpInputMethodV1Activate(m) => m.write(writer, state).await,
            Self::ZwpInputMethodV1Deactivate(m) => m.write(writer, state).await,
        }
    }
}
