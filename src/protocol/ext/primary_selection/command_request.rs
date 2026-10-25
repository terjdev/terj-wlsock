use super::ObjectInteface as PrimarySelectionObjectInteface;
use super::{
    ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest,
    ZwpPrimarySelectionDeviceManagerV1DestroyRequest,
    ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest, ZwpPrimarySelectionDeviceV1DestroyRequest,
    ZwpPrimarySelectionDeviceV1SetSelectionRequest, ZwpPrimarySelectionOfferV1DestroyRequest,
    ZwpPrimarySelectionOfferV1ReceiveRequest, ZwpPrimarySelectionSourceV1DestroyRequest,
    ZwpPrimarySelectionSourceV1OfferRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches primary selection requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest].
    ZwpPrimarySelectionDeviceManagerV1CreateSource(
        ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest,
    ),
    /// Message [ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest].
    ZwpPrimarySelectionDeviceManagerV1GetDevice(ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest),
    /// Message [ZwpPrimarySelectionDeviceManagerV1DestroyRequest].
    ZwpPrimarySelectionDeviceManagerV1Destroy(ZwpPrimarySelectionDeviceManagerV1DestroyRequest),
    /// Message [ZwpPrimarySelectionDeviceV1SetSelectionRequest].
    ZwpPrimarySelectionDeviceV1SetSelection(ZwpPrimarySelectionDeviceV1SetSelectionRequest),
    /// Message [ZwpPrimarySelectionDeviceV1DestroyRequest].
    ZwpPrimarySelectionDeviceV1Destroy(ZwpPrimarySelectionDeviceV1DestroyRequest),
    /// Message [ZwpPrimarySelectionOfferV1ReceiveRequest].
    ZwpPrimarySelectionOfferV1Receive(ZwpPrimarySelectionOfferV1ReceiveRequest),
    /// Message [ZwpPrimarySelectionOfferV1DestroyRequest].
    ZwpPrimarySelectionOfferV1Destroy(ZwpPrimarySelectionOfferV1DestroyRequest),
    /// Message [ZwpPrimarySelectionSourceV1OfferRequest].
    ZwpPrimarySelectionSourceV1Offer(ZwpPrimarySelectionSourceV1OfferRequest),
    /// Message [ZwpPrimarySelectionSourceV1DestroyRequest].
    ZwpPrimarySelectionSourceV1Destroy(ZwpPrimarySelectionSourceV1DestroyRequest),
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
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionDeviceManagerV1 { .. },
            ) => match opcode {
                ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceManagerV1CreateSource(
                        ZwpPrimarySelectionDeviceManagerV1CreateSourceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceManagerV1GetDevice(
                        ZwpPrimarySelectionDeviceManagerV1GetDeviceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionDeviceManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceManagerV1Destroy(
                        ZwpPrimarySelectionDeviceManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::PrimarySelection(
                PrimarySelectionObjectInteface::ZwpPrimarySelectionDeviceV1 { .. },
            ) => match opcode {
                ZwpPrimarySelectionDeviceV1SetSelectionRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceV1SetSelection(
                        ZwpPrimarySelectionDeviceV1SetSelectionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionDeviceV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionDeviceV1Destroy(
                        ZwpPrimarySelectionDeviceV1DestroyRequest::read_with_interface(
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
                ZwpPrimarySelectionOfferV1ReceiveRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionOfferV1Receive(
                        ZwpPrimarySelectionOfferV1ReceiveRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionOfferV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionOfferV1Destroy(
                        ZwpPrimarySelectionOfferV1DestroyRequest::read_with_interface(
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
                ZwpPrimarySelectionSourceV1OfferRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionSourceV1Offer(
                        ZwpPrimarySelectionSourceV1OfferRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwpPrimarySelectionSourceV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwpPrimarySelectionSourceV1Destroy(
                        ZwpPrimarySelectionSourceV1DestroyRequest::read_with_interface(
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
            Self::ZwpPrimarySelectionDeviceManagerV1CreateSource(m) => m.object_id(),
            Self::ZwpPrimarySelectionDeviceManagerV1GetDevice(m) => m.object_id(),
            Self::ZwpPrimarySelectionDeviceManagerV1Destroy(m) => m.object_id(),
            Self::ZwpPrimarySelectionDeviceV1SetSelection(m) => m.object_id(),
            Self::ZwpPrimarySelectionDeviceV1Destroy(m) => m.object_id(),
            Self::ZwpPrimarySelectionOfferV1Receive(m) => m.object_id(),
            Self::ZwpPrimarySelectionOfferV1Destroy(m) => m.object_id(),
            Self::ZwpPrimarySelectionSourceV1Offer(m) => m.object_id(),
            Self::ZwpPrimarySelectionSourceV1Destroy(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1CreateSource(m) => m.opcode(),
            Self::ZwpPrimarySelectionDeviceManagerV1GetDevice(m) => m.opcode(),
            Self::ZwpPrimarySelectionDeviceManagerV1Destroy(m) => m.opcode(),
            Self::ZwpPrimarySelectionDeviceV1SetSelection(m) => m.opcode(),
            Self::ZwpPrimarySelectionDeviceV1Destroy(m) => m.opcode(),
            Self::ZwpPrimarySelectionOfferV1Receive(m) => m.opcode(),
            Self::ZwpPrimarySelectionOfferV1Destroy(m) => m.opcode(),
            Self::ZwpPrimarySelectionSourceV1Offer(m) => m.opcode(),
            Self::ZwpPrimarySelectionSourceV1Destroy(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1CreateSource(m) => m.payload_size(),
            Self::ZwpPrimarySelectionDeviceManagerV1GetDevice(m) => m.payload_size(),
            Self::ZwpPrimarySelectionDeviceManagerV1Destroy(m) => m.payload_size(),
            Self::ZwpPrimarySelectionDeviceV1SetSelection(m) => m.payload_size(),
            Self::ZwpPrimarySelectionDeviceV1Destroy(m) => m.payload_size(),
            Self::ZwpPrimarySelectionOfferV1Receive(m) => m.payload_size(),
            Self::ZwpPrimarySelectionOfferV1Destroy(m) => m.payload_size(),
            Self::ZwpPrimarySelectionSourceV1Offer(m) => m.payload_size(),
            Self::ZwpPrimarySelectionSourceV1Destroy(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwpPrimarySelectionDeviceManagerV1CreateSource(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionDeviceManagerV1GetDevice(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionDeviceManagerV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionDeviceV1SetSelection(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionDeviceV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionOfferV1Receive(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionOfferV1Destroy(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionSourceV1Offer(m) => m.write(writer, state).await,
            Self::ZwpPrimarySelectionSourceV1Destroy(m) => m.write(writer, state).await,
        }
    }
}
