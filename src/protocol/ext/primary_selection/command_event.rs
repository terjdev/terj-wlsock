use super::ObjectInteface as PrimarySelectionObjectInteface;
use super::{
    ZwpPrimarySelectionDeviceV1DataOfferEvent, ZwpPrimarySelectionDeviceV1SelectionEvent,
    ZwpPrimarySelectionOfferV1OfferEvent, ZwpPrimarySelectionSourceV1CancelledEvent,
    ZwpPrimarySelectionSourceV1SendEvent,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches primary selection events.
#[derive(Debug)]
pub enum CommandEvent {
    /// Message [ZwpPrimarySelectionDeviceV1DataOfferEvent].
    ZwpPrimarySelectionDeviceV1DataOffer(ZwpPrimarySelectionDeviceV1DataOfferEvent),
    /// Message [ZwpPrimarySelectionDeviceV1SelectionEvent].
    ZwpPrimarySelectionDeviceV1Selection(ZwpPrimarySelectionDeviceV1SelectionEvent),
    /// Message [ZwpPrimarySelectionOfferV1OfferEvent].
    ZwpPrimarySelectionOfferV1Offer(ZwpPrimarySelectionOfferV1OfferEvent),
    /// Message [ZwpPrimarySelectionSourceV1SendEvent].
    ZwpPrimarySelectionSourceV1Send(ZwpPrimarySelectionSourceV1SendEvent),
    /// Message [ZwpPrimarySelectionSourceV1CancelledEvent].
    ZwpPrimarySelectionSourceV1Cancelled(ZwpPrimarySelectionSourceV1CancelledEvent),
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
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionDeviceV1 { .. },
            ) => match opcode {
                ZwpPrimarySelectionDeviceV1DataOfferEvent::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceV1DataOffer(
                        ZwpPrimarySelectionDeviceV1DataOfferEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionDeviceV1SelectionEvent::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceV1Selection(
                        ZwpPrimarySelectionDeviceV1SelectionEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionOfferV1 { .. },
            ) => match opcode {
                ZwpPrimarySelectionOfferV1OfferEvent::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionOfferV1Offer(
                        ZwpPrimarySelectionOfferV1OfferEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionSourceV1 { .. },
            ) => match opcode {
                ZwpPrimarySelectionSourceV1SendEvent::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionSourceV1Send(
                        ZwpPrimarySelectionSourceV1SendEvent::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionSourceV1CancelledEvent::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionSourceV1Cancelled(
                        ZwpPrimarySelectionSourceV1CancelledEvent::read_with_interface(
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
            Self::ZwpPrimarySelectionDeviceV1DataOffer(m) => m.object_id(),
            Self::ZwpPrimarySelectionDeviceV1Selection(m) => m.object_id(),
            Self::ZwpPrimarySelectionOfferV1Offer(m) => m.object_id(),
            Self::ZwpPrimarySelectionSourceV1Send(m) => m.object_id(),
            Self::ZwpPrimarySelectionSourceV1Cancelled(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpPrimarySelectionDeviceV1DataOffer(m) => m.opcode(),
            Self::ZwpPrimarySelectionDeviceV1Selection(m) => m.opcode(),
            Self::ZwpPrimarySelectionOfferV1Offer(m) => m.opcode(),
            Self::ZwpPrimarySelectionSourceV1Send(m) => m.opcode(),
            Self::ZwpPrimarySelectionSourceV1Cancelled(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpPrimarySelectionDeviceV1DataOffer(m) => m.payload_size(),
            Self::ZwpPrimarySelectionDeviceV1Selection(m) => m.payload_size(),
            Self::ZwpPrimarySelectionOfferV1Offer(m) => m.payload_size(),
            Self::ZwpPrimarySelectionSourceV1Send(m) => m.payload_size(),
            Self::ZwpPrimarySelectionSourceV1Cancelled(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpPrimarySelectionDeviceV1DataOffer(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionDeviceV1Selection(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionOfferV1Offer(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionSourceV1Send(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionSourceV1Cancelled(m) => m.write(writer, state).await,
        }
    }
}
