use super::{
    ExtDataControlDeviceV1DestroyRequest, ExtDataControlDeviceV1SetPrimarySelectionRequest,
    ExtDataControlDeviceV1SetSelectionRequest, ExtDataControlManagerV1CreateDataSourceRequest,
    ExtDataControlManagerV1DestroyRequest, ExtDataControlManagerV1GetDataDeviceRequest,
    ExtDataControlOfferV1DestroyRequest, ExtDataControlOfferV1ReceiveRequest,
    ExtDataControlSourceV1DestroyRequest, ExtDataControlSourceV1OfferRequest,
};
use crate::protocol::ext::data_control::ObjectInteface as DataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Data control requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtDataControlManagerV1CreateDataSourceRequest].
    ExtDataControlManagerV1CreateDataSource(ExtDataControlManagerV1CreateDataSourceRequest),
    /// Message [ExtDataControlManagerV1GetDataDeviceRequest].
    ExtDataControlManagerV1GetDataDevice(ExtDataControlManagerV1GetDataDeviceRequest),
    /// Message [ExtDataControlManagerV1DestroyRequest].
    ExtDataControlManagerV1Destroy(ExtDataControlManagerV1DestroyRequest),
    /// Message [ExtDataControlDeviceV1SetSelectionRequest].
    ExtDataControlDeviceV1SetSelection(ExtDataControlDeviceV1SetSelectionRequest),
    /// Message [ExtDataControlDeviceV1DestroyRequest].
    ExtDataControlDeviceV1Destroy(ExtDataControlDeviceV1DestroyRequest),
    /// Message [ExtDataControlDeviceV1SetPrimarySelectionRequest].
    ExtDataControlDeviceV1SetPrimarySelection(ExtDataControlDeviceV1SetPrimarySelectionRequest),
    /// Message [ExtDataControlSourceV1OfferRequest].
    ExtDataControlSourceV1Offer(ExtDataControlSourceV1OfferRequest),
    /// Message [ExtDataControlSourceV1DestroyRequest].
    ExtDataControlSourceV1Destroy(ExtDataControlSourceV1DestroyRequest),
    /// Message [ExtDataControlOfferV1ReceiveRequest].
    ExtDataControlOfferV1Receive(ExtDataControlOfferV1ReceiveRequest),
    /// Message [ExtDataControlOfferV1DestroyRequest].
    ExtDataControlOfferV1Destroy(ExtDataControlOfferV1DestroyRequest),
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
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlManagerV1 {
                ..
            }) => match opcode {
                ExtDataControlManagerV1CreateDataSourceRequest::OPCODE => {
                    Ok(Self::ExtDataControlManagerV1CreateDataSource(
                        ExtDataControlManagerV1CreateDataSourceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlManagerV1GetDataDeviceRequest::OPCODE => {
                    Ok(Self::ExtDataControlManagerV1GetDataDevice(
                        ExtDataControlManagerV1GetDataDeviceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtDataControlManagerV1Destroy(
                        ExtDataControlManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlDeviceV1 {
                ..
            }) => match opcode {
                ExtDataControlDeviceV1SetSelectionRequest::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1SetSelection(
                        ExtDataControlDeviceV1SetSelectionRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlDeviceV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1Destroy(
                        ExtDataControlDeviceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlDeviceV1SetPrimarySelectionRequest::OPCODE => {
                    Ok(Self::ExtDataControlDeviceV1SetPrimarySelection(
                        ExtDataControlDeviceV1SetPrimarySelectionRequest::read_with_interface(
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
                ExtDataControlSourceV1OfferRequest::OPCODE => {
                    Ok(Self::ExtDataControlSourceV1Offer(
                        ExtDataControlSourceV1OfferRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlSourceV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtDataControlSourceV1Destroy(
                        ExtDataControlSourceV1DestroyRequest::read_with_interface(
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
                ExtDataControlOfferV1ReceiveRequest::OPCODE => {
                    Ok(Self::ExtDataControlOfferV1Receive(
                        ExtDataControlOfferV1ReceiveRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                ExtDataControlOfferV1DestroyRequest::OPCODE => {
                    Ok(Self::ExtDataControlOfferV1Destroy(
                        ExtDataControlOfferV1DestroyRequest::read_with_interface(
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
            Self::ExtDataControlManagerV1CreateDataSource(message) => message.object_id(),
            Self::ExtDataControlManagerV1GetDataDevice(message) => message.object_id(),
            Self::ExtDataControlManagerV1Destroy(message) => message.object_id(),
            Self::ExtDataControlDeviceV1SetSelection(message) => message.object_id(),
            Self::ExtDataControlDeviceV1Destroy(message) => message.object_id(),
            Self::ExtDataControlDeviceV1SetPrimarySelection(message) => message.object_id(),
            Self::ExtDataControlSourceV1Offer(message) => message.object_id(),
            Self::ExtDataControlSourceV1Destroy(message) => message.object_id(),
            Self::ExtDataControlOfferV1Receive(message) => message.object_id(),
            Self::ExtDataControlOfferV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtDataControlManagerV1CreateDataSource(message) => message.opcode(),
            Self::ExtDataControlManagerV1GetDataDevice(message) => message.opcode(),
            Self::ExtDataControlManagerV1Destroy(message) => message.opcode(),
            Self::ExtDataControlDeviceV1SetSelection(message) => message.opcode(),
            Self::ExtDataControlDeviceV1Destroy(message) => message.opcode(),
            Self::ExtDataControlDeviceV1SetPrimarySelection(message) => message.opcode(),
            Self::ExtDataControlSourceV1Offer(message) => message.opcode(),
            Self::ExtDataControlSourceV1Destroy(message) => message.opcode(),
            Self::ExtDataControlOfferV1Receive(message) => message.opcode(),
            Self::ExtDataControlOfferV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtDataControlManagerV1CreateDataSource(message) => message.payload_size(),
            Self::ExtDataControlManagerV1GetDataDevice(message) => message.payload_size(),
            Self::ExtDataControlManagerV1Destroy(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1SetSelection(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1Destroy(message) => message.payload_size(),
            Self::ExtDataControlDeviceV1SetPrimarySelection(message) => message.payload_size(),
            Self::ExtDataControlSourceV1Offer(message) => message.payload_size(),
            Self::ExtDataControlSourceV1Destroy(message) => message.payload_size(),
            Self::ExtDataControlOfferV1Receive(message) => message.payload_size(),
            Self::ExtDataControlOfferV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtDataControlManagerV1CreateDataSource(message) => {
                message.write(writer, state).await
            }
            Self::ExtDataControlManagerV1GetDataDevice(message) => {
                message.write(writer, state).await
            }
            Self::ExtDataControlManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1SetSelection(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1Destroy(message) => message.write(writer, state).await,
            Self::ExtDataControlDeviceV1SetPrimarySelection(message) => {
                message.write(writer, state).await
            }
            Self::ExtDataControlSourceV1Offer(message) => message.write(writer, state).await,
            Self::ExtDataControlSourceV1Destroy(message) => message.write(writer, state).await,
            Self::ExtDataControlOfferV1Receive(message) => message.write(writer, state).await,
            Self::ExtDataControlOfferV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
