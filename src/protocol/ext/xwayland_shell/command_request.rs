use super::{
    XwaylandShellV1DestroyRequest, XwaylandShellV1GetXwaylandSurfaceRequest,
    XwaylandSurfaceV1DestroyRequest, XwaylandSurfaceV1SetSerialRequest,
};
use crate::protocol::ext::xwayland_shell::ObjectInteface as XwaylandShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, InterfaceReader, ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};
/// Dispatches Xwayland shell requests.
#[derive(Debug)]
pub enum CommandRequest {
    /// Message [XwaylandShellV1DestroyRequest].
    XwaylandShellV1Destroy(XwaylandShellV1DestroyRequest),
    /// Message [XwaylandShellV1GetXwaylandSurfaceRequest].
    XwaylandShellV1GetXwaylandSurface(XwaylandShellV1GetXwaylandSurfaceRequest),
    /// Message [XwaylandSurfaceV1SetSerialRequest].
    XwaylandSurfaceV1SetSerial(XwaylandSurfaceV1SetSerialRequest),
    /// Message [XwaylandSurfaceV1DestroyRequest].
    XwaylandSurfaceV1Destroy(XwaylandSurfaceV1DestroyRequest),
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
            ObjectInteface::XwaylandShell(XwaylandShellObjectInteface::XwaylandShellV1 {
                ..
            }) => match opcode {
                XwaylandShellV1DestroyRequest::OPCODE => Ok(Self::XwaylandShellV1Destroy(
                    XwaylandShellV1DestroyRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XwaylandShellV1GetXwaylandSurfaceRequest::OPCODE => {
                    Ok(Self::XwaylandShellV1GetXwaylandSurface(
                        XwaylandShellV1GetXwaylandSurfaceRequest::read_with_interface(
                            object_id, opcode, interface, reader, state,
                        )
                        .await?,
                    ))
                }
                _ => Err(FdReaderError::InvalidProtocol),
            },
            ObjectInteface::XwaylandShell(XwaylandShellObjectInteface::XwaylandSurfaceV1 {
                ..
            }) => match opcode {
                XwaylandSurfaceV1SetSerialRequest::OPCODE => Ok(Self::XwaylandSurfaceV1SetSerial(
                    XwaylandSurfaceV1SetSerialRequest::read_with_interface(
                        object_id, opcode, interface, reader, state,
                    )
                    .await?,
                )),
                XwaylandSurfaceV1DestroyRequest::OPCODE => Ok(Self::XwaylandSurfaceV1Destroy(
                    XwaylandSurfaceV1DestroyRequest::read_with_interface(
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
            Self::XwaylandShellV1Destroy(message) => message.object_id(),
            Self::XwaylandShellV1GetXwaylandSurface(message) => message.object_id(),
            Self::XwaylandSurfaceV1SetSerial(message) => message.object_id(),
            Self::XwaylandSurfaceV1Destroy(message) => message.object_id(),
        }
    }
    fn opcode(&self) -> u16 {
        match self {
            Self::XwaylandShellV1Destroy(message) => message.opcode(),
            Self::XwaylandShellV1GetXwaylandSurface(message) => message.opcode(),
            Self::XwaylandSurfaceV1SetSerial(message) => message.opcode(),
            Self::XwaylandSurfaceV1Destroy(message) => message.opcode(),
        }
    }
    fn payload_size(&self) -> usize {
        match self {
            Self::XwaylandShellV1Destroy(message) => message.payload_size(),
            Self::XwaylandShellV1GetXwaylandSurface(message) => message.payload_size(),
            Self::XwaylandSurfaceV1SetSerial(message) => message.payload_size(),
            Self::XwaylandSurfaceV1Destroy(message) => message.payload_size(),
        }
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        match self {
            Self::XwaylandShellV1Destroy(message) => message.write(writer, state).await,
            Self::XwaylandShellV1GetXwaylandSurface(message) => message.write(writer, state).await,
            Self::XwaylandSurfaceV1SetSerial(message) => message.write(writer, state).await,
            Self::XwaylandSurfaceV1Destroy(message) => message.write(writer, state).await,
        }
    }
}
