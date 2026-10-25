use super::{
    ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest,
    ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest,
    ZwlrVirtualPointerManagerV1DestroyRequest, ZwlrVirtualPointerV1AxisDiscreteRequest,
    ZwlrVirtualPointerV1AxisRequest, ZwlrVirtualPointerV1AxisSourceRequest,
    ZwlrVirtualPointerV1AxisStopRequest, ZwlrVirtualPointerV1ButtonRequest,
    ZwlrVirtualPointerV1DestroyRequest, ZwlrVirtualPointerV1FrameRequest,
    ZwlrVirtualPointerV1MotionAbsoluteRequest, ZwlrVirtualPointerV1MotionRequest,
};
use crate::protocol::wlr::virtual_pointer::ObjectInteface as WlrVirtualPointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches WLR virtual pointer requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [ZwlrVirtualPointerV1MotionRequest].
    ZwlrVirtualPointerV1Motion(ZwlrVirtualPointerV1MotionRequest),
    /// Message [ZwlrVirtualPointerV1MotionAbsoluteRequest].
    ZwlrVirtualPointerV1MotionAbsolute(ZwlrVirtualPointerV1MotionAbsoluteRequest),
    /// Message [ZwlrVirtualPointerV1ButtonRequest].
    ZwlrVirtualPointerV1Button(ZwlrVirtualPointerV1ButtonRequest),
    /// Message [ZwlrVirtualPointerV1AxisRequest].
    ZwlrVirtualPointerV1Axis(ZwlrVirtualPointerV1AxisRequest),
    /// Message [ZwlrVirtualPointerV1FrameRequest].
    ZwlrVirtualPointerV1Frame(ZwlrVirtualPointerV1FrameRequest),
    /// Message [ZwlrVirtualPointerV1AxisSourceRequest].
    ZwlrVirtualPointerV1AxisSource(ZwlrVirtualPointerV1AxisSourceRequest),
    /// Message [ZwlrVirtualPointerV1AxisStopRequest].
    ZwlrVirtualPointerV1AxisStop(ZwlrVirtualPointerV1AxisStopRequest),
    /// Message [ZwlrVirtualPointerV1AxisDiscreteRequest].
    ZwlrVirtualPointerV1AxisDiscrete(ZwlrVirtualPointerV1AxisDiscreteRequest),
    /// Message [ZwlrVirtualPointerV1DestroyRequest].
    ZwlrVirtualPointerV1Destroy(ZwlrVirtualPointerV1DestroyRequest),
    /// Message [ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest].
    ZwlrVirtualPointerManagerV1CreateVirtualPointer(
        ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest,
    ),
    /// Message [ZwlrVirtualPointerManagerV1DestroyRequest].
    ZwlrVirtualPointerManagerV1Destroy(ZwlrVirtualPointerManagerV1DestroyRequest),
    /// Message [ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest].
    ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(
        ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest,
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
ObjectInteface::WlrVirtualPointer(WlrVirtualPointerObjectInteface::ZwlrVirtualPointerV1{..})=>match opcode {
ZwlrVirtualPointerV1MotionRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1Motion(ZwlrVirtualPointerV1MotionRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1MotionAbsoluteRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1MotionAbsolute(ZwlrVirtualPointerV1MotionAbsoluteRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1ButtonRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1Button(ZwlrVirtualPointerV1ButtonRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1AxisRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1Axis(ZwlrVirtualPointerV1AxisRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1FrameRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1Frame(ZwlrVirtualPointerV1FrameRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1AxisSourceRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1AxisSource(ZwlrVirtualPointerV1AxisSourceRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1AxisStopRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1AxisStop(ZwlrVirtualPointerV1AxisStopRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1AxisDiscreteRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1AxisDiscrete(ZwlrVirtualPointerV1AxisDiscreteRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerV1DestroyRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerV1Destroy(ZwlrVirtualPointerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
ObjectInteface::WlrVirtualPointer(WlrVirtualPointerObjectInteface::ZwlrVirtualPointerManagerV1{..})=>match opcode {
ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerManagerV1CreateVirtualPointer(ZwlrVirtualPointerManagerV1CreateVirtualPointerRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerManagerV1DestroyRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerManagerV1Destroy(ZwlrVirtualPointerManagerV1DestroyRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest::OPCODE=>Ok(Self::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutputRequest::read_with_interface(object_id,opcode,interface,reader,state).await?)),
_=>Err(FdReaderError::InvalidProtocol),},
_=>Err(FdReaderError::InvalidProtocol),}
    }
}
impl CommandWriter for CommandRequest {
    fn object_id(&self) -> u32 {
        match self {
            Self::ZwlrVirtualPointerV1Motion(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1MotionAbsolute(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1Button(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1Axis(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1Frame(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1AxisSource(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1AxisStop(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1AxisDiscrete(message) => message.object_id(),
            Self::ZwlrVirtualPointerV1Destroy(message) => message.object_id(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointer(message) => message.object_id(),
            Self::ZwlrVirtualPointerManagerV1Destroy(message) => message.object_id(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(message) => {
                message.object_id()
            }
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::ZwlrVirtualPointerV1Motion(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1MotionAbsolute(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1Button(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1Axis(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1Frame(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1AxisSource(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1AxisStop(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1AxisDiscrete(message) => message.opcode(),
            Self::ZwlrVirtualPointerV1Destroy(message) => message.opcode(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointer(message) => message.opcode(),
            Self::ZwlrVirtualPointerManagerV1Destroy(message) => message.opcode(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(message) => {
                message.opcode()
            }
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::ZwlrVirtualPointerV1Motion(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1MotionAbsolute(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1Button(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1Axis(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1Frame(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1AxisSource(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1AxisStop(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1AxisDiscrete(message) => message.payload_size(),
            Self::ZwlrVirtualPointerV1Destroy(message) => message.payload_size(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointer(message) => {
                message.payload_size()
            }
            Self::ZwlrVirtualPointerManagerV1Destroy(message) => message.payload_size(),
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(message) => {
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
            Self::ZwlrVirtualPointerV1Motion(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1MotionAbsolute(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1Button(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1Axis(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1Frame(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1AxisSource(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1AxisStop(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1AxisDiscrete(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointer(message) => {
                message.write(writer, state).await
            }
            Self::ZwlrVirtualPointerManagerV1Destroy(message) => message.write(writer, state).await,
            Self::ZwlrVirtualPointerManagerV1CreateVirtualPointerWithOutput(message) => {
                message.write(writer, state).await
            }
        }
    }
}
