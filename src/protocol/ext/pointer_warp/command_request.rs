use super::{WpPointerWarpV1DestroyRequest, WpPointerWarpV1WarpPointerRequest};
use crate::protocol::ext::pointer_warp::ObjectInteface as PointerWarpObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Pointer warp requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [WpPointerWarpV1DestroyRequest].
    WpPointerWarpV1Destroy(WpPointerWarpV1DestroyRequest),
    /// Message [WpPointerWarpV1WarpPointerRequest].
    WpPointerWarpV1WarpPointer(WpPointerWarpV1WarpPointerRequest),
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
            ObjectInteface::PointerWarp(PointerWarpObjectInteface::WpPointerWarpV1 { .. }) => {
                match opcode {
                    WpPointerWarpV1DestroyRequest::OPCODE => Ok(Self::WpPointerWarpV1Destroy(
                        WpPointerWarpV1DestroyRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    )),
                    WpPointerWarpV1WarpPointerRequest::OPCODE => {
                        Ok(Self::WpPointerWarpV1WarpPointer(
                            WpPointerWarpV1WarpPointerRequest::read_with_interface(
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
            Self::WpPointerWarpV1Destroy(message) => message.object_id(),
            Self::WpPointerWarpV1WarpPointer(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::WpPointerWarpV1Destroy(message) => message.opcode(),
            Self::WpPointerWarpV1WarpPointer(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::WpPointerWarpV1Destroy(message) => message.payload_size(),
            Self::WpPointerWarpV1WarpPointer(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::WpPointerWarpV1Destroy(message) => message.write(writer, state).await,
            Self::WpPointerWarpV1WarpPointer(message) => message.write(writer, state).await,
        }
    }
}
