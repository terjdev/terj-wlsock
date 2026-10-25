use super::{
    WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest,
    WpSinglePixelBufferManagerV1DestroyRequest,
};
use crate::protocol::ext::single_pixel_buffer::ObjectInteface as SinglePixelBufferObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Single pixel buffer requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpSinglePixelBufferManagerV1DestroyRequest].
    WpSinglePixelBufferManagerV1Destroy(WpSinglePixelBufferManagerV1DestroyRequest),
    /// Message [WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest].
    WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(
        WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest,
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
ObjectInteface::SinglePixelBuffer(SinglePixelBufferObjectInteface::WpSinglePixelBufferManagerV1{..})=>match opcode {
WpSinglePixelBufferManagerV1DestroyRequest::OPCODE=>Ok(Self::WpSinglePixelBufferManagerV1Destroy(WpSinglePixelBufferManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest::OPCODE=>Ok(Self::WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(WpSinglePixelBufferManagerV1CreateU32RgbaBufferRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpSinglePixelBufferManagerV1Destroy(message) => message.object_id(),
            Self::WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpSinglePixelBufferManagerV1Destroy(message) => message.opcode(),
            Self::WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpSinglePixelBufferManagerV1Destroy(message) => message.payload_size(),
            Self::WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(message) => {
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
            Self::WpSinglePixelBufferManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::WpSinglePixelBufferManagerV1CreateU32RgbaBuffer(message) => {
                message.write(writer, state).await
            }
        }
    }
}
