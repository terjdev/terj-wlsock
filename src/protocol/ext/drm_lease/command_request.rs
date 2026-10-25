use super::{
    WpDrmLeaseConnectorV1DestroyRequest, WpDrmLeaseDeviceV1CreateLeaseRequestRequest,
    WpDrmLeaseDeviceV1ReleaseRequest, WpDrmLeaseRequestV1RequestConnectorRequest,
    WpDrmLeaseRequestV1SubmitRequest, WpDrmLeaseV1DestroyRequest,
};
use crate::protocol::ext::drm_lease::ObjectInteface as DrmLeaseObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches DRM lease requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpDrmLeaseDeviceV1CreateLeaseRequestRequest].
    WpDrmLeaseDeviceV1CreateLeaseRequest(WpDrmLeaseDeviceV1CreateLeaseRequestRequest),
    /// Message [WpDrmLeaseDeviceV1ReleaseRequest].
    WpDrmLeaseDeviceV1Release(WpDrmLeaseDeviceV1ReleaseRequest),
    /// Message [WpDrmLeaseConnectorV1DestroyRequest].
    WpDrmLeaseConnectorV1Destroy(WpDrmLeaseConnectorV1DestroyRequest),
    /// Message [WpDrmLeaseRequestV1RequestConnectorRequest].
    WpDrmLeaseRequestV1RequestConnector(WpDrmLeaseRequestV1RequestConnectorRequest),
    /// Message [WpDrmLeaseRequestV1SubmitRequest].
    WpDrmLeaseRequestV1Submit(WpDrmLeaseRequestV1SubmitRequest),
    /// Message [WpDrmLeaseV1DestroyRequest].
    WpDrmLeaseV1Destroy(WpDrmLeaseV1DestroyRequest),
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
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseDeviceV1 { .. }) => {
                match opcode {
                    WpDrmLeaseDeviceV1CreateLeaseRequestRequest::OPCODE => {
                        Ok(Self::WpDrmLeaseDeviceV1CreateLeaseRequest(
                            WpDrmLeaseDeviceV1CreateLeaseRequestRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    WpDrmLeaseDeviceV1ReleaseRequest::OPCODE => {
                        Ok(Self::WpDrmLeaseDeviceV1Release(
                            WpDrmLeaseDeviceV1ReleaseRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseConnectorV1 { .. }) => {
                match opcode {
                    WpDrmLeaseConnectorV1DestroyRequest::OPCODE => {
                        Ok(Self::WpDrmLeaseConnectorV1Destroy(
                            WpDrmLeaseConnectorV1DestroyRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseRequestV1 { .. }) => {
                match opcode {
                    WpDrmLeaseRequestV1RequestConnectorRequest::OPCODE => {
                        Ok(Self::WpDrmLeaseRequestV1RequestConnector(
                            WpDrmLeaseRequestV1RequestConnectorRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    WpDrmLeaseRequestV1SubmitRequest::OPCODE => {
                        Ok(Self::WpDrmLeaseRequestV1Submit(
                            WpDrmLeaseRequestV1SubmitRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::DrmLease(DrmLeaseObjectInteface::WpDrmLeaseV1 { .. }) => match opcode {
                WpDrmLeaseV1DestroyRequest::OPCODE => Ok(Self::WpDrmLeaseV1Destroy(
                    WpDrmLeaseV1DestroyRequest::read_with_interface(
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
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpDrmLeaseDeviceV1CreateLeaseRequest(message) => message.object_id(),
            Self::WpDrmLeaseDeviceV1Release(message) => message.object_id(),
            Self::WpDrmLeaseConnectorV1Destroy(message) => message.object_id(),
            Self::WpDrmLeaseRequestV1RequestConnector(message) => message.object_id(),
            Self::WpDrmLeaseRequestV1Submit(message) => message.object_id(),
            Self::WpDrmLeaseV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpDrmLeaseDeviceV1CreateLeaseRequest(message) => message.opcode(),
            Self::WpDrmLeaseDeviceV1Release(message) => message.opcode(),
            Self::WpDrmLeaseConnectorV1Destroy(message) => message.opcode(),
            Self::WpDrmLeaseRequestV1RequestConnector(message) => message.opcode(),
            Self::WpDrmLeaseRequestV1Submit(message) => message.opcode(),
            Self::WpDrmLeaseV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpDrmLeaseDeviceV1CreateLeaseRequest(message) => message.payload_size(),
            Self::WpDrmLeaseDeviceV1Release(message) => message.payload_size(),
            Self::WpDrmLeaseConnectorV1Destroy(message) => message.payload_size(),
            Self::WpDrmLeaseRequestV1RequestConnector(message) => message.payload_size(),
            Self::WpDrmLeaseRequestV1Submit(message) => message.payload_size(),
            Self::WpDrmLeaseV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpDrmLeaseDeviceV1CreateLeaseRequest(message) => {
                message.write(writer, state).await
            }
            Self::WpDrmLeaseDeviceV1Release(message) => message.write(writer, state).await,
            Self::WpDrmLeaseConnectorV1Destroy(message) => message.write(writer, state).await,
            Self::WpDrmLeaseRequestV1RequestConnector(message) => {
                message.write(writer, state).await
            }
            Self::WpDrmLeaseRequestV1Submit(message) => message.write(writer, state).await,
            Self::WpDrmLeaseV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
