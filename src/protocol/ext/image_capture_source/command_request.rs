use super::{
    ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest,
    ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest,
    ExtImageCaptureSourceV1DestroyRequest, ExtOutputImageCaptureSourceManagerV1CreateSourceRequest,
    ExtOutputImageCaptureSourceManagerV1DestroyRequest,
};
use crate::protocol::ext::image_capture_source::ObjectInteface as ImageCaptureSourceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Image capture source requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtImageCaptureSourceV1DestroyRequest].
    ExtImageCaptureSourceV1Destroy(ExtImageCaptureSourceV1DestroyRequest),
    /// Message [ExtOutputImageCaptureSourceManagerV1CreateSourceRequest].
    ExtOutputImageCaptureSourceManagerV1CreateSource(
        ExtOutputImageCaptureSourceManagerV1CreateSourceRequest,
    ),
    /// Message [ExtOutputImageCaptureSourceManagerV1DestroyRequest].
    ExtOutputImageCaptureSourceManagerV1Destroy(ExtOutputImageCaptureSourceManagerV1DestroyRequest),
    /// Message [ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest].
    ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(
        ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest,
    ),
    /// Message [ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest].
    ExtForeignToplevelImageCaptureSourceManagerV1Destroy(
        ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest,
    ),
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
ObjectInteface::ImageCaptureSource(ImageCaptureSourceObjectInteface::ExtImageCaptureSourceV1{..})=>match opcode {
ExtImageCaptureSourceV1DestroyRequest::OPCODE=>Ok(Self::ExtImageCaptureSourceV1Destroy(ExtImageCaptureSourceV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ImageCaptureSource(ImageCaptureSourceObjectInteface::ExtOutputImageCaptureSourceManagerV1{..})=>match opcode {
ExtOutputImageCaptureSourceManagerV1CreateSourceRequest::OPCODE=>Ok(Self::ExtOutputImageCaptureSourceManagerV1CreateSource(ExtOutputImageCaptureSourceManagerV1CreateSourceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtOutputImageCaptureSourceManagerV1DestroyRequest::OPCODE=>Ok(Self::ExtOutputImageCaptureSourceManagerV1Destroy(ExtOutputImageCaptureSourceManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ImageCaptureSource(ImageCaptureSourceObjectInteface::ExtForeignToplevelImageCaptureSourceManagerV1{..})=>match opcode {
ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest::OPCODE=>Ok(Self::ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest::OPCODE=>Ok(Self::ExtForeignToplevelImageCaptureSourceManagerV1Destroy(ExtForeignToplevelImageCaptureSourceManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtImageCaptureSourceV1Destroy(message) => message.object_id(),
            Self::ExtOutputImageCaptureSourceManagerV1CreateSource(message) => message.object_id(),
            Self::ExtOutputImageCaptureSourceManagerV1Destroy(message) => message.object_id(),
            Self::ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(message) => {
                message.object_id()
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1Destroy(message) => {
                message.object_id()
            }
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtImageCaptureSourceV1Destroy(message) => message.opcode(),
            Self::ExtOutputImageCaptureSourceManagerV1CreateSource(message) => message.opcode(),
            Self::ExtOutputImageCaptureSourceManagerV1Destroy(message) => message.opcode(),
            Self::ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(message) => {
                message.opcode()
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtImageCaptureSourceV1Destroy(message) => message.payload_size(),
            Self::ExtOutputImageCaptureSourceManagerV1CreateSource(message) => {
                message.payload_size()
            }
            Self::ExtOutputImageCaptureSourceManagerV1Destroy(message) => message.payload_size(),
            Self::ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(message) => {
                message.payload_size()
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1Destroy(message) => {
                message.payload_size()
            }
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtImageCaptureSourceV1Destroy(message) => message.write(writer, state).await,
            Self::ExtOutputImageCaptureSourceManagerV1CreateSource(message) => {
                message.write(writer, state).await
            }
            Self::ExtOutputImageCaptureSourceManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1CreateSource(message) => {
                message.write(writer, state).await
            }
            Self::ExtForeignToplevelImageCaptureSourceManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
        }
    }
}
