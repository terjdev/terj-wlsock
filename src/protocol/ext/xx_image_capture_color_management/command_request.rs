use super::ObjectInteface as XxImageCaptureColorManagementObjectInteface;
use super::{
    XxImageCaptureColorManagerV1DestroyRequest,
    XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest,
    XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest,
    XxImageCaptureSourceColorsV1DestroyRequest, XxImageCaptureSourceColorsV1GetPreferredRequest,
};
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches image capture color management requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XxImageCaptureColorManagerV1DestroyRequest].
    XxImageCaptureColorManagerV1Destroy(XxImageCaptureColorManagerV1DestroyRequest),
    /// Message [XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest].
    XxImageCaptureColorManagerV1GetCaptureSourceColors(
        XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest,
    ),
    /// Message [XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest].
    XxImageCaptureColorManagerV1SetFrameImageDescription(
        XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest,
    ),
    /// Message [XxImageCaptureSourceColorsV1DestroyRequest].
    XxImageCaptureSourceColorsV1Destroy(XxImageCaptureSourceColorsV1DestroyRequest),
    /// Message [XxImageCaptureSourceColorsV1GetPreferredRequest].
    XxImageCaptureSourceColorsV1GetPreferred(XxImageCaptureSourceColorsV1GetPreferredRequest),
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
        match interface{
ObjectInteface::XxImageCaptureColorManagement(XxImageCaptureColorManagementObjectInteface::XxImageCaptureColorManagerV1{..})=>match opcode{XxImageCaptureColorManagerV1DestroyRequest::OPCODE=>Ok(Self::XxImageCaptureColorManagerV1Destroy(XxImageCaptureColorManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest::OPCODE=>Ok(Self::XxImageCaptureColorManagerV1GetCaptureSourceColors(XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest::OPCODE=>Ok(Self::XxImageCaptureColorManagerV1SetFrameImageDescription(XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::XxImageCaptureColorManagement(XxImageCaptureColorManagementObjectInteface::XxImageCaptureSourceColorsV1{..})=>match opcode{XxImageCaptureSourceColorsV1DestroyRequest::OPCODE=>Ok(Self::XxImageCaptureSourceColorsV1Destroy(XxImageCaptureSourceColorsV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),XxImageCaptureSourceColorsV1GetPreferredRequest::OPCODE=>Ok(Self::XxImageCaptureSourceColorsV1GetPreferred(XxImageCaptureSourceColorsV1GetPreferredRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::XxImageCaptureColorManagerV1Destroy(m) => m.object_id(),
            Self::XxImageCaptureColorManagerV1GetCaptureSourceColors(m) => m.object_id(),
            Self::XxImageCaptureColorManagerV1SetFrameImageDescription(m) => m.object_id(),
            Self::XxImageCaptureSourceColorsV1Destroy(m) => m.object_id(),
            Self::XxImageCaptureSourceColorsV1GetPreferred(m) => m.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XxImageCaptureColorManagerV1Destroy(m) => m.opcode(),
            Self::XxImageCaptureColorManagerV1GetCaptureSourceColors(m) => m.opcode(),
            Self::XxImageCaptureColorManagerV1SetFrameImageDescription(m) => m.opcode(),
            Self::XxImageCaptureSourceColorsV1Destroy(m) => m.opcode(),
            Self::XxImageCaptureSourceColorsV1GetPreferred(m) => m.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XxImageCaptureColorManagerV1Destroy(m) => m.payload_size(),
            Self::XxImageCaptureColorManagerV1GetCaptureSourceColors(m) => m.payload_size(),
            Self::XxImageCaptureColorManagerV1SetFrameImageDescription(m) => m.payload_size(),
            Self::XxImageCaptureSourceColorsV1Destroy(m) => m.payload_size(),
            Self::XxImageCaptureSourceColorsV1GetPreferred(m) => m.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XxImageCaptureColorManagerV1Destroy(m) => m.write(writer, state).await,
            Self::XxImageCaptureColorManagerV1GetCaptureSourceColors(m) => {
                m.write(writer, state).await
            }
            Self::XxImageCaptureColorManagerV1SetFrameImageDescription(m) => {
                m.write(writer, state).await
            }
            Self::XxImageCaptureSourceColorsV1Destroy(m) => m.write(writer, state).await,
            Self::XxImageCaptureSourceColorsV1GetPreferred(m) => m.write(writer, state).await,
        }
    }
}
