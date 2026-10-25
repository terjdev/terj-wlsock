use super::{
    WpFifoManagerV1DestroyRequest, WpFifoManagerV1GetFifoRequest, WpFifoV1DestroyRequest,
    WpFifoV1SetBarrierRequest, WpFifoV1WaitBarrierRequest,
};
use crate::protocol::ext::fifo::ObjectInteface as FifoObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Fifo requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpFifoManagerV1DestroyRequest].
    WpFifoManagerV1Destroy(WpFifoManagerV1DestroyRequest),
    /// Message [WpFifoManagerV1GetFifoRequest].
    WpFifoManagerV1GetFifo(WpFifoManagerV1GetFifoRequest),
    /// Message [WpFifoV1SetBarrierRequest].
    WpFifoV1SetBarrier(WpFifoV1SetBarrierRequest),
    /// Message [WpFifoV1WaitBarrierRequest].
    WpFifoV1WaitBarrier(WpFifoV1WaitBarrierRequest),
    /// Message [WpFifoV1DestroyRequest].
    WpFifoV1Destroy(WpFifoV1DestroyRequest),
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
            ObjectInteface::Fifo(FifoObjectInteface::WpFifoManagerV1 { .. }) => match opcode {
                WpFifoManagerV1DestroyRequest::OPCODE => Ok(Self::WpFifoManagerV1Destroy(
                    WpFifoManagerV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpFifoManagerV1GetFifoRequest::OPCODE => Ok(Self::WpFifoManagerV1GetFifo(
                    WpFifoManagerV1GetFifoRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::Fifo(FifoObjectInteface::WpFifoV1 { .. }) => match opcode {
                WpFifoV1SetBarrierRequest::OPCODE => Ok(Self::WpFifoV1SetBarrier(
                    WpFifoV1SetBarrierRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpFifoV1WaitBarrierRequest::OPCODE => Ok(Self::WpFifoV1WaitBarrier(
                    WpFifoV1WaitBarrierRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                WpFifoV1DestroyRequest::OPCODE => Ok(Self::WpFifoV1Destroy(
                    WpFifoV1DestroyRequest::read_with_interface(
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
            Self::WpFifoManagerV1Destroy(message) => message.object_id(),
            Self::WpFifoManagerV1GetFifo(message) => message.object_id(),
            Self::WpFifoV1SetBarrier(message) => message.object_id(),
            Self::WpFifoV1WaitBarrier(message) => message.object_id(),
            Self::WpFifoV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpFifoManagerV1Destroy(message) => message.opcode(),
            Self::WpFifoManagerV1GetFifo(message) => message.opcode(),
            Self::WpFifoV1SetBarrier(message) => message.opcode(),
            Self::WpFifoV1WaitBarrier(message) => message.opcode(),
            Self::WpFifoV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpFifoManagerV1Destroy(message) => message.payload_size(),
            Self::WpFifoManagerV1GetFifo(message) => message.payload_size(),
            Self::WpFifoV1SetBarrier(message) => message.payload_size(),
            Self::WpFifoV1WaitBarrier(message) => message.payload_size(),
            Self::WpFifoV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpFifoManagerV1Destroy(message) => message.write(writer, state).await,
            Self::WpFifoManagerV1GetFifo(message) => message.write(writer, state).await,
            Self::WpFifoV1SetBarrier(message) => message.write(writer, state).await,
            Self::WpFifoV1WaitBarrier(message) => message.write(writer, state).await,
            Self::WpFifoV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
