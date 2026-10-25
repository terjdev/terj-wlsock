use super::{
    ExtDataControlDeviceV1DataOfferEvent, ExtDataControlDeviceV1FinishedEvent,
    ExtDataControlDeviceV1PrimarySelectionEvent, ExtDataControlDeviceV1SelectionEvent,
    ExtDataControlOfferV1OfferEvent, ExtDataControlSourceV1CancelledEvent,
    ExtDataControlSourceV1SendEvent,
};
use crate::protocol::ext::data_control::ObjectInteface as DataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Data control events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ExtDataControlDeviceV1DataOfferEvent].
    ExtDataControlDeviceV1DataOffer(ExtDataControlDeviceV1DataOfferEvent),
    /// Message [ExtDataControlDeviceV1SelectionEvent].
    ExtDataControlDeviceV1Selection(ExtDataControlDeviceV1SelectionEvent),
    /// Message [ExtDataControlDeviceV1FinishedEvent].
    ExtDataControlDeviceV1Finished(ExtDataControlDeviceV1FinishedEvent),
    /// Message [ExtDataControlDeviceV1PrimarySelectionEvent].
    ExtDataControlDeviceV1PrimarySelection(ExtDataControlDeviceV1PrimarySelectionEvent),
    /// Message [ExtDataControlSourceV1SendEvent].
    ExtDataControlSourceV1Send(ExtDataControlSourceV1SendEvent),
    /// Message [ExtDataControlSourceV1CancelledEvent].
    ExtDataControlSourceV1Cancelled(ExtDataControlSourceV1CancelledEvent),
    /// Message [ExtDataControlOfferV1OfferEvent].
    ExtDataControlOfferV1Offer(ExtDataControlOfferV1OfferEvent),
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
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlDeviceV1 {
                ..
            }) => match opcode {
                ExtDataControlDeviceV1DataOfferEvent::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1DataOffer(
                        ExtDataControlDeviceV1DataOfferEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlDeviceV1SelectionEvent::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1Selection(
                        ExtDataControlDeviceV1SelectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlDeviceV1FinishedEvent::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1Finished(
                        ExtDataControlDeviceV1FinishedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlDeviceV1PrimarySelectionEvent::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1PrimarySelection(
                        ExtDataControlDeviceV1PrimarySelectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlSourceV1 {
                ..
            }) => match opcode {
                ExtDataControlSourceV1SendEvent::OPCODE => Ok(Self::ExtDataControlSourceV1Send(
                    ExtDataControlSourceV1SendEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ExtDataControlSourceV1CancelledEvent::OPCODE => {
                    Ok(Self::ExtDataControlSourceV1Cancelled(
                        ExtDataControlSourceV1CancelledEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlOfferV1 {
                ..
            }) => match opcode {
                ExtDataControlOfferV1OfferEvent::OPCODE => Ok(Self::ExtDataControlOfferV1Offer(
                    ExtDataControlOfferV1OfferEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandEvent {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtDataControlDeviceV1DataOffer(message) => message.object_id(),
            Self::ExtDataControlDeviceV1Selection(message) => message.object_id(),
            Self::ExtDataControlDeviceV1Finished(message) => message.object_id(),
            Self::ExtDataControlDeviceV1PrimarySelection(message) => message.object_id(),
            Self::ExtDataControlSourceV1Send(message) => message.object_id(),
            Self::ExtDataControlSourceV1Cancelled(message) => message.object_id(),
            Self::ExtDataControlOfferV1Offer(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtDataControlDeviceV1DataOffer(message) => message.opcode(),
            Self::ExtDataControlDeviceV1Selection(message) => message.opcode(),
            Self::ExtDataControlDeviceV1Finished(message) => message.opcode(),
            Self::ExtDataControlDeviceV1PrimarySelection(message) => message.opcode(),
            Self::ExtDataControlSourceV1Send(message) => message.opcode(),
            Self::ExtDataControlSourceV1Cancelled(message) => message.opcode(),
            Self::ExtDataControlOfferV1Offer(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtDataControlDeviceV1DataOffer(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1Selection(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1Finished(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1PrimarySelection(message) => message.payload_size(),
            Self::ExtDataControlSourceV1Send(message) => message.payload_size(),
            Self::ExtDataControlSourceV1Cancelled(message) => message.payload_size(),
            Self::ExtDataControlOfferV1Offer(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtDataControlDeviceV1DataOffer(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1Selection(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1Finished(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1PrimarySelection(message) => {
                message.write(writer, state).await
            }
            Self::ExtDataControlSourceV1Send(message) => message.write(writer, state).await,
            Self::ExtDataControlSourceV1Cancelled(message) => message.write(writer, state).await,
            Self::ExtDataControlOfferV1Offer(message) => message.write(writer, state).await,
        }
    }
}
