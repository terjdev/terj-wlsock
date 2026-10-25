use super::{
    ZwlrDataControlDeviceV1DestroyRequest, ZwlrDataControlDeviceV1SetPrimarySelectionRequest,
    ZwlrDataControlDeviceV1SetSelectionRequest, ZwlrDataControlManagerV1CreateDataSourceRequest,
    ZwlrDataControlManagerV1DestroyRequest, ZwlrDataControlManagerV1GetDataDeviceRequest,
    ZwlrDataControlOfferV1DestroyRequest, ZwlrDataControlOfferV1ReceiveRequest,
    ZwlrDataControlSourceV1DestroyRequest, ZwlrDataControlSourceV1OfferRequest,
};
use crate::protocol::wlr::data_control::ObjectInteface as WlrDataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR data control requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrDataControlManagerV1CreateDataSourceRequest].
    ZwlrDataControlManagerV1CreateDataSource(ZwlrDataControlManagerV1CreateDataSourceRequest),
    /// Message [ZwlrDataControlManagerV1GetDataDeviceRequest].
    ZwlrDataControlManagerV1GetDataDevice(ZwlrDataControlManagerV1GetDataDeviceRequest),
    /// Message [ZwlrDataControlManagerV1DestroyRequest].
    ZwlrDataControlManagerV1Destroy(ZwlrDataControlManagerV1DestroyRequest),
    /// Message [ZwlrDataControlDeviceV1SetSelectionRequest].
    ZwlrDataControlDeviceV1SetSelection(ZwlrDataControlDeviceV1SetSelectionRequest),
    /// Message [ZwlrDataControlDeviceV1DestroyRequest].
    ZwlrDataControlDeviceV1Destroy(ZwlrDataControlDeviceV1DestroyRequest),
    /// Message [ZwlrDataControlDeviceV1SetPrimarySelectionRequest].
    ZwlrDataControlDeviceV1SetPrimarySelection(ZwlrDataControlDeviceV1SetPrimarySelectionRequest),
    /// Message [ZwlrDataControlSourceV1OfferRequest].
    ZwlrDataControlSourceV1Offer(ZwlrDataControlSourceV1OfferRequest),
    /// Message [ZwlrDataControlSourceV1DestroyRequest].
    ZwlrDataControlSourceV1Destroy(ZwlrDataControlSourceV1DestroyRequest),
    /// Message [ZwlrDataControlOfferV1ReceiveRequest].
    ZwlrDataControlOfferV1Receive(ZwlrDataControlOfferV1ReceiveRequest),
    /// Message [ZwlrDataControlOfferV1DestroyRequest].
    ZwlrDataControlOfferV1Destroy(ZwlrDataControlOfferV1DestroyRequest),
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
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlManagerV1 { .. },
            ) => match opcode {
                ZwlrDataControlManagerV1CreateDataSourceRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlManagerV1CreateDataSource(
                        ZwlrDataControlManagerV1CreateDataSourceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlManagerV1GetDataDeviceRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlManagerV1GetDataDevice(
                        ZwlrDataControlManagerV1GetDataDeviceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlManagerV1Destroy(
                        ZwlrDataControlManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlDeviceV1 { .. },
            ) => match opcode {
                ZwlrDataControlDeviceV1SetSelectionRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1SetSelection(
                        ZwlrDataControlDeviceV1SetSelectionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlDeviceV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1Destroy(
                        ZwlrDataControlDeviceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlDeviceV1SetPrimarySelectionRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlDeviceV1SetPrimarySelection(
                        ZwlrDataControlDeviceV1SetPrimarySelectionRequest::read_with_interface(
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
                ZwlrDataControlSourceV1OfferRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlSourceV1Offer(
                        ZwlrDataControlSourceV1OfferRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlSourceV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlSourceV1Destroy(
                        ZwlrDataControlSourceV1DestroyRequest::read_with_interface(
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
                ZwlrDataControlOfferV1ReceiveRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlOfferV1Receive(
                        ZwlrDataControlOfferV1ReceiveRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ZwlrDataControlOfferV1DestroyRequest::OPCODE => {
                    Ok(Self::ZwlrDataControlOfferV1Destroy(
                        ZwlrDataControlOfferV1DestroyRequest::read_with_interface(
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
            Self::ZwlrDataControlManagerV1CreateDataSource(message) => message.object_id(),
            Self::ZwlrDataControlManagerV1GetDataDevice(message) => message.object_id(),
            Self::ZwlrDataControlManagerV1Destroy(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1SetSelection(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1Destroy(message) => message.object_id(),
            Self::ZwlrDataControlDeviceV1SetPrimarySelection(message) => message.object_id(),
            Self::ZwlrDataControlSourceV1Offer(message) => message.object_id(),
            Self::ZwlrDataControlSourceV1Destroy(message) => message.object_id(),
            Self::ZwlrDataControlOfferV1Receive(message) => message.object_id(),
            Self::ZwlrDataControlOfferV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrDataControlManagerV1CreateDataSource(message) => message.opcode(),
            Self::ZwlrDataControlManagerV1GetDataDevice(message) => message.opcode(),
            Self::ZwlrDataControlManagerV1Destroy(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1SetSelection(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1Destroy(message) => message.opcode(),
            Self::ZwlrDataControlDeviceV1SetPrimarySelection(message) => message.opcode(),
            Self::ZwlrDataControlSourceV1Offer(message) => message.opcode(),
            Self::ZwlrDataControlSourceV1Destroy(message) => message.opcode(),
            Self::ZwlrDataControlOfferV1Receive(message) => message.opcode(),
            Self::ZwlrDataControlOfferV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrDataControlManagerV1CreateDataSource(message) => message.payload_size(),
            Self::ZwlrDataControlManagerV1GetDataDevice(message) => message.payload_size(),
            Self::ZwlrDataControlManagerV1Destroy(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1SetSelection(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1Destroy(message) => message.payload_size(),
            Self::ZwlrDataControlDeviceV1SetPrimarySelection(message) => message.payload_size(),
            Self::ZwlrDataControlSourceV1Offer(message) => message.payload_size(),
            Self::ZwlrDataControlSourceV1Destroy(message) => message.payload_size(),
            Self::ZwlrDataControlOfferV1Receive(message) => message.payload_size(),
            Self::ZwlrDataControlOfferV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ZwlrDataControlManagerV1CreateDataSource(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrDataControlManagerV1GetDataDevice(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrDataControlManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrDataControlDeviceV1SetSelection(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrDataControlDeviceV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrDataControlDeviceV1SetPrimarySelection(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrDataControlSourceV1Offer(message) => message.write(writer, state).await,
            Self::ZwlrDataControlSourceV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrDataControlOfferV1Receive(message) => message.write(writer, state).await,
            Self::ZwlrDataControlOfferV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
