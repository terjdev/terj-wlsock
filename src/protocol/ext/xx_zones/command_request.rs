use super::ObjectInteface as XxZonesObjectInteface;
use super::{
    XxZoneItemV1DestroyRequest, XxZoneItemV1SetPositionRequest, XxZoneManagerV1DestroyRequest,
    XxZoneManagerV1GetZoneFromHandleRequest, XxZoneManagerV1GetZoneItemRequest,
    XxZoneManagerV1GetZoneRequest, XxZoneV1AddItemRequest, XxZoneV1DestroyRequest,
    XxZoneV1RemoveItemRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches zones requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxZoneManagerV1DestroyRequest].
    XxZoneManagerV1Destroy(XxZoneManagerV1DestroyRequest),
    /// Message [XxZoneManagerV1GetZoneItemRequest].
    XxZoneManagerV1GetZoneItem(XxZoneManagerV1GetZoneItemRequest),
    /// Message [XxZoneManagerV1GetZoneRequest].
    XxZoneManagerV1GetZone(XxZoneManagerV1GetZoneRequest),
    /// Message [XxZoneManagerV1GetZoneFromHandleRequest].
    XxZoneManagerV1GetZoneFromHandle(XxZoneManagerV1GetZoneFromHandleRequest),
    /// Message [XxZoneItemV1DestroyRequest].
    XxZoneItemV1Destroy(XxZoneItemV1DestroyRequest),
    /// Message [XxZoneItemV1SetPositionRequest].
    XxZoneItemV1SetPosition(XxZoneItemV1SetPositionRequest),
    /// Message [XxZoneV1DestroyRequest].
    XxZoneV1Destroy(XxZoneV1DestroyRequest),
    /// Message [XxZoneV1AddItemRequest].
    XxZoneV1AddItem(XxZoneV1AddItemRequest),
    /// Message [XxZoneV1RemoveItemRequest].
    XxZoneV1RemoveItem(XxZoneV1RemoveItemRequest),
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
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneManagerV1 { .. }) => {
                match opcode {
                    XxZoneManagerV1DestroyRequest::OPCODE => Ok(Self::XxZoneManagerV1Destroy(
                        XxZoneManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxZoneManagerV1GetZoneItemRequest::OPCODE => {
                        Ok(Self::XxZoneManagerV1GetZoneItem(
                            XxZoneManagerV1GetZoneItemRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    XxZoneManagerV1GetZoneRequest::OPCODE => Ok(Self::XxZoneManagerV1GetZone(
                        XxZoneManagerV1GetZoneRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    XxZoneManagerV1GetZoneFromHandleRequest::OPCODE => {
                        Ok(Self::XxZoneManagerV1GetZoneFromHandle(
                            XxZoneManagerV1GetZoneFromHandleRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneItemV1 { .. }) => match opcode {
                XxZoneItemV1DestroyRequest::OPCODE => Ok(Self::XxZoneItemV1Destroy(
                    XxZoneItemV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneItemV1SetPositionRequest::OPCODE => Ok(Self::XxZoneItemV1SetPosition(
                    XxZoneItemV1SetPositionRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 { .. }) => match opcode {
                XxZoneV1DestroyRequest::OPCODE => Ok(Self::XxZoneV1Destroy(
                    XxZoneV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1AddItemRequest::OPCODE => Ok(Self::XxZoneV1AddItem(
                    XxZoneV1AddItemRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XxZoneV1RemoveItemRequest::OPCODE => Ok(Self::XxZoneV1RemoveItem(
                    XxZoneV1RemoveItemRequest::read_with_interface(
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
            Self::XxZoneManagerV1Destroy(m) => m.object_id(),
            Self::XxZoneManagerV1GetZoneItem(m) => m.object_id(),
            Self::XxZoneManagerV1GetZone(m) => m.object_id(),
            Self::XxZoneManagerV1GetZoneFromHandle(m) => m.object_id(),
            Self::XxZoneItemV1Destroy(m) => m.object_id(),
            Self::XxZoneItemV1SetPosition(m) => m.object_id(),
            Self::XxZoneV1Destroy(m) => m.object_id(),
            Self::XxZoneV1AddItem(m) => m.object_id(),
            Self::XxZoneV1RemoveItem(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxZoneManagerV1Destroy(m) => m.opcode(),
            Self::XxZoneManagerV1GetZoneItem(m) => m.opcode(),
            Self::XxZoneManagerV1GetZone(m) => m.opcode(),
            Self::XxZoneManagerV1GetZoneFromHandle(m) => m.opcode(),
            Self::XxZoneItemV1Destroy(m) => m.opcode(),
            Self::XxZoneItemV1SetPosition(m) => m.opcode(),
            Self::XxZoneV1Destroy(m) => m.opcode(),
            Self::XxZoneV1AddItem(m) => m.opcode(),
            Self::XxZoneV1RemoveItem(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxZoneManagerV1Destroy(m) => m.payload_size(),
            Self::XxZoneManagerV1GetZoneItem(m) => m.payload_size(),
            Self::XxZoneManagerV1GetZone(m) => m.payload_size(),
            Self::XxZoneManagerV1GetZoneFromHandle(m) => m.payload_size(),
            Self::XxZoneItemV1Destroy(m) => m.payload_size(),
            Self::XxZoneItemV1SetPosition(m) => m.payload_size(),
            Self::XxZoneV1Destroy(m) => m.payload_size(),
            Self::XxZoneV1AddItem(m) => m.payload_size(),
            Self::XxZoneV1RemoveItem(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxZoneManagerV1Destroy(m) => m.write(writer, state).await,
            Self::XxZoneManagerV1GetZoneItem(m) => m.write(writer, state).await,
            Self::XxZoneManagerV1GetZone(m) => m.write(writer, state).await,
            Self::XxZoneManagerV1GetZoneFromHandle(m) => m.write(writer, state).await,
            Self::XxZoneItemV1Destroy(m) => m.write(writer, state).await,
            Self::XxZoneItemV1SetPosition(m) => m.write(writer, state).await,
            Self::XxZoneV1Destroy(m) => m.write(writer, state).await,
            Self::XxZoneV1AddItem(m) => m.write(writer, state).await,
            Self::XxZoneV1RemoveItem(m) => m.write(writer, state).await,
        }
    }
}
