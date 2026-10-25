use super::{
    WpCursorShapeDeviceV1DestroyRequest, WpCursorShapeDeviceV1SetShapeRequest,
    WpCursorShapeManagerV1DestroyRequest, WpCursorShapeManagerV1GetPointerRequest,
    WpCursorShapeManagerV1GetTabletToolV2Request,
};
use crate::protocol::ext::cursor_shape::ObjectInteface as CursorShapeObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Cursor shape requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpCursorShapeManagerV1DestroyRequest].
    WpCursorShapeManagerV1Destroy(WpCursorShapeManagerV1DestroyRequest),
    /// Message [WpCursorShapeManagerV1GetPointerRequest].
    WpCursorShapeManagerV1GetPointer(WpCursorShapeManagerV1GetPointerRequest),
    /// Message [WpCursorShapeManagerV1GetTabletToolV2Request].
    WpCursorShapeManagerV1GetTabletToolV2(WpCursorShapeManagerV1GetTabletToolV2Request),
    /// Message [WpCursorShapeDeviceV1DestroyRequest].
    WpCursorShapeDeviceV1Destroy(WpCursorShapeDeviceV1DestroyRequest),
    /// Message [WpCursorShapeDeviceV1SetShapeRequest].
    WpCursorShapeDeviceV1SetShape(WpCursorShapeDeviceV1SetShapeRequest),
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
            ObjectInteface::CursorShape(CursorShapeObjectInteface::WpCursorShapeManagerV1 {
                ..
            }) => match opcode {
                WpCursorShapeManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpCursorShapeManagerV1Destroy(
                        WpCursorShapeManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpCursorShapeManagerV1GetPointerRequest::OPCODE => {
                    Ok(Self::WpCursorShapeManagerV1GetPointer(
                        WpCursorShapeManagerV1GetPointerRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpCursorShapeManagerV1GetTabletToolV2Request::OPCODE => {
                    Ok(Self::WpCursorShapeManagerV1GetTabletToolV2(
                        WpCursorShapeManagerV1GetTabletToolV2Request::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::CursorShape(CursorShapeObjectInteface::WpCursorShapeDeviceV1 {
                ..
            }) => match opcode {
                WpCursorShapeDeviceV1DestroyRequest::OPCODE => {
                    Ok(Self::WpCursorShapeDeviceV1Destroy(
                        WpCursorShapeDeviceV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpCursorShapeDeviceV1SetShapeRequest::OPCODE => {
                    Ok(Self::WpCursorShapeDeviceV1SetShape(
                        WpCursorShapeDeviceV1SetShapeRequest::read_with_interface(
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
            Self::WpCursorShapeManagerV1Destroy(message) => message.object_id(),
            Self::WpCursorShapeManagerV1GetPointer(message) => message.object_id(),
            Self::WpCursorShapeManagerV1GetTabletToolV2(message) => message.object_id(),
            Self::WpCursorShapeDeviceV1Destroy(message) => message.object_id(),
            Self::WpCursorShapeDeviceV1SetShape(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpCursorShapeManagerV1Destroy(message) => message.opcode(),
            Self::WpCursorShapeManagerV1GetPointer(message) => message.opcode(),
            Self::WpCursorShapeManagerV1GetTabletToolV2(message) => message.opcode(),
            Self::WpCursorShapeDeviceV1Destroy(message) => message.opcode(),
            Self::WpCursorShapeDeviceV1SetShape(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpCursorShapeManagerV1Destroy(message) => message.payload_size(),
            Self::WpCursorShapeManagerV1GetPointer(message) => message.payload_size(),
            Self::WpCursorShapeManagerV1GetTabletToolV2(message) => message.payload_size(),
            Self::WpCursorShapeDeviceV1Destroy(message) => message.payload_size(),
            Self::WpCursorShapeDeviceV1SetShape(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpCursorShapeManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpCursorShapeManagerV1GetPointer(message) => message.write(writer, state).await,
            Self::WpCursorShapeManagerV1GetTabletToolV2(message) => {
                message.write(writer, state).await
            }
            Self::WpCursorShapeDeviceV1Destroy(message) => message.write(writer, state).await,
            Self::WpCursorShapeDeviceV1SetShape(message) => message.write(writer, state).await,
        }
    }
}
