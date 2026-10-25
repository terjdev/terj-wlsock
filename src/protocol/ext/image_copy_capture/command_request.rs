use super::{
    ExtImageCopyCaptureCursorSessionV1DestroyRequest,
    ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest,
    ExtImageCopyCaptureFrameV1AttachBufferRequest, ExtImageCopyCaptureFrameV1CaptureRequest,
    ExtImageCopyCaptureFrameV1DamageBufferRequest, ExtImageCopyCaptureFrameV1DestroyRequest,
    ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest,
    ExtImageCopyCaptureManagerV1CreateSessionRequest, ExtImageCopyCaptureManagerV1DestroyRequest,
    ExtImageCopyCaptureSessionV1CreateFrameRequest, ExtImageCopyCaptureSessionV1DestroyRequest,
};
use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Image copy capture requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtImageCopyCaptureManagerV1CreateSessionRequest].
    ExtImageCopyCaptureManagerV1CreateSession(ExtImageCopyCaptureManagerV1CreateSessionRequest),
    /// Message [ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest].
    ExtImageCopyCaptureManagerV1CreatePointerCursorSession(
        ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest,
    ),
    /// Message [ExtImageCopyCaptureManagerV1DestroyRequest].
    ExtImageCopyCaptureManagerV1Destroy(ExtImageCopyCaptureManagerV1DestroyRequest),
    /// Message [ExtImageCopyCaptureSessionV1CreateFrameRequest].
    ExtImageCopyCaptureSessionV1CreateFrame(ExtImageCopyCaptureSessionV1CreateFrameRequest),
    /// Message [ExtImageCopyCaptureSessionV1DestroyRequest].
    ExtImageCopyCaptureSessionV1Destroy(ExtImageCopyCaptureSessionV1DestroyRequest),
    /// Message [ExtImageCopyCaptureFrameV1DestroyRequest].
    ExtImageCopyCaptureFrameV1Destroy(ExtImageCopyCaptureFrameV1DestroyRequest),
    /// Message [ExtImageCopyCaptureFrameV1AttachBufferRequest].
    ExtImageCopyCaptureFrameV1AttachBuffer(ExtImageCopyCaptureFrameV1AttachBufferRequest),
    /// Message [ExtImageCopyCaptureFrameV1DamageBufferRequest].
    ExtImageCopyCaptureFrameV1DamageBuffer(ExtImageCopyCaptureFrameV1DamageBufferRequest),
    /// Message [ExtImageCopyCaptureFrameV1CaptureRequest].
    ExtImageCopyCaptureFrameV1Capture(ExtImageCopyCaptureFrameV1CaptureRequest),
    /// Message [ExtImageCopyCaptureCursorSessionV1DestroyRequest].
    ExtImageCopyCaptureCursorSessionV1Destroy(ExtImageCopyCaptureCursorSessionV1DestroyRequest),
    /// Message [ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest].
    ExtImageCopyCaptureCursorSessionV1GetCaptureSession(
        ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest,
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
ObjectInteface::ImageCopyCapture(ImageCopyCaptureObjectInteface::ExtImageCopyCaptureManagerV1{..})=>match opcode {
ExtImageCopyCaptureManagerV1CreateSessionRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureManagerV1CreateSession(ExtImageCopyCaptureManagerV1CreateSessionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureManagerV1CreatePointerCursorSession(ExtImageCopyCaptureManagerV1CreatePointerCursorSessionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureManagerV1DestroyRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureManagerV1Destroy(ExtImageCopyCaptureManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ImageCopyCapture(ImageCopyCaptureObjectInteface::ExtImageCopyCaptureSessionV1{..})=>match opcode {
ExtImageCopyCaptureSessionV1CreateFrameRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureSessionV1CreateFrame(ExtImageCopyCaptureSessionV1CreateFrameRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureSessionV1DestroyRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureSessionV1Destroy(ExtImageCopyCaptureSessionV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ImageCopyCapture(ImageCopyCaptureObjectInteface::ExtImageCopyCaptureFrameV1{..})=>match opcode {
ExtImageCopyCaptureFrameV1DestroyRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureFrameV1Destroy(ExtImageCopyCaptureFrameV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureFrameV1AttachBufferRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureFrameV1AttachBuffer(ExtImageCopyCaptureFrameV1AttachBufferRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureFrameV1DamageBufferRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureFrameV1DamageBuffer(ExtImageCopyCaptureFrameV1DamageBufferRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureFrameV1CaptureRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureFrameV1Capture(ExtImageCopyCaptureFrameV1CaptureRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ImageCopyCapture(ImageCopyCaptureObjectInteface::ExtImageCopyCaptureCursorSessionV1{..})=>match opcode {
ExtImageCopyCaptureCursorSessionV1DestroyRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureCursorSessionV1Destroy(ExtImageCopyCaptureCursorSessionV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest::OPCODE=>Ok(Self::ExtImageCopyCaptureCursorSessionV1GetCaptureSession(ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtImageCopyCaptureManagerV1CreateSession(message) => message.object_id(),
            Self::ExtImageCopyCaptureManagerV1CreatePointerCursorSession(message) => {
                message.object_id()
            }
            Self::ExtImageCopyCaptureManagerV1Destroy(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1CreateFrame(message) => message.object_id(),
            Self::ExtImageCopyCaptureSessionV1Destroy(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Destroy(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1AttachBuffer(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1DamageBuffer(message) => message.object_id(),
            Self::ExtImageCopyCaptureFrameV1Capture(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1Destroy(message) => message.object_id(),
            Self::ExtImageCopyCaptureCursorSessionV1GetCaptureSession(message) => {
                message.object_id()
            }
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtImageCopyCaptureManagerV1CreateSession(message) => message.opcode(),
            Self::ExtImageCopyCaptureManagerV1CreatePointerCursorSession(message) => {
                message.opcode()
            }
            Self::ExtImageCopyCaptureManagerV1Destroy(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1CreateFrame(message) => message.opcode(),
            Self::ExtImageCopyCaptureSessionV1Destroy(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Destroy(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1AttachBuffer(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1DamageBuffer(message) => message.opcode(),
            Self::ExtImageCopyCaptureFrameV1Capture(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1Destroy(message) => message.opcode(),
            Self::ExtImageCopyCaptureCursorSessionV1GetCaptureSession(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtImageCopyCaptureManagerV1CreateSession(message) => message.payload_size(),
            Self::ExtImageCopyCaptureManagerV1CreatePointerCursorSession(message) => {
                message.payload_size()
            }
            Self::ExtImageCopyCaptureManagerV1Destroy(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1CreateFrame(message) => message.payload_size(),
            Self::ExtImageCopyCaptureSessionV1Destroy(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Destroy(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1AttachBuffer(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1DamageBuffer(message) => message.payload_size(),
            Self::ExtImageCopyCaptureFrameV1Capture(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1Destroy(message) => message.payload_size(),
            Self::ExtImageCopyCaptureCursorSessionV1GetCaptureSession(message) => {
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
            Self::ExtImageCopyCaptureManagerV1CreateSession(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureManagerV1CreatePointerCursorSession(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1CreateFrame(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureSessionV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1Destroy(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureFrameV1AttachBuffer(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1DamageBuffer(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureFrameV1Capture(message) => message.write(writer, state).await,
            Self::ExtImageCopyCaptureCursorSessionV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtImageCopyCaptureCursorSessionV1GetCaptureSession(message) => {
                message.write(writer, state).await
            }
        }
    }
}
