use super::ObjectInteface as FullscreenShellObjectInteface;
use super::{
    ZwpFullscreenShellModeFeedbackV1ModeFailedEvent,
    ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent,
    ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent, ZwpFullscreenShellV1CapabilityEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches fullscreen shell events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpFullscreenShellV1CapabilityEvent].
    ZwpFullscreenShellV1Capability(ZwpFullscreenShellV1CapabilityEvent),
    /// Message [ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent].
    ZwpFullscreenShellModeFeedbackV1ModeSuccessful(
        ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent,
    ),
    /// Message [ZwpFullscreenShellModeFeedbackV1ModeFailedEvent].
    ZwpFullscreenShellModeFeedbackV1ModeFailed(ZwpFullscreenShellModeFeedbackV1ModeFailedEvent),
    /// Message [ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent].
    ZwpFullscreenShellModeFeedbackV1PresentCancelled(
        ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent,
    ),
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
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellV1 { .. },
            ) => match opcode {
                ZwpFullscreenShellV1CapabilityEvent::OPCODE => {
                    Ok(Self::ZwpFullscreenShellV1Capability(
                        ZwpFullscreenShellV1CapabilityEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::FullscreenShell(
                FullscreenShellObjectInteface::ZwpFullscreenShellModeFeedbackV1 { .. },
            ) => match opcode {
                ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent::OPCODE => {
                    Ok(Self::ZwpFullscreenShellModeFeedbackV1ModeSuccessful(
                        ZwpFullscreenShellModeFeedbackV1ModeSuccessfulEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpFullscreenShellModeFeedbackV1ModeFailedEvent::OPCODE => {
                    Ok(Self::ZwpFullscreenShellModeFeedbackV1ModeFailed(
                        ZwpFullscreenShellModeFeedbackV1ModeFailedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent::OPCODE => {
                    Ok(Self::ZwpFullscreenShellModeFeedbackV1PresentCancelled(
                        ZwpFullscreenShellModeFeedbackV1PresentCancelledEvent::read_with_interface(
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
            Self::ZwpFullscreenShellV1Capability(m) => m.object_id(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeSuccessful(m) => m.object_id(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeFailed(m) => m.object_id(),
            Self::ZwpFullscreenShellModeFeedbackV1PresentCancelled(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpFullscreenShellV1Capability(m) => m.opcode(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeSuccessful(m) => m.opcode(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeFailed(m) => m.opcode(),
            Self::ZwpFullscreenShellModeFeedbackV1PresentCancelled(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpFullscreenShellV1Capability(m) => m.payload_size(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeSuccessful(m) => m.payload_size(),
            Self::ZwpFullscreenShellModeFeedbackV1ModeFailed(m) => m.payload_size(),
            Self::ZwpFullscreenShellModeFeedbackV1PresentCancelled(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpFullscreenShellV1Capability(m) => m.write(writer, state).await,
            Self::ZwpFullscreenShellModeFeedbackV1ModeSuccessful(m) => m.write(writer, state).await,
            Self::ZwpFullscreenShellModeFeedbackV1ModeFailed(m) => m.write(writer, state).await,
            Self::ZwpFullscreenShellModeFeedbackV1PresentCancelled(m) => {
                m.write(writer, state).await
            }
        }
    }
}
