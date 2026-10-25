use super::{
    ZwlrDataControlDeviceV1DataOfferEvent, ZwlrDataControlDeviceV1FinishedEvent,
    ZwlrDataControlDeviceV1PrimarySelectionEvent, ZwlrDataControlDeviceV1SelectionEvent,
    ZwlrDataControlOfferV1OfferEvent, ZwlrDataControlSourceV1CancelledEvent,
    ZwlrDataControlSourceV1SendEvent,
};
use crate::protocol::wlr::data_control::ObjectInteface as WlrDataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR data control events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwlrDataControlDeviceV1DataOfferEvent].
    ZwlrDataControlDeviceV1DataOffer(ZwlrDataControlDeviceV1DataOfferEvent),
    /// Message [ZwlrDataControlDeviceV1SelectionEvent].
    ZwlrDataControlDeviceV1Selection(ZwlrDataControlDeviceV1SelectionEvent),
    /// Message [ZwlrDataControlDeviceV1FinishedEvent].
    ZwlrDataControlDeviceV1Finished(ZwlrDataControlDeviceV1FinishedEvent),
    /// Message [ZwlrDataControlDeviceV1PrimarySelectionEvent].
    ZwlrDataControlDeviceV1PrimarySelection(ZwlrDataControlDeviceV1PrimarySelectionEvent),
    /// Message [ZwlrDataControlSourceV1SendEvent].
    ZwlrDataControlSourceV1Send(ZwlrDataControlSourceV1SendEvent),
    /// Message [ZwlrDataControlSourceV1CancelledEvent].
    ZwlrDataControlSourceV1Cancelled(ZwlrDataControlSourceV1CancelledEvent),
    /// Message [ZwlrDataControlOfferV1OfferEvent].
    ZwlrDataControlOfferV1Offer(ZwlrDataControlOfferV1OfferEvent),
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
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlDeviceV1 { .. },
            ) => match opcode {
                ZwlrDataControlDeviceV1DataOfferEvent::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1DataOffer(
                        ZwlrDataControlDeviceV1DataOfferEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlDeviceV1SelectionEvent::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1Selection(
                        ZwlrDataControlDeviceV1SelectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlDeviceV1FinishedEvent::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1Finished(
                        ZwlrDataControlDeviceV1FinishedEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlDeviceV1PrimarySelectionEvent::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1PrimarySelection(
                        ZwlrDataControlDeviceV1PrimarySelectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlSourceV1 { .. },
            ) => match opcode {
                ZwlrDataControlSourceV1SendEvent::OPCODE => Ok(Self::ZwlrDataControlSourceV1Send(
                    ZwlrDataControlSourceV1SendEvent::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                ZwlrDataControlSourceV1CancelledEvent::OPCODE => {
                    Ok(Self::ZwlrDataControlSourceV1Cancelled(
                        ZwlrDataControlSourceV1CancelledEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlOfferV1 { .. },
            ) => match opcode {
                ZwlrDataControlOfferV1OfferEvent::OPCODE => Ok(Self::ZwlrDataControlOfferV1Offer(
                    ZwlrDataControlOfferV1OfferEvent::read_with_interface(
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
            Self::ZwlrDataControlDeviceV1DataOffer(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1Selection(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1Finished(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1PrimarySelection(message) => message.object_id(),
            Self::ZwlrDataControlSourceV1Send(message) => message.object_id(),
            Self::ZwlrDataControlSourceV1Cancelled(message) => message.object_id(),
            Self::ZwlrDataControlOfferV1Offer(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrDataControlDeviceV1DataOffer(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1Selection(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1Finished(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1PrimarySelection(message) => message.opcode(),
            Self::ZwlrDataControlSourceV1Send(message) => message.opcode(),
            Self::ZwlrDataControlSourceV1Cancelled(message) => message.opcode(),
            Self::ZwlrDataControlOfferV1Offer(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrDataControlDeviceV1DataOffer(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1Selection(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1Finished(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1PrimarySelection(message) => message.payload_size(),
            Self::ZwlrDataControlSourceV1Send(message) => message.payload_size(),
            Self::ZwlrDataControlSourceV1Cancelled(message) => message.payload_size(),
            Self::ZwlrDataControlOfferV1Offer(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrDataControlDeviceV1DataOffer(message) => message.write(writer, state).await,
            Self::ZwlrDataControlDeviceV1Selection(message) => message.write(writer, state).await,
            Self::ZwlrDataControlDeviceV1Finished(message) => message.write(writer, state).await,
            Self::ZwlrDataControlDeviceV1PrimarySelection(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrDataControlSourceV1Send(message) => message.write(writer, state).await,
            Self::ZwlrDataControlSourceV1Cancelled(message) => message.write(writer, state).await,
            Self::ZwlrDataControlOfferV1Offer(message) => message.write(writer, state).await,
        }
    }
}
