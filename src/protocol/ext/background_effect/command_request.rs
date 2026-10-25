use super::{
    ExtBackgroundEffectManagerV1DestroyRequest,
    ExtBackgroundEffectManagerV1GetBackgroundEffectRequest,
    ExtBackgroundEffectSurfaceV1DestroyRequest, ExtBackgroundEffectSurfaceV1SetBlurRegionRequest,
};
use crate::protocol::ext::background_effect::ObjectInteface as BackgroundEffectObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Background effect requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ExtBackgroundEffectManagerV1DestroyRequest].
    ExtBackgroundEffectManagerV1Destroy(ExtBackgroundEffectManagerV1DestroyRequest),
    /// Message [ExtBackgroundEffectManagerV1GetBackgroundEffectRequest].
    ExtBackgroundEffectManagerV1GetBackgroundEffect(
        ExtBackgroundEffectManagerV1GetBackgroundEffectRequest,
    ),
    /// Message [ExtBackgroundEffectSurfaceV1DestroyRequest].
    ExtBackgroundEffectSurfaceV1Destroy(ExtBackgroundEffectSurfaceV1DestroyRequest),
    /// Message [ExtBackgroundEffectSurfaceV1SetBlurRegionRequest].
    ExtBackgroundEffectSurfaceV1SetBlurRegion(ExtBackgroundEffectSurfaceV1SetBlurRegionRequest),
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
ObjectInteface::BackgroundEffect(BackgroundEffectObjectInteface::ExtBackgroundEffectManagerV1{..})=>match opcode {
ExtBackgroundEffectManagerV1DestroyRequest::OPCODE=>Ok(Self::ExtBackgroundEffectManagerV1Destroy(ExtBackgroundEffectManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtBackgroundEffectManagerV1GetBackgroundEffectRequest::OPCODE=>Ok(Self::ExtBackgroundEffectManagerV1GetBackgroundEffect(ExtBackgroundEffectManagerV1GetBackgroundEffectRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::BackgroundEffect(BackgroundEffectObjectInteface::ExtBackgroundEffectSurfaceV1{..})=>match opcode {
ExtBackgroundEffectSurfaceV1DestroyRequest::OPCODE=>Ok(Self::ExtBackgroundEffectSurfaceV1Destroy(ExtBackgroundEffectSurfaceV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ExtBackgroundEffectSurfaceV1SetBlurRegionRequest::OPCODE=>Ok(Self::ExtBackgroundEffectSurfaceV1SetBlurRegion(ExtBackgroundEffectSurfaceV1SetBlurRegionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ExtBackgroundEffectManagerV1Destroy(message) => message.object_id(),
            Self::ExtBackgroundEffectManagerV1GetBackgroundEffect(message) => message.object_id(),
            Self::ExtBackgroundEffectSurfaceV1Destroy(message) => message.object_id(),
            Self::ExtBackgroundEffectSurfaceV1SetBlurRegion(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ExtBackgroundEffectManagerV1Destroy(message) => message.opcode(),
            Self::ExtBackgroundEffectManagerV1GetBackgroundEffect(message) => message.opcode(),
            Self::ExtBackgroundEffectSurfaceV1Destroy(message) => message.opcode(),
            Self::ExtBackgroundEffectSurfaceV1SetBlurRegion(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ExtBackgroundEffectManagerV1Destroy(message) => message.payload_size(),
            Self::ExtBackgroundEffectManagerV1GetBackgroundEffect(message) => {
                message.payload_size()
            }
            Self::ExtBackgroundEffectSurfaceV1Destroy(message) => message.payload_size(),
            Self::ExtBackgroundEffectSurfaceV1SetBlurRegion(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::ExtBackgroundEffectManagerV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtBackgroundEffectManagerV1GetBackgroundEffect(message) => {
                message.write(writer, state).await
            }
            Self::ExtBackgroundEffectSurfaceV1Destroy(message) => {
                message.write(writer, state).await
            }
            Self::ExtBackgroundEffectSurfaceV1SetBlurRegion(message) => {
                message.write(writer, state).await
            }
        }
    }
}
