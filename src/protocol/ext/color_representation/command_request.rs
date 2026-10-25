use super::{
    WpColorRepresentationManagerV1DestroyRequest, WpColorRepresentationManagerV1GetSurfaceRequest,
    WpColorRepresentationSurfaceV1DestroyRequest,
    WpColorRepresentationSurfaceV1SetAlphaModeRequest,
    WpColorRepresentationSurfaceV1SetChromaLocationRequest,
    WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest,
};
use crate::protocol::ext::color_representation::ObjectInteface as ColorRepresentationObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Color representation requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpColorRepresentationManagerV1DestroyRequest].
    WpColorRepresentationManagerV1Destroy(WpColorRepresentationManagerV1DestroyRequest),
    /// Message [WpColorRepresentationManagerV1GetSurfaceRequest].
    WpColorRepresentationManagerV1GetSurface(WpColorRepresentationManagerV1GetSurfaceRequest),
    /// Message [WpColorRepresentationSurfaceV1DestroyRequest].
    WpColorRepresentationSurfaceV1Destroy(WpColorRepresentationSurfaceV1DestroyRequest),
    /// Message [WpColorRepresentationSurfaceV1SetAlphaModeRequest].
    WpColorRepresentationSurfaceV1SetAlphaMode(WpColorRepresentationSurfaceV1SetAlphaModeRequest),
    /// Message [WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest].
    WpColorRepresentationSurfaceV1SetCoefficientsAndRange(
        WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest,
    ),
    /// Message [WpColorRepresentationSurfaceV1SetChromaLocationRequest].
    WpColorRepresentationSurfaceV1SetChromaLocation(
        WpColorRepresentationSurfaceV1SetChromaLocationRequest,
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
ObjectInteface::ColorRepresentation(ColorRepresentationObjectInteface::WpColorRepresentationManagerV1{..})=>match opcode {
WpColorRepresentationManagerV1DestroyRequest::OPCODE=>Ok(Self::WpColorRepresentationManagerV1Destroy(WpColorRepresentationManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorRepresentationManagerV1GetSurfaceRequest::OPCODE=>Ok(Self::WpColorRepresentationManagerV1GetSurface(WpColorRepresentationManagerV1GetSurfaceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::ColorRepresentation(ColorRepresentationObjectInteface::WpColorRepresentationSurfaceV1{..})=>match opcode {
WpColorRepresentationSurfaceV1DestroyRequest::OPCODE=>Ok(Self::WpColorRepresentationSurfaceV1Destroy(WpColorRepresentationSurfaceV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorRepresentationSurfaceV1SetAlphaModeRequest::OPCODE=>Ok(Self::WpColorRepresentationSurfaceV1SetAlphaMode(WpColorRepresentationSurfaceV1SetAlphaModeRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest::OPCODE=>Ok(Self::WpColorRepresentationSurfaceV1SetCoefficientsAndRange(WpColorRepresentationSurfaceV1SetCoefficientsAndRangeRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
WpColorRepresentationSurfaceV1SetChromaLocationRequest::OPCODE=>Ok(Self::WpColorRepresentationSurfaceV1SetChromaLocation(WpColorRepresentationSurfaceV1SetChromaLocationRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::WpColorRepresentationManagerV1Destroy(message) => message.object_id(),
            Self::WpColorRepresentationManagerV1GetSurface(message) => message.object_id(),
            Self::WpColorRepresentationSurfaceV1Destroy(message) => message.object_id(),
            Self::WpColorRepresentationSurfaceV1SetAlphaMode(message) => message.object_id(),
            Self::WpColorRepresentationSurfaceV1SetCoefficientsAndRange(message) => {
                message.object_id()
            }
            Self::WpColorRepresentationSurfaceV1SetChromaLocation(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpColorRepresentationManagerV1Destroy(message) => message.opcode(),
            Self::WpColorRepresentationManagerV1GetSurface(message) => message.opcode(),
            Self::WpColorRepresentationSurfaceV1Destroy(message) => message.opcode(),
            Self::WpColorRepresentationSurfaceV1SetAlphaMode(message) => message.opcode(),
            Self::WpColorRepresentationSurfaceV1SetCoefficientsAndRange(message) => {
                message.opcode()
            }
            Self::WpColorRepresentationSurfaceV1SetChromaLocation(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpColorRepresentationManagerV1Destroy(message) => message.payload_size(),
            Self::WpColorRepresentationManagerV1GetSurface(message) => message.payload_size(),
            Self::WpColorRepresentationSurfaceV1Destroy(message) => message.payload_size(),
            Self::WpColorRepresentationSurfaceV1SetAlphaMode(message) => message.payload_size(),
            Self::WpColorRepresentationSurfaceV1SetCoefficientsAndRange(message) => {
                message.payload_size()
            }
            Self::WpColorRepresentationSurfaceV1SetChromaLocation(message) => {
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
            Self::WpColorRepresentationManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::WpColorRepresentationManagerV1GetSurface(message) => {
                message.write(writer, state).await
            }
            Self::WpColorRepresentationSurfaceV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::WpColorRepresentationSurfaceV1SetAlphaMode(message) => {
                message.write(writer, state).await
            }
            Self::WpColorRepresentationSurfaceV1SetCoefficientsAndRange(message) => {
                message.write(writer, state).await
            }
            Self::WpColorRepresentationSurfaceV1SetChromaLocation(message) => {
                message.write(writer, state).await
            }
        }
    }
}
