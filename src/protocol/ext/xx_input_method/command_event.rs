use super::ObjectInteface as XxInputMethodObjectInteface;
use super::{
    XxInputMethodV1ActivateEvent, XxInputMethodV1AnnounceProtocolCompatEvent,
    XxInputMethodV1AnnounceSupportedFeaturesEvent, XxInputMethodV1ContentTypeEvent,
    XxInputMethodV1DeactivateEvent, XxInputMethodV1DoneEvent,
    XxInputMethodV1SetAvailableActionsEvent, XxInputMethodV1SurroundingTextEvent,
    XxInputMethodV1TextChangeCauseEvent, XxInputMethodV1UnavailableEvent,
    XxInputPopupSurfaceV2RepositionedEvent, XxInputPopupSurfaceV2StartConfigureEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches xx input method events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [XxInputMethodV1ActivateEvent].
    XxInputMethodV1Activate(XxInputMethodV1ActivateEvent),
    /// Message [XxInputMethodV1DeactivateEvent].
    XxInputMethodV1Deactivate(XxInputMethodV1DeactivateEvent),
    /// Message [XxInputMethodV1SurroundingTextEvent].
    XxInputMethodV1SurroundingText(XxInputMethodV1SurroundingTextEvent),
    /// Message [XxInputMethodV1TextChangeCauseEvent].
    XxInputMethodV1TextChangeCause(XxInputMethodV1TextChangeCauseEvent),
    /// Message [XxInputMethodV1ContentTypeEvent].
    XxInputMethodV1ContentType(XxInputMethodV1ContentTypeEvent),
    /// Message [XxInputMethodV1SetAvailableActionsEvent].
    XxInputMethodV1SetAvailableActions(XxInputMethodV1SetAvailableActionsEvent),
    /// Message [XxInputMethodV1AnnounceSupportedFeaturesEvent].
    XxInputMethodV1AnnounceSupportedFeatures(XxInputMethodV1AnnounceSupportedFeaturesEvent),
    /// Message [XxInputMethodV1AnnounceProtocolCompatEvent].
    XxInputMethodV1AnnounceProtocolCompat(XxInputMethodV1AnnounceProtocolCompatEvent),
    /// Message [XxInputMethodV1DoneEvent].
    XxInputMethodV1Done(XxInputMethodV1DoneEvent),
    /// Message [XxInputMethodV1UnavailableEvent].
    XxInputMethodV1Unavailable(XxInputMethodV1UnavailableEvent),
    /// Message [XxInputPopupSurfaceV2StartConfigureEvent].
    XxInputPopupSurfaceV2StartConfigure(XxInputPopupSurfaceV2StartConfigureEvent),
    /// Message [XxInputPopupSurfaceV2RepositionedEvent].
    XxInputPopupSurfaceV2Repositioned(XxInputPopupSurfaceV2RepositionedEvent),
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
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputMethodV1 {
                ..
            }) => match opcode {
                XxInputMethodV1ActivateEvent::OPCODE => Ok(Self::XxInputMethodV1Activate(
                    XxInputMethodV1ActivateEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxInputMethodV1DeactivateEvent::OPCODE => Ok(Self::XxInputMethodV1Deactivate(
                    XxInputMethodV1DeactivateEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxInputMethodV1SurroundingTextEvent::OPCODE => {
                    Ok(Self::XxInputMethodV1SurroundingText(
                        XxInputMethodV1SurroundingTextEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputMethodV1TextChangeCauseEvent::OPCODE => {
                    Ok(Self::XxInputMethodV1TextChangeCause(
                        XxInputMethodV1TextChangeCauseEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputMethodV1ContentTypeEvent::OPCODE => Ok(Self::XxInputMethodV1ContentType(
                    XxInputMethodV1ContentTypeEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxInputMethodV1SetAvailableActionsEvent::OPCODE => {
                    Ok(Self::XxInputMethodV1SetAvailableActions(
                        XxInputMethodV1SetAvailableActionsEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputMethodV1AnnounceSupportedFeaturesEvent::OPCODE => {
                    Ok(Self::XxInputMethodV1AnnounceSupportedFeatures(
                        XxInputMethodV1AnnounceSupportedFeaturesEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputMethodV1AnnounceProtocolCompatEvent::OPCODE => {
                    Ok(Self::XxInputMethodV1AnnounceProtocolCompat(
                        XxInputMethodV1AnnounceProtocolCompatEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputMethodV1DoneEvent::OPCODE => Ok(Self::XxInputMethodV1Done(
                    XxInputMethodV1DoneEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxInputMethodV1UnavailableEvent::OPCODE => Ok(Self::XxInputMethodV1Unavailable(
                    XxInputMethodV1UnavailableEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxInputMethod(XxInputMethodObjectInteface::XxInputPopupSurfaceV2 {
                ..
            }) => match opcode {
                XxInputPopupSurfaceV2StartConfigureEvent::OPCODE => {
                    Ok(Self::XxInputPopupSurfaceV2StartConfigure(
                        XxInputPopupSurfaceV2StartConfigureEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                XxInputPopupSurfaceV2RepositionedEvent::OPCODE => {
                    Ok(Self::XxInputPopupSurfaceV2Repositioned(
                        XxInputPopupSurfaceV2RepositionedEvent::read_with_interface(
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
            Self::XxInputMethodV1Activate(m) => m.object_id(),
            Self::XxInputMethodV1Deactivate(m) => m.object_id(),
            Self::XxInputMethodV1SurroundingText(m) => m.object_id(),
            Self::XxInputMethodV1TextChangeCause(m) => m.object_id(),
            Self::XxInputMethodV1ContentType(m) => m.object_id(),
            Self::XxInputMethodV1SetAvailableActions(m) => m.object_id(),
            Self::XxInputMethodV1AnnounceSupportedFeatures(m) => m.object_id(),
            Self::XxInputMethodV1AnnounceProtocolCompat(m) => m.object_id(),
            Self::XxInputMethodV1Done(m) => m.object_id(),
            Self::XxInputMethodV1Unavailable(m) => m.object_id(),
            Self::XxInputPopupSurfaceV2StartConfigure(m) => m.object_id(),
            Self::XxInputPopupSurfaceV2Repositioned(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxInputMethodV1Activate(m) => m.opcode(),
            Self::XxInputMethodV1Deactivate(m) => m.opcode(),
            Self::XxInputMethodV1SurroundingText(m) => m.opcode(),
            Self::XxInputMethodV1TextChangeCause(m) => m.opcode(),
            Self::XxInputMethodV1ContentType(m) => m.opcode(),
            Self::XxInputMethodV1SetAvailableActions(m) => m.opcode(),
            Self::XxInputMethodV1AnnounceSupportedFeatures(m) => m.opcode(),
            Self::XxInputMethodV1AnnounceProtocolCompat(m) => m.opcode(),
            Self::XxInputMethodV1Done(m) => m.opcode(),
            Self::XxInputMethodV1Unavailable(m) => m.opcode(),
            Self::XxInputPopupSurfaceV2StartConfigure(m) => m.opcode(),
            Self::XxInputPopupSurfaceV2Repositioned(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxInputMethodV1Activate(m) => m.payload_size(),
            Self::XxInputMethodV1Deactivate(m) => m.payload_size(),
            Self::XxInputMethodV1SurroundingText(m) => m.payload_size(),
            Self::XxInputMethodV1TextChangeCause(m) => m.payload_size(),
            Self::XxInputMethodV1ContentType(m) => m.payload_size(),
            Self::XxInputMethodV1SetAvailableActions(m) => m.payload_size(),
            Self::XxInputMethodV1AnnounceSupportedFeatures(m) => m.payload_size(),
            Self::XxInputMethodV1AnnounceProtocolCompat(m) => m.payload_size(),
            Self::XxInputMethodV1Done(m) => m.payload_size(),
            Self::XxInputMethodV1Unavailable(m) => m.payload_size(),
            Self::XxInputPopupSurfaceV2StartConfigure(m) => m.payload_size(),
            Self::XxInputPopupSurfaceV2Repositioned(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxInputMethodV1Activate(m) => m.write(writer, state).await,
            Self::XxInputMethodV1Deactivate(m) => m.write(writer, state).await,
            Self::XxInputMethodV1SurroundingText(m) => m.write(writer, state).await,
            Self::XxInputMethodV1TextChangeCause(m) => m.write(writer, state).await,
            Self::XxInputMethodV1ContentType(m) => m.write(writer, state).await,
            Self::XxInputMethodV1SetAvailableActions(m) => m.write(writer, state).await,
            Self::XxInputMethodV1AnnounceSupportedFeatures(m) => m.write(writer, state).await,
            Self::XxInputMethodV1AnnounceProtocolCompat(m) => m.write(writer, state).await,
            Self::XxInputMethodV1Done(m) => m.write(writer, state).await,
            Self::XxInputMethodV1Unavailable(m) => m.write(writer, state).await,
            Self::XxInputPopupSurfaceV2StartConfigure(m) => m.write(writer, state).await,
            Self::XxInputPopupSurfaceV2Repositioned(m) => m.write(writer, state).await,
        }
    }
}
