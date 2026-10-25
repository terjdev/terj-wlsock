use crate::protocol::ext::presentation_time::ObjectInteface as PresentationTimeObjectInteface;
use crate::protocol::ext::presentation_time::{
    WpPresentationClockIdEvent, WpPresentationFeedbackDiscardedEvent,
    WpPresentationFeedbackPresentedEvent, WpPresentationFeedbackSyncOutputEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Contains Presentation time events and delegates their encoding and decoding.
#[derive(Debug)]
pub enum CommandEvent {
    /// Event [WpPresentationClockIdEvent].
    WpPresentationClockId(WpPresentationClockIdEvent),
    /// Event [WpPresentationFeedbackSyncOutputEvent].
    WpPresentationFeedbackSyncOutput(WpPresentationFeedbackSyncOutputEvent),
    /// Event [WpPresentationFeedbackPresentedEvent].
    WpPresentationFeedbackPresented(WpPresentationFeedbackPresentedEvent),
    /// Event [WpPresentationFeedbackDiscardedEvent].
    WpPresentationFeedbackDiscarded(WpPresentationFeedbackDiscardedEvent),
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpPresentationClockId(event) => event.object_id(),
            Self::WpPresentationFeedbackSyncOutput(event) => event.object_id(),
            Self::WpPresentationFeedbackPresented(event) => event.object_id(),
            Self::WpPresentationFeedbackDiscarded(event) => event.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpPresentationClockId(event) => event.opcode(),
            Self::WpPresentationFeedbackSyncOutput(event) => event.opcode(),
            Self::WpPresentationFeedbackPresented(event) => event.opcode(),
            Self::WpPresentationFeedbackDiscarded(event) => event.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpPresentationClockId(event) => event.payload_size(),
            Self::WpPresentationFeedbackSyncOutput(event) => event.payload_size(),
            Self::WpPresentationFeedbackPresented(event) => event.payload_size(),
            Self::WpPresentationFeedbackDiscarded(event) => event.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpPresentationClockId(event) => event.write(writer, state).await,
            Self::WpPresentationFeedbackSyncOutput(event) => event.write(writer, state).await,
            Self::WpPresentationFeedbackPresented(event) => event.write(writer, state).await,
            Self::WpPresentationFeedbackDiscarded(event) => event.write(writer, state).await,
        }
    }
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
            ObjectInteface::PresentationTime(PresentationTimeObjectInteface::WpPresentation {
                ..
            }) => match opcode {
                WpPresentationClockIdEvent::OPCODE => Ok(Self::WpPresentationClockId(
                    WpPresentationClockIdEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PresentationTime(
                PresentationTimeObjectInteface::WpPresentationFeedback { .. },
            ) => match opcode {
                WpPresentationFeedbackSyncOutputEvent::OPCODE => {
                    Ok(Self::WpPresentationFeedbackSyncOutput(
                        WpPresentationFeedbackSyncOutputEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpPresentationFeedbackPresentedEvent::OPCODE => {
                    Ok(Self::WpPresentationFeedbackPresented(
                        WpPresentationFeedbackPresentedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpPresentationFeedbackDiscardedEvent::OPCODE => {
                    Ok(Self::WpPresentationFeedbackDiscarded(
                        WpPresentationFeedbackDiscardedEvent::read_with_interface(
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
