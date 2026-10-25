use super::{
    WpContentTypeManagerV1DestroyRequest, WpContentTypeManagerV1GetSurfaceContentTypeRequest,
    WpContentTypeV1DestroyRequest, WpContentTypeV1SetContentTypeRequest,
};
use crate::protocol::ext::content_type::ObjectInteface as ContentTypeObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Content type requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpContentTypeManagerV1DestroyRequest].
    WpContentTypeManagerV1Destroy(WpContentTypeManagerV1DestroyRequest),
    /// Message [WpContentTypeManagerV1GetSurfaceContentTypeRequest].
    WpContentTypeManagerV1GetSurfaceContentType(WpContentTypeManagerV1GetSurfaceContentTypeRequest),
    /// Message [WpContentTypeV1DestroyRequest].
    WpContentTypeV1Destroy(WpContentTypeV1DestroyRequest),
    /// Message [WpContentTypeV1SetContentTypeRequest].
    WpContentTypeV1SetContentType(WpContentTypeV1SetContentTypeRequest),
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
            ObjectInteface::ContentType(ContentTypeObjectInteface::WpContentTypeManagerV1 {
                ..
            }) => match opcode {
                WpContentTypeManagerV1DestroyRequest::OPCODE => {
                    Ok(Self::WpContentTypeManagerV1Destroy(
                        WpContentTypeManagerV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                WpContentTypeManagerV1GetSurfaceContentTypeRequest::OPCODE => {
                    Ok(Self::WpContentTypeManagerV1GetSurfaceContentType(
                        WpContentTypeManagerV1GetSurfaceContentTypeRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::ContentType(ContentTypeObjectInteface::WpContentTypeV1 { .. }) => {
                match opcode {
                    WpContentTypeV1DestroyRequest::OPCODE => Ok(Self::WpContentTypeV1Destroy(
                        WpContentTypeV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpContentTypeV1SetContentTypeRequest::OPCODE => {
                        Ok(Self::WpContentTypeV1SetContentType(
                            WpContentTypeV1SetContentTypeRequest::read_with_interface(
                                object_id, opcode, interface, reader, state,
                            )
                            .await?,
                        ))
                    }
                    _ => Err(FdReaderError::InvalidProtocol),
                }
            }
            _ => Err(FdReaderError::InvalidProtocol),
        }
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpContentTypeManagerV1Destroy(message) => message.object_id(),
            Self::WpContentTypeManagerV1GetSurfaceContentType(message) => message.object_id(),
            Self::WpContentTypeV1Destroy(message) => message.object_id(),
            Self::WpContentTypeV1SetContentType(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpContentTypeManagerV1Destroy(message) => message.opcode(),
            Self::WpContentTypeManagerV1GetSurfaceContentType(message) => message.opcode(),
            Self::WpContentTypeV1Destroy(message) => message.opcode(),
            Self::WpContentTypeV1SetContentType(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpContentTypeManagerV1Destroy(message) => message.payload_size(),
            Self::WpContentTypeManagerV1GetSurfaceContentType(message) => message.payload_size(),
            Self::WpContentTypeV1Destroy(message) => message.payload_size(),
            Self::WpContentTypeV1SetContentType(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpContentTypeManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpContentTypeManagerV1GetSurfaceContentType(message) => {
                message.write(writer, state).await
            }
            Self::WpContentTypeV1Destroy(message) => message.write(writer, state).await,
            Self::WpContentTypeV1SetContentType(message) => message.write(writer, state).await,
        }
    }
}
